use core::panic;
use std::collections::HashMap;

use crate::path_reader::PathInfo;

pub struct CommandReader<'a> {
    command_mark: String,
    value_delimeter_mark: String,
    commands: HashMap<String, &'a mut dyn Command>,
}

impl<'a> CommandReader<'a> {
    pub fn read(&mut self, console_line: String) {
        let mut result: Vec<PathInfo> = Vec::new();

        self.evaluate_commands(console_line, &mut result);
    }

    fn evaluate_commands(&mut self, console_line: String, payload: &mut Vec<PathInfo>) {
        console_line
            .split(&self.command_mark.as_str().to_owned())
            .filter(|str| !str.is_empty())
            .map(|str| str.trim())
            .for_each(|str| {
                let command_params: Vec<&str> = str.split(&self.value_delimeter_mark).collect();

                let command_name = match command_params.get(0) {
                    Some(key) => key,
                    None => panic!("Unable to parse key {:?}", command_params),
                };

                let command_val = match command_params.get(1) {
                    Some(key) => key,
                    None => "",
                };

                let current_command = match self.commands.get_mut(&command_name.to_string()) {
                    Some(key) => key,
                    None => panic!("Unable to find command for query {:?}", command_params),
                };

                current_command.parse_params(command_val.to_string());
                current_command.apply(payload);
            });
    }

    pub fn new(
        command_mark: String,
        value_delimeter_mark: String,
        commands_unsorted: Vec<&'a mut dyn Command>,
    ) -> Self {
        let mut commands = HashMap::new();

        for ele in commands_unsorted {
            commands.insert(ele.name(), ele);
        }

        CommandReader {
            value_delimeter_mark,
            command_mark,
            commands,
        }
    }
}

pub trait Command {
    fn name(&self) -> String;

    fn apply(&self, payload: &mut Vec<PathInfo>);

    fn parse_params(&mut self, params: String);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ name: {:?} }}", self.name())
    }
}
