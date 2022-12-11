// pub mod path_reader;

use crate::{input_command_reader::Command, path_reader::PathInfo};

#[derive(Debug)]
pub struct LimitCommand {
    pub limit: usize,
}

impl Command for LimitCommand {
    fn name(&self) -> String {
        "limit".to_string()
    }

    fn apply(&self, payload: &mut Vec<PathInfo>) {
        payload.truncate(self.limit);
        payload.sort_by(|a, b| b.size.total_cmp(&a.size))
    }

    fn parse_params(&mut self, params: String) {
        self.limit = params.parse().unwrap();
    }
}
