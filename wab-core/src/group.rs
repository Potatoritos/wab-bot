use crate::command::Command;
use typemap::ShareMap;

pub type GroupInitFunction = fn(&mut ShareMap);

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub build_events: fn() -> Vec<crate::Event>,
    pub init: Option<GroupInitFunction>,
}
