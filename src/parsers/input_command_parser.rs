use crate::commands::main::{CommandOption, CommandParams};

pub struct CommandParser {
    command_delimeter_mark: String,
    _command_options_mark: String,
    value_delimeter_mark: String,
}

impl CommandParser {
    pub fn parse(&self, console_line: String) -> Vec<(String, CommandParams)> {
        let mut result: Vec<(String, CommandParams)> = Vec::new();

        let commands_and_values: Vec<&str> =
            console_line.split(&self.command_delimeter_mark).collect();

        for command_and_value in commands_and_values {
            if self.is_command_option(command_and_value) {
                let command_options = command_and_value.split(&self.value_delimeter_mark).collect::<Vec<&str>>();
                match result.last_mut() {
                    None => {}
                    Some(name_command) => {
                        let command_name = command_options.get(0)
                            .map(|full_option_name| {
                                full_option_name.replace(&self._command_options_mark, "").trim().to_string()
                            })
                            .expect("Unable to parse option name");
                        let command_val = command_options.get(1)
                            .unwrap_or(&"")
                            .trim()
                            .to_string();

                        name_command.1.command_options
                            .push(CommandOption {
                                name: command_name,
                                value: command_val,
                            });
                    }
                };
                continue;
            }

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

            result.push((command_name.to_string(), CommandParams {
                command_value: command_val.to_string(),
                command_options: Vec::new(),
            }));
        }


        result
    }

    pub fn is_command_option(&self, command: &str) -> bool {
        command.starts_with(&self._command_options_mark)
    }

    pub fn new(
        command_delimeter_mark: &str,
        command_options_mark: &str,
        value_delimeter_mark: &str,
    ) -> Self {
        CommandParser {
            command_delimeter_mark: command_delimeter_mark.to_string(),
            value_delimeter_mark: value_delimeter_mark.to_string(),
            _command_options_mark: command_options_mark.to_string(),
        }
    }
}
