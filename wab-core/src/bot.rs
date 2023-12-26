use super::{Command, Group};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use typemap::TypeMap;

pub struct Bot {
    groups: Vec<Group>,
    commands: HashMap<String, Command>,
    state: Arc<RwLock<TypeMap>>,
    posted_commands: bool,
    initialized_commands: bool,
}

impl Bot {
    fn new(groups: Vec<Group>) -> Self {
        let mut commands = HashMap::new();

        for group in &groups {
            for command in (group.build_commands)() {
                let name = String::from(command.name());
                if commands.contains_key(&name) {
                    panic!("Duplicate command name: '{}'", &name);
                }
                commands.insert(name, command);
            }
        }

        let state = Arc::new(RwLock::new(TypeMap::new()));

        Bot {
            groups,
            commands,
            state,
            posted_commands: false,
            initialized_commands: false,
        }
    }
    pub async fn post_commands(&mut self) {
        self.posted_commands = true;
    }
    pub async fn start(&mut self) {
        self.init_commands().await;
    }
    async fn init_commands(&mut self) {
        for group in &self.groups {
            if let Some(init_fn) = group.init {
                init_fn(self.state.clone()).await;
            }
        }
        self.initialized_commands = true;
    }
    pub fn builder() -> BotBuilder {
        BotBuilder::new()
    }
}

#[derive(Default)]
pub struct BotBuilder {
    groups: Vec<Group>,
}

impl BotBuilder {
    fn new() -> Self {
        Self::default()
    }
    pub fn group(mut self, group: Group) -> Self {
        self.groups.push(group);
        self
    }
    pub fn groups(mut self, groups: Vec<Group>) -> Self {
        self.groups = groups;
        self
    }
    pub fn build(self) -> Bot {
        Bot::new(self.groups)
    }
}
