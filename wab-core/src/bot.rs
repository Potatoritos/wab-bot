use crate::{Argument, Client, CommandContext, CommandHandler, EventFunction, Group, State};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event as EventData, EventType, Intents, Shard, ShardId};
use twilight_http::client::InteractionClient;
use twilight_model::application::interaction::{
    application_command::{CommandData, CommandOptionValue},
    Interaction, InteractionData, InteractionType,
};
use twilight_model::id::{marker::ApplicationMarker, Id};
use typemap::{ShareMap, TypeMap};

pub struct EventDispatchContext {
    state: Arc<State>,
    client: Arc<Client>,
    commands: Arc<CommandHandler>,
    events: Arc<HashMap<EventType, Vec<EventFunction>>>,
}

pub struct Bot {
    state: Arc<State>,
    commands: Arc<CommandHandler>,
    events: Arc<HashMap<EventType, Vec<EventFunction>>>,
}
impl Bot {
    fn new<'a>(groups: &[&'a Group]) -> Self {
        let mut state: ShareMap = TypeMap::custom();
        let mut commands = Vec::new();
        let mut events: HashMap<EventType, Vec<EventFunction>> = HashMap::new();

        for group in groups.iter() {
            for command in (group.build_commands)() {
                commands.push(command);
            }
            for event in (group.build_events)() {
                events
                    .entry(event.kind)
                    .or_insert(Vec::new())
                    .push(event.function);
            }
            if let Some(init) = group.init {
                init(&mut state);
            }
        }

        Bot {
            state: Arc::new(State {
                storage: RwLock::new(state),
            }),
            commands: Arc::new(CommandHandler::new(commands)),
            events: Arc::new(events),
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
        let application_id = Id::new(app_id.parse::<u64>().unwrap());

        let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);

        let http = twilight_http::Client::new(token);
        let interaction_client = http.interaction(application_id);

        self.register_interactions(&interaction_client).await;

        let cache = InMemoryCache::builder()
            .resource_types(resource_types)
            .build();

        let client = Arc::new(Client {
            http,
            cache,
            application_id,
        });

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
            client.cache.update(&event);

            let ctx = EventDispatchContext {
                state: self.state.clone(),
                client: client.clone(),
                commands: self.commands.clone(),
                events: self.events.clone(),
            };

            tokio::spawn(handle_event(ctx, event));
        }
    }
    pub fn builder<'a>() -> BotBuilder<'a> {
        BotBuilder::new()
    }
}

async fn handle_event(
    ctx: EventDispatchContext,
    event: EventData,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        EventData::InteractionCreate(ic) => {
            handle_interaction(&ctx, ic.0).await?;
        }
        _ => {
            if let Some(event_fns) = ctx.events.get(&event.kind()) {
                let event = Arc::new(event);
                for event_fn in event_fns {
                    tokio::spawn(event_fn(event.clone()));
                }
            }
        }
    }
    Ok(())
}

async fn handle_interaction(
    ctx: &EventDispatchContext,
    interaction: Interaction,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match interaction.kind {
        InteractionType::ApplicationCommand => {
            handle_application_command(ctx, interaction).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_application_command(
    ctx: &EventDispatchContext,
    interaction: Interaction,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = if let Some(InteractionData::ApplicationCommand(data)) = &interaction.data {
        data
    } else {
        panic!();
    };

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
        args.insert(o.name.clone(), Argument::try_from(&o.value).unwrap());
    }

    if let Some(cmd) = ctx.commands.get(&name) {
        let cmd_ctx = CommandContext {
            state: ctx.state.clone(),
            client: ctx.client.clone(),
            interaction,
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
