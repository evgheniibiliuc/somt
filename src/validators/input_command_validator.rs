use std::collections::HashMap;

use crate::readers::input_command_reader::Command;

pub struct CommandValidator<'a> {
    commands: HashMap<String, &'a dyn Command>,
}

impl<'a> CommandValidator<'a> {
    pub fn validate(
        &self,
        parsed_comands: &'a Vec<(String, String)>,
    ) -> Result<Vec<(&'a dyn Command, String)>, String> {
        let mut erros: Vec<String> = Vec::new();
        let result: Vec<(&dyn Command, String)> = Vec::new();

        for parsed_command in parsed_comands {
            let command_name = &parsed_command.0;

            match self.commands.get(command_name) {
                Some(cmd) => result.push((cmd, parsed_command.1)),
                None => {
                    let err = format!("[!] Invalid command [{}]", parsed_command.0);

                    erros.push(err);
                }
            }
        }

        if erros.is_empty() {
            return Ok(result);
        }

        for error in erros {
            print!("{}", error);
        }

        return Err("Failed validation".to_string());
    }

    pub fn new(commands_unsorted: Vec<&'a dyn Command>) -> Self {
        let mut commands = HashMap::new();

        for ele in commands_unsorted {
            commands.insert(ele.name(), ele);
        }

        CommandValidator { commands }
    }
}
