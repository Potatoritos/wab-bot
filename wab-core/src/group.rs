use crate::command::Command;
use crate::BoxedFuture;
use std::collections::HashMap;
use twilight_gateway::{Event, EventType};
use typemap::ShareMap;

pub type GroupInitFunction = fn(&mut ShareMap);
pub type EventFunction = fn(Event) -> BoxedFuture<()>;

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub build_events: fn() -> Vec<crate::Event>,
    pub init: Option<GroupInitFunction>,
}
