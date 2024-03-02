use std::io::{self};

use commands::provider::command_provider::CommandProvider;
use parsers::input_command_parser::CommandParser;
use validators::input_command_validator::CommandValidator;

use crate::commands::main::PayloadContext;
use crate::runner::command_runner::CommandRunner;

mod commands;
mod parsers;
mod readers;
mod validators;
mod runner;

fn main() {
    let mut commands = CommandProvider::get_available_commands();
    let command_parser = CommandParser::new(" ", "--", "=");
    let command_evaluator = CommandRunner::new();
    let command_validator = CommandValidator::new();

    loop {
        let mut input = String::new();
        let mut payload_context = PayloadContext::new();

        println!("⠙");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to handle response");

        let parsed_commands = command_parser.parse(input.to_owned());

        match command_validator.validate(&parsed_commands, &commands) {
            Ok(_) => command_evaluator.run(&parsed_commands, &mut commands, &mut payload_context),
            Err(_) => println!("Please fix issues mentioned above"),
        }
    }
}
