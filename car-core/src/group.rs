use tokio::sync::RwLock;
use std::sync::Arc;
use typemap::TypeMap;
use super::command::{Command, BoxedFuture};

pub type GroupInitFunction = fn(Arc<RwLock<TypeMap>>) -> BoxedFuture<()>;

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub init: Option<GroupInitFunction>,
}
