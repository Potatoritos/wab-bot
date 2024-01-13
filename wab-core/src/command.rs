use std::{collections::HashMap, future::Future, pin::Pin};

use twilight_model::application::command::CommandOption;

use super::argument::Argument;
use super::context::CommandContext;
use super::parameter::Parameter;

pub type CommandResult = Result<(), ()>;

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
pub type CommandFunction = fn(CommandContext, HashMap<String, Argument>) -> BoxedFuture<CommandResult>;

#[derive(Debug)]
pub struct Command {
    name: String,
    description: String,
    parameters: Vec<Parameter>,
    function: CommandFunction,
}
impl Command {
    pub fn run(&self, ctx: CommandContext, args: HashMap<String, Argument>) -> BoxedFuture<CommandResult> {
        (self.function)(ctx, args)
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn description(&self) -> &str {
        self.description.as_str()
    }
    pub fn parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }
    pub fn create_twilight_command_options(&self) -> Vec<CommandOption> {
        self.parameters
            .iter()
            .map(|p| CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: p.create_twilight_choices(),
                description: String::from(p.description()),
                description_localizations: None,
                kind: p.kind().create_twilight_option_type(),
                max_length: p.create_twilight_max_length(),
                min_length: p.create_twilight_min_length(),
                max_value: p.create_twilight_max_value(),
                min_value: p.create_twilight_min_value(),
                name: String::from(p.name()),
                name_localizations: None,
                options: None,
                required: Some(p.required().clone()),
            })
            .collect()
    }
    pub fn builder() -> CommandBuilder {
        CommandBuilder::new()
    }
}
#[derive(Default)]
pub struct CommandBuilder {
    name: String,
    category: String,
    description: String,
    parameters: Vec<Parameter>,
    function: Option<CommandFunction>,
}
impl CommandBuilder {
    fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = category.into();
        self
    }
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }
    pub fn parameter(mut self, option: Parameter) -> Self {
        self.parameters.push(option);
        self
    }
    pub fn parameters(mut self, options: Vec<Parameter>) -> Self {
        self.parameters = options;
        self
    }
    pub fn function(mut self, function: CommandFunction) -> Self {
        self.function = Some(function);
        self
    }
    pub fn build(self) -> Command {
        assert!(!self.name.is_empty() && self.name.len() <= 32);
        assert!(self.name.chars().filter(|c| c == &' ').count() <= 2);
        assert!(!self.description.is_empty() && self.description.len() <= 100);
        Command {
            name: self.name,
            description: self.description,
            parameters: self.parameters,
            function: self.function.unwrap(),
        }
    }
}
