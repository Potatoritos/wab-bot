use crate::Command;
use std::collections::{hash_map::Entry, HashMap, HashSet};
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
use twilight_util::builder::command::CommandBuilder;

pub struct CommandHandler {
    commands: HashMap<String, Command>,
    command_adjacency: HashMap<String, HashSet<String>>,
}
impl CommandHandler {
    pub fn new(commands: Vec<Command>) -> Self {
        let mut command_map = HashMap::new();
        for command in commands {
            let name = String::from(command.name());
            match command_map.entry(name) {
                Entry::Occupied(o) => panic!("Duplicate command name: '{}'", o.key()),
                Entry::Vacant(v) => v.insert(command),
            };
        }

        let mut command_adjacency = HashMap::new();

        for name in command_map.keys() {
            let mut indices: Vec<usize> = name
                .chars()
                .enumerate()
                .filter(|(_, c)| c == &' ')
                .map(|(i, _)| i)
                .collect();
            indices.push(name.len());

            for i in 0..indices.len() {
                let set = command_adjacency
                    .entry(String::from(&name[..indices[i]]))
                    .or_insert_with(|| HashSet::new());

                if i != indices.len() - 1 {
                    set.insert(String::from(&name[..indices[i + 1]]));
                }
            }
        }
        
        Self {
            commands: command_map,
            command_adjacency,
        }
    }
    pub fn create_application_commands(&self) -> Vec<ApplicationCommand> {
        let mut vec = Vec::new();
        for (root, depth_2_cmds) in self
            .command_adjacency
            .iter()
            .filter(|(name, _)| name.find(' ').is_none())
        {
            let mut cmd = ApplicationCommand {
                application_id: None,
                default_member_permissions: None,
                dm_permission: None,
                description: String::from(""),
                description_localizations: None,
                guild_id: None,
                id: None,
                kind: CommandType::ChatInput,
                name: String::from(root),
                name_localizations: None,
                nsfw: None,
                options: Vec::new(),
                version: Id::new(1),
            };
            if let Some(c) = self.commands.get(root) {
                cmd.description = String::from(c.description());
                cmd.options = c.create_twilight_command_options();
                vec.push(cmd);
                continue;
            }

            for depth_2_name in depth_2_cmds {
                let mut sub = CommandOption {
                    autocomplete: None,
                    channel_types: None,
                    choices: None,
                    description: String::from(""),
                    description_localizations: None,
                    kind: CommandOptionType::SubCommandGroup,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name: String::from(&depth_2_name[(depth_2_name.find(' ').unwrap() + 1)..]),
                    name_localizations: None,
                    options: None,
                    required: None,
                };
                if let Some(c) = self.commands.get(depth_2_name) {
                    sub.description = String::from(c.description());
                    sub.kind = CommandOptionType::SubCommand;
                    sub.options = Some(c.create_twilight_command_options());
                    cmd.options.push(sub);
                    continue;
                }

                let mut options = Vec::new();
                for depth_3_name in self.command_adjacency.get(depth_2_name).unwrap() {
                    let c = self.commands.get(depth_3_name).unwrap();
                    options.push(CommandOption {
                        autocomplete: None,
                        channel_types: None,
                        choices: None,
                        description: String::from(c.description()),
                        description_localizations: None,
                        kind: CommandOptionType::SubCommand,
                        max_length: None,
                        max_value: None,
                        min_length: None,
                        min_value: None,
                        name: String::from(&depth_3_name[(depth_3_name.rfind(' ').unwrap() + 1)..]),
                        name_localizations: None,
                        options: Some(c.create_twilight_command_options()),
                        required: None,
                    });
                }
                sub.options = Some(options);
                cmd.options.push(sub);
            }
            vec.push(cmd);
        }
        vec
    }
}
