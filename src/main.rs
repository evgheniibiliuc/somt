mod commands;
mod parsers;
mod readers;
mod validators;

use commands::dir_read_command::DirReadCommand;
use commands::help_command::HelpCommand;
use commands::limit_command::LimitCommand;
use commands::payload_printer_command::PayloadPrinterCommand;
use parsers::input_command_parser::CommandParser;
use readers::input_command_reader::{Command, CommandEvaluator};
use std::collections::HashMap;
use std::io::{self};
use validators::input_command_validator::CommandValidator;

use crate::commands::ends_with_command::EndsWithCommand;
use crate::commands::find_file_command::FindFileCommand;
use crate::commands::sort_command::SortCommand;
use crate::readers::path_reader::PathInfo;

fn main() {
    loop {
        let mut limited: Box<dyn Command> = Box::new(LimitCommand::new());
        let mut help: Box<dyn Command> = Box::new(HelpCommand::new());
        let mut payload_printer: Box<dyn Command> = Box::new(PayloadPrinterCommand::new());
        let mut dir_read: Box<dyn Command> = Box::new(DirReadCommand::new());
        let mut sort: Box<dyn Command> = Box::new(SortCommand::new());
        let mut file_extension: Box<dyn Command> = Box::new(EndsWithCommand::new());
        let mut find_file: Box<dyn Command> = Box::new(FindFileCommand::new());

        let mut commands: HashMap<String, &mut dyn Command> = HashMap::new();
        commands.insert(limited.name(), limited.as_mut());
        commands.insert(help.name(), help.as_mut());
        commands.insert(payload_printer.name(), payload_printer.as_mut());
        commands.insert(dir_read.name(), dir_read.as_mut());
        commands.insert(sort.name(), sort.as_mut());
        commands.insert(file_extension.name(), file_extension.as_mut());
        commands.insert(find_file.name(), find_file.as_mut());

        let command_parser = CommandParser::new(" ", "--", "=");
        let command_evaluator = CommandEvaluator::new();
        let command_validator = CommandValidator::new();

        let mut input = String::new();
        let mut payload: Vec<PathInfo> = Vec::new();

        println!("/+");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to handle reponse");

        let parsed_commands = command_parser.parse(input.to_owned());
        println!("{:?}", parsed_commands);
        match command_validator.validate(&commands, &parsed_commands) {
            Ok(_) => command_evaluator.evaluate(&parsed_commands, &mut commands, payload.as_mut()),
            Err(_) => println!("Please fix issues mentioned above"),
        };
    }
}
