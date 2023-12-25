use super::command::Command;

pub struct Group {
    pub build_commands: fn() -> Vec<Command>,
    pub setup: fn(),
}
