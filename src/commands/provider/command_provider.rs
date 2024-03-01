use std::collections::HashMap;
use crate::commands::main::Command;

use crate::commands::core::dir_read_command::DirReadCommand;
use crate::commands::core::ends_with_command::EndsWithCommand;
use crate::commands::core::find_file_command::FindFileCommand;
use crate::commands::core::grouped_command::GroupCommand;
use crate::commands::core::help_command::HelpCommand;
use crate::commands::largest_command::LargestCommand;
use crate::commands::core::limit_command::LimitCommand;
use crate::commands::core::payload_printer_command::PayloadPrinterCommand;
use crate::commands::core::sort_command::SortCommand;

pub struct CommandProvider {}

impl CommandProvider {
    pub fn get_available_commands() -> HashMap<String, Box<dyn Command>> {
        let limited: Box<dyn Command> = Box::new(LimitCommand::new());
        let help: Box<dyn Command> = Box::new(HelpCommand::new());
        let payload_printer: Box<dyn Command> = Box::new(PayloadPrinterCommand::new());
        let dir_read: Box<dyn Command> = Box::new(DirReadCommand::new());
        let sort: Box<dyn Command> = Box::new(SortCommand::new());
        let file_extension: Box<dyn Command> = Box::new(EndsWithCommand::new());
        let find_file: Box<dyn Command> = Box::new(FindFileCommand::new());
        let grouped: Box<dyn Command> = Box::new(GroupCommand::new());
        let largest: Box<dyn Command> = Box::new(LargestCommand::new());

        let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
        commands.insert(limited.name(), limited);
        commands.insert(help.name(), help);
        commands.insert(payload_printer.name(), payload_printer);
        commands.insert(dir_read.name(), dir_read);
        commands.insert(sort.name(), sort);
        commands.insert(file_extension.name(), file_extension);
        commands.insert(find_file.name(), find_file);
        commands.insert(grouped.name(), grouped);
        commands.insert(largest.name(), largest);

        commands
    }
}