use std::collections::HashMap;

use crate::readers::input_command_reader::{Command, CommandParams};

pub struct CommandValidator {}

impl CommandValidator {
    pub fn validate(
        &self,
        parsed_commands: &Vec<(String, CommandParams)>,
        commands: &HashMap<String, Box<dyn Command>>
    ) -> Result<String, String> {
        let mut is_err_present = false;

        for (command_name, _command_params) in parsed_commands {
            match commands.get(command_name) {
                Some(_cmd) => (),
                None => {
                    println!("[!] Invalid command [{}]\n\r", command_name);
                    is_err_present = true;
                }
            }
        }

        if is_err_present {
            return Err("Failed validation".to_string());
        }

        return Ok("passed all checks".to_string());
    }

    pub fn new() -> Self {
        return CommandValidator {};
    }
}
