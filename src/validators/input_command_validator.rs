use std::collections::HashMap;

use crate::readers::input_command_reader::{Command, CommandParams};

pub struct CommandValidator {}

impl CommandValidator {
    pub fn validate(
        &self,
        commands: &HashMap<String, &mut dyn Command>,
        parsed_comands: &Vec<(String, CommandParams)>,
    ) -> Result<String, String> {
        let mut is_err_present = false;

        for (command_name, _command_params) in parsed_comands {
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
