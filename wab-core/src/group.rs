use super::command::Command;
use typemap::ShareMap;

pub type GroupInitFunction = fn(&mut ShareMap);

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub init: Option<GroupInitFunction>,
}
