use crate::command::Command;
use typemap_rev::TypeMap;

pub type GroupInitFunction = fn(&mut TypeMap);

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub build_events: fn() -> Vec<crate::Event>,
    pub init: Option<GroupInitFunction>,
}
