use crate::command::Command;
use std::sync::Arc;
use tokio::sync::RwLock;
use typemap_rev::{TypeMap, TypeMapKey};

pub struct SetupContext {
    pub state: TypeMap,
}
impl SetupContext {
    pub fn create_state<T>(&mut self, initial_value: T)
    where
        T: TypeMapKey<Value = Arc<RwLock<T>>> + Sync + Send,
    {
        self.state.insert::<T>(Arc::new(RwLock::new(initial_value)));
    }
    pub fn new() -> Self {
        Self {
            state: TypeMap::new(),
        }
    }
}

pub type GroupSetupFunction = fn(&mut SetupContext);

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub build_events: fn() -> Vec<crate::Event>,
    pub setup: Option<GroupSetupFunction>,
}
