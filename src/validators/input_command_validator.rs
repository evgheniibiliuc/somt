use std::collections::HashMap;

use crate::readers::input_command_reader::Command;

pub struct CommandValidator {}

impl CommandValidator {
    pub fn validate(
        &self,
        commands: &HashMap<String, &mut dyn Command>,
        parsed_comands: &Vec<(String, String)>,
    ) -> Result<String, String> {
        let mut is_err_present = false;

        for parsed_command in parsed_comands {
            let command_name = &parsed_command.0;

            match commands.get(command_name) {
                Some(_cmd) => (),
                None => {
                    println!("[!] Invalid command [{}]\n\r", parsed_command.0);
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
