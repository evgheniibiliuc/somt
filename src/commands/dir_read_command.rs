use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::{PathInfo, PathReader},
};

#[derive(Debug)]
pub struct DirReadCommand {
    path: String,
    path_reader: PathReader,
}

impl DirReadCommand {
    pub fn new() -> Self {
        DirReadCommand {
            path: "/".to_string(),
            path_reader: PathReader::new(),
        }
    }
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
