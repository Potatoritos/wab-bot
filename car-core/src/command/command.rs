use std::{collections::HashMap, future::Future, pin::Pin};

use super::parameter::Parameter;
use super::context::Context;
use super::argument::Argument;

pub type CommandResult = Result<(), ()>;

pub type BoxedFuture<T> = Pin<Box<dyn Future<Output = T>>>;
pub type CommandFunction = fn(Context, HashMap<String, Argument>) -> BoxedFuture<CommandResult>;

pub struct Command {
    name: String,
    description: String,
    parameters: Vec<Parameter>,
    function: CommandFunction
}
impl Command {
    pub fn run(&self, ctx: Context, args: HashMap<String, Argument>) -> BoxedFuture<CommandResult> {
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
    pub fn builder() -> CommandBuilder {
        CommandBuilder::new()
    }
}
#[derive(Default)]
pub struct CommandBuilder {
    name: String,
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
        assert!(!self.description.is_empty() && self.description.len() <= 100);
        Command {
            name: self.name,
            description: self.description,
            parameters: self.parameters,
            function: self.function.unwrap(),
        }
    }
}
