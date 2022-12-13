mod commands;
mod readers;

use commands::dir_read_command::DirReadCommand;
use commands::help_command::HelpCommand;
use commands::limit_command::LimitCommand;
use commands::payload_printer_command::PayloadPrinterCommand;
use readers::input_command_reader::{Command, CommandReader};
use readers::path_reader::PathReader;
use std::io;

fn main() {
    let mut limited: Box<dyn Command> = Box::new(LimitCommand { limit: 100 });
    let mut help: Box<dyn Command> = Box::new(HelpCommand {});
    let mut payload_printer: Box<dyn Command> = Box::new(PayloadPrinterCommand {});
    let mut dir_read: Box<dyn Command> = Box::new(DirReadCommand {
        path: "/".to_string(),
        path_reader: PathReader::new(),
    });

    let mut command_reader = CommandReader::new(
        "-",
        "=",
        vec![
            limited.as_mut(),
            help.as_mut(),
            payload_printer.as_mut(),
            dir_read.as_mut(),
        ],
    );

    loop {
        let mut input = String::new();

        println!("Welcome:");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to handle reponse");

        command_reader.read(input.to_owned());
    }
}
