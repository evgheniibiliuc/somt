use std::collections::HashMap;

use crate::commands::main::{Command, CommandParams, PayloadContext};

pub struct CommandRunner {}

impl<'a> CommandRunner {
    pub fn run(
        &self,
        parsed_commands_values: &Vec<(String, CommandParams)>,
        commands_by_name: &mut HashMap<String, Box<dyn Command>>,
        payload_context: &mut PayloadContext,
    ) {
        for (command_name, command_params) in parsed_commands_values {
            match commands_by_name.get_mut(command_name) {
                Some(cmd) => {
                    cmd.parse_params(command_params);
                    cmd.apply(payload_context)
                }
                None => todo!(),
            }
        }
    }

    pub fn new() -> Self {
        CommandRunner {}
    }
}