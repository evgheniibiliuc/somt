// pub mod path_reader;

use crate::{readers::input_command_reader::Command, readers::path_reader::PathInfo};

#[derive(Debug)]
pub struct LimitCommand {
    pub limit: usize,
}

impl LimitCommand {
    pub fn new() -> Self {
        LimitCommand { limit: 100 }
    }
}
impl Command for LimitCommand {
    fn name(&self) -> String {
        "limit".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        payload.truncate(self.limit);
    }

    fn parse_params(&mut self, params: String) {
        self.limit = match params.parse() {
            Ok(val) => val,
            Err(_) => self.limit,
        };
    }
}
