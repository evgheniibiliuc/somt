use std::collections::HashMap;
use crate::commands::main::{Command, CommandParams};

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