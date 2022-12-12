
use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::{PathInfo, PathReader},
};

#[derive(Debug)]
pub struct DirCommand {
    pub path: String,
    pub path_reader : PathReader
}

impl Command for DirCommand {
    fn name(&self) -> String {
        "dir".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        let result = self.path_reader.read_dir(&self.path);

        payload.append(result);
    }

    fn parse_params(&mut self, params: String) {
        self.path = params;
    }
}