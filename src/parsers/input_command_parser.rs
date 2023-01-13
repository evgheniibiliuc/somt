use std::collections::HashMap;

use crate::readers::input_command_reader::{CommandOption, CommandParams};

pub struct CommandParser {
    command_delimeter_mark: String,
    command_options_mark: String,
    value_delimeter_mark: String,
}

impl CommandParser {
    pub fn parse(&self, console_line: String) -> HashMap<String, CommandParams> {
        let mut result: HashMap<String, CommandParams> = HashMap::new();
        let mut latest_command: String = String::new();

        let commands_and_values: Vec<&str> =
            console_line.split(&self.command_delimeter_mark).collect();

        for command_and_value in commands_and_values {
            let param_value_list: Vec<&str> = command_and_value
                .split(&self.value_delimeter_mark)
                .filter(|str| !str.is_empty())
                .collect();

            let command_name = match param_value_list.get(0) {
                Some(key) => key.trim(),
                None => panic!("Unable to parse key {:?}", param_value_list),
            };

            let command_val = match param_value_list.get(1) {
                Some(key) => key.trim(),
                None => "",
            };

            println!(
                "Command name : [{}] and value [{}]",
                command_name, command_val
            );

            if self.is_command_option(command_and_value) {
                let command_params = result.get_mut(&latest_command);

                match command_params {
                    Some(command_param) => command_param.command_options.push(CommandOption {
                        name: command_name.replace(&self.command_options_mark, ""),
                        value: command_val.to_string(),
                    }),
                    None => println!("[!] Unable to find command [{:?}]", command_and_value),
                }
            } else {
                latest_command = command_name.to_string();

                result.insert(
                    command_name.to_string(),
                    CommandParams {
                        command_value: command_val.to_string(),
                        command_options: Vec::new(),
                    },
                );
            }
        }

        result
    }

    pub fn is_command_option(&self, command: &str) -> bool {
        command.starts_with(&self.command_options_mark)
    }

    pub fn new(
        command_delimeter_mark: &str,
        command_options_mark: &str,
        value_delimeter_mark: &str,
    ) -> Self {
        CommandParser {
            command_delimeter_mark: command_delimeter_mark.to_string(),
            value_delimeter_mark: value_delimeter_mark.to_string(),
            command_options_mark: command_options_mark.to_string(),
        }
    }
}
