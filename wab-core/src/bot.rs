use crate::{Argument, Command, CommandHandler, Context, Group};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::{client::InteractionClient, Client};
use twilight_model::application::interaction::{
    application_command::{CommandData, CommandOptionValue},
    Interaction, InteractionData, InteractionType,
};
use twilight_model::id::marker;
use twilight_model::id::Id;
use typemap::{ShareMap, TypeMap};

pub struct EventDispatchContext {
    state: Arc<RwLock<ShareMap>>,
    client: Arc<Client>,
    cache: Arc<InMemoryCache>,
    commands: Arc<CommandHandler>,
    event: Event,
}

pub struct Bot {
    state: Arc<RwLock<ShareMap>>,
    commands: Arc<CommandHandler>,
}
impl Bot {
    fn new<'a>(groups: &[&'a Group]) -> Self {
        let mut state: ShareMap = TypeMap::custom();
        let mut commands = Vec::new();

        for group in groups.iter() {
            for command in (group.build_commands)() {
                commands.push(command);
            }
            if let Some(init) = group.init {
                init(&mut state);
            }
        }

        Bot {
            state: Arc::new(RwLock::new(state)),
            commands: Arc::new(CommandHandler::new(commands)),
        }
    }
    async fn register_interactions(&self, interaction_client: &InteractionClient<'_>) {
        let application_commands = self.commands.create_application_commands();

        println!("{:#?}", application_commands);

        let result = interaction_client
            .set_guild_commands(Id::new(495327409487478785), &application_commands)
            .await;

        println!("result: {:#?}", result);
    }
    pub async fn run(
        &self,
        token: String,
        app_id: String,
        intents: Intents,
        resource_types: ResourceType,
    ) {
        let app_id = Id::new(app_id.parse::<u64>().unwrap());

        let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

        let client = Arc::new(Client::new(token));
        let interaction_client = client.interaction(app_id);

        self.register_interactions(&interaction_client).await;

        let cache = Arc::new(
            InMemoryCache::builder()
                .resource_types(resource_types)
                .build(),
        );

        loop {
            let event = match shard.next_event().await {
                Ok(event) => event,
                Err(source) => {
                    tracing::warn!(?source, "error receiving event");

                    if source.is_fatal() {
                        break;
                    }

                    continue;
                }
            };
            cache.update(&event);

            let ctx = EventDispatchContext {
                state: self.state.clone(),
                client: client.clone(),
                cache: cache.clone(),
                commands: self.commands.clone(),
                event: event,
            };

            tokio::spawn(handle_event(ctx));

            // self.handle_event(event, &client).await;
        }
    }
    pub fn builder<'a>() -> BotBuilder<'a> {
        BotBuilder::new()
    }
}

async fn handle_event(ctx: EventDispatchContext) -> Result<(), Box<dyn Error + Send + Sync>> {
    match &ctx.event {
        Event::InteractionCreate(ic) => {
            handle_interaction(&ctx, &ic.0).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_interaction(
    ctx: &EventDispatchContext,
    interaction: &Interaction,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match interaction.kind {
        InteractionType::ApplicationCommand => {
            if let Some(InteractionData::ApplicationCommand(data)) = &interaction.data {
                handle_application_command(ctx, interaction, data).await?;
            }
        }
        _ => {}
    }
    Ok(())
}

async fn handle_application_command(
    ctx: &EventDispatchContext,
    interaction: &Interaction,
    data: &Box<CommandData>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut name = String::from(&data.name);
    let mut options = &data.options;
    tracing::debug!("{:#?}", data);
    while options.len() == 1 {
        match &options[0].value {
            CommandOptionValue::SubCommand(o) | CommandOptionValue::SubCommandGroup(o) => {
                name.push(' ');
                name.push_str(&options[0].name);
                options = &o;
            }
            _ => break,
        }
    }

    let mut args = HashMap::new();
    for o in options {
        let arg = match &o.value {
            CommandOptionValue::Boolean(x) => Argument::Boolean(*x),
            CommandOptionValue::Integer(x) => Argument::Integer(*x),
            CommandOptionValue::Number(x) => Argument::Float(*x),
            CommandOptionValue::String(x) => Argument::String(x.to_string()),
            _ => panic!(),
        };
        args.insert(o.name.clone(), arg);
    }

    if let Some(cmd) = ctx.commands.get(&name) {
        let cmd_ctx = Context {
            state: ctx.state.clone(),
            client: ctx.client.clone(),
            cache: ctx.cache.clone(),
        };
        let result = cmd.run(cmd_ctx, args).await;
    } else {
        tracing::warn!("Could not find command: '{}'", name)
    }
    Ok(())
}

pub struct BotBuilder<'a> {
    groups: Vec<&'a Group>,
}

impl<'a> BotBuilder<'a> {
    fn new() -> Self {
        Self { groups: Vec::new() }
    }
    pub fn group(mut self, group: &'a Group) -> Self {
        self.groups.push(group);
        self
    }
    pub fn groups(mut self, groups: Vec<&'a Group>) -> Self {
        self.groups = groups;
        self
    }
    pub fn build(self) -> Bot {
        Bot::new(&self.groups)
    }
}
