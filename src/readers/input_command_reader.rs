use std::collections::HashMap;

use super::path_reader::PathInfo;
pub struct CommandEvaluator {}

impl<'a> CommandEvaluator {
    pub fn evaluate(
        &self,
        parsed_commands_values: &Vec<(String, CommandParams)>,
        commands_by_name: &mut HashMap<String, Box<dyn Command>>,
        payload: &mut Vec<PathInfo>,
    ) {
        for (command_name, command_params) in parsed_commands_values {
            match commands_by_name.get_mut(command_name) {
                Some(cmd) => {
                    cmd.parse_params(command_params);
                    cmd.apply(payload)
                }
                None => todo!(),
            }
        }
    }

    pub fn new() -> Self {
        CommandEvaluator {}
    }
}

pub trait Command {
    fn name(&self) -> String;

    fn apply(&mut self, payload: &mut Vec<PathInfo>);

    fn parse_params(&mut self, params: &CommandParams);

    fn get_option_keys(&self) -> Vec<String> {
        vec![]
    }
}

#[derive(Debug)]
pub struct CommandOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct CommandParams {
    pub command_value: String,
    pub command_options: Vec<CommandOption>,
}

impl CommandParams {
    pub fn _new(command_value: &str, command_options: Vec<CommandOption>) -> Self {
        CommandParams {
            command_value: command_value.to_string(),
            command_options,
        }
    }
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ name: {:?} }}", self.name())
    }
}
