
use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::{PathInfo, PathReader},
};

#[derive(Debug)]
pub struct DirReadCommand {
    pub path: String,
    pub path_reader : PathReader
}

impl Command for DirReadCommand {
    fn name(&self) -> String {
        "dir_read".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        let result = self.path_reader.read_dir(&self.path);

        payload.append(result);
    }

    fn parse_params(&mut self, params: String) {
        self.path = params;
    }
}