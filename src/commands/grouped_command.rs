use std::collections::HashMap;

use crate::readers::input_command_reader::{Command, CommandParams};
use crate::readers::path_reader::PathInfo;

pub struct GroupCommand {
    line_separator: char,
}

impl Command for GroupCommand {
    fn name(&self) -> String {
        "grouped".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        let mut m = HashMap::new();
        let mut result = Vec::new();

        for path_info in &mut *payload {
            for x in path_info.path.split(self.line_separator) {
                *m.entry(x).or_insert(0.0) += path_info.size
            }
        }

        for (key, val) in m {
            result.push(PathInfo {
                path: key.to_string(),
                size: val,
            });
        }

        payload.clear();
        payload.append(&mut result);
    }

    fn parse_params(&mut self, params: &CommandParams) {
        // do nothing
    }
}

impl GroupCommand {
    pub fn new() -> Self {
        GroupCommand { line_separator: std::path::MAIN_SEPARATOR }
    }
}