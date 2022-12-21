use std::collections::HashMap;

use super::path_reader::PathInfo;
pub struct CommandEvaluator {}

impl<'a> CommandEvaluator {
    pub fn evaluate(
        &self,
        parsed_commands_values: Vec<(String, String)>,
        commands_by_name: &mut HashMap<String, &mut dyn  Command>,
        payload: &mut Vec<PathInfo>,
    ) {
        for parsed_command_value in parsed_commands_values {
            match commands_by_name.get_mut(&parsed_command_value.0) {
                Some(cmd) => {
                    cmd.parse_params(parsed_command_value.1);
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

    fn parse_params(&mut self, params: String);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ name: {:?} }}", self.name())
    }
}
