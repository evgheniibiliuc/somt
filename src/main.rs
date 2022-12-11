pub mod dir_command;
pub mod help_command;
pub mod input_command_reader;
pub mod limit_command;
pub mod path_reader;
pub mod payload_printer_command;

use dir_command::DirCommand;
use help_command::HelpCommand;
use input_command_reader::{Command, CommandReader};
use limit_command::LimitCommand;
use payload_printer_command::PayloadPrinterCommand;
use std::io;

fn main() {
    let mut limited: Box<dyn Command> = Box::new(LimitCommand { limit: 100 });
    let mut help: Box<dyn Command> = Box::new(HelpCommand {});
    let mut payload_printer: Box<dyn Command> = Box::new(PayloadPrinterCommand {});
    let mut dir: Box<dyn Command> = Box::new(DirCommand {
        path: "/".to_string(),
    });

    let mut vec2: Vec<&mut dyn Command> = Vec::new();

    vec2.push(help.as_mut());
    vec2.push(payload_printer.as_mut());
    vec2.push(dir.as_mut());
    vec2.push(limited.as_mut());


    let mut command_reader = CommandReader::new("-".to_string(), "=".to_string(), vec2);

    loop {
        let mut input = String::new();

        println!("Welcome:");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to handle reponse");
        
        command_reader.read(input.to_owned());
    }
}
