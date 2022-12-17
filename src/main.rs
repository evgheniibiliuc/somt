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
use readers::path_reader::PathReader;
use std::io;
use validators::input_command_validator::CommandValidator;

use crate::readers::path_reader::PathInfo;

fn main() {
    let mut limited: Box<dyn Command> = Box::new(LimitCommand { limit: 100 });
    let mut help: Box<dyn Command> = Box::new(HelpCommand {});
    let mut payload_printer: Box<dyn Command> = Box::new(PayloadPrinterCommand {});
    let mut dir_read: Box<dyn Command> = Box::new(DirReadCommand {
        path: "/".to_string(),
        path_reader: PathReader::new(),
    });

    let mut_commands = vec![
        limited.as_ref(),
        help.as_ref(),
        payload_printer.as_ref(),
        dir_read.as_ref(),
    ];

    let command_parser = CommandParser::new("-".to_string(), "=".to_string());
    let command_validator = CommandValidator::new(mut_commands);
    let command_evaluator = CommandEvaluator::new();

    loop {
        let mut input = String::new();
        let mut payload: Vec<PathInfo> = Vec::new();

        println!("Welcome:");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to handle reponse");

        let parsed_commands = command_parser.parse(input.to_owned());

        match command_validator.validate(&parsed_commands) {
            Ok(cmds) => command_evaluator.evaluate(vec![], payload.as_mut()),
            Err(_) => todo!(),
        }
    }
}
