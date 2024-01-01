use crate::{Command, CommandHandler, Group};

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_http::Client;
use twilight_model::application::command::{
    Command as ApplicationCommand, CommandOption, CommandOptionChoice, CommandOptionChoiceValue,
    CommandOptionType, CommandOptionValue, CommandType,
};
use twilight_model::application::interaction::{
    application_command::{CommandData, CommandOptionValue as InteractionCommandOptionValue},
    Interaction, InteractionData, InteractionType,
};
use twilight_model::id::marker;
use twilight_model::id::Id;
use typemap::TypeMap;

pub struct Bot<'a> {
    groups: Vec<&'a Group>,
    command_handler: CommandHandler,
    state: Arc<RwLock<TypeMap>>,
    registered_interactions: bool,
    initialized_commands: bool,
}

impl<'a> Bot<'a> {
    fn new(groups: Vec<&'a Group>) -> Self {
        let mut commands = Vec::new();

        for group in &groups {
            for command in (group.build_commands)() {
                commands.push(command);
            }
        }

        let state = Arc::new(RwLock::new(TypeMap::new()));

        Bot {
            groups,
            command_handler: CommandHandler::new(commands),
            state,
            registered_interactions: false,
            initialized_commands: false,
        }
    }
    pub async fn register_interactions(&mut self, app_id: String) {
        self.registered_interactions = true;
        
        let application_commands = self.command_handler.create_application_commands();
        
        println!("{:#?}", application_commands);
    }
    pub async fn run(&mut self, token: String) {
        self.init_groups().await;

        let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES | Intents::MESSAGE_CONTENT;
        let mut shard = Shard::new(ShardId::ONE, token.clone(), intents);
        let client = Arc::new(Client::new(token));
        let resource_types = ResourceType::CHANNEL
            | ResourceType::EMOJI
            | ResourceType::GUILD
            | ResourceType::MEMBER
            | ResourceType::MESSAGE
            | ResourceType::PRESENCE
            | ResourceType::REACTION
            | ResourceType::ROLE
            | ResourceType::USER_CURRENT
            | ResourceType::USER
            | ResourceType::VOICE_STATE
            | ResourceType::STICKER;
        let cache = InMemoryCache::builder()
            .resource_types(resource_types)
            .build();

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

            self.handle_event(event, &client).await;
        }
    }
    async fn handle_event(&self, event: Event, client: &Client) {
        match event {
            Event::InteractionCreate(ic) => {
                self.handle_interaction(client, &ic.0).await;
            }
            _ => {}
        }
    }
    async fn handle_interaction(&self, client: &Client, interaction: &Interaction) {
        match interaction.kind {
            InteractionType::ApplicationCommand => {
                if let Some(InteractionData::ApplicationCommand(data)) = &interaction.data {
                    self.handle_application_command(&client, &interaction, &data)
                        .await;
                }
            }
            _ => {}
        }
    }
    async fn handle_application_command(
        &self,
        client: &Client,
        interaction: &Interaction,
        data: &Box<CommandData>,
    ) {
        let name = String::from(&data.name);
        let options = &data.options;
        // while options.len() == 1 {
        // if let InteractionCommandOptionValue::SubCommand(option) = &options[0].value {}
        // }
    }
    async fn init_groups(&mut self) {
        for group in &self.groups {
            if let Some(init_fn) = group.init {
                init_fn(self.state.clone()).await;
            }
        }
        self.initialized_commands = true;
    }
    pub fn builder() -> BotBuilder<'a> {
        BotBuilder::new()
    }
}

#[derive(Default)]
pub struct BotBuilder<'a> {
    groups: Vec<&'a Group>,
}

impl<'a> BotBuilder<'a> {
    fn new() -> Self {
        Self::default()
    }
    pub fn group(mut self, group: &'a Group) -> Self {
        self.groups.push(group);
        self
    }
    pub fn groups(mut self, groups: Vec<&'a Group>) -> Self {
        self.groups = groups;
        self
    }
    pub fn build(self) -> Bot<'a> {
        Bot::new(self.groups)
    }
}
