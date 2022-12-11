use crate::{
    input_command_reader::Command,
    path_reader::{PathInfo, PathReader},
};

#[derive(Debug)]
pub struct DirCommand {
    pub path: String,
}

impl Command for DirCommand {
    fn name(&self) -> String {
        "dir".to_string()
    }

    fn apply(&self, payload: &mut Vec<PathInfo>) {
        let mut path_reader = PathReader::new();
        let result = path_reader.read_dir(&self.path);
      
        payload.append(result);
    }

    fn parse_params(&mut self, params: String) {
        self.path = params;
    }
}
