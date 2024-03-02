use std::collections::HashMap;

use crate::commands::largest_command::LargestCommand;
use crate::commands::main::Command;

pub struct CommandProvider {}

impl CommandProvider {
    pub fn get_available_commands() -> HashMap<String, Box<dyn Command>> {
        let largest: Box<dyn Command> = Box::new(LargestCommand::new());

        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert(largest.name(), largest);

        commands
    }
}