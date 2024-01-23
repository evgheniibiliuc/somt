use crate::readers::{
    input_command_reader::{Command, CommandParams},
    path_reader::{PathReader, SimpleRecursivePathReader},
};

pub struct FindFileCommand {
    _search_path: String,
    file_name: String,
    _path_reader: Box<dyn PathReader>,
}

impl FindFileCommand {
    pub fn new() -> Self {
        FindFileCommand {
            file_name: "".to_string(),
            _path_reader: Box::new(SimpleRecursivePathReader::new()),
            _search_path: "/".to_string(),
        }
    }
}

impl Command for FindFileCommand {
    fn name(&self) -> String {
        "find_file".to_string()
    }

    fn apply(&mut self, _payload: &mut Vec<crate::readers::path_reader::PathInfo>) {}

    fn parse_params(&mut self, params: &CommandParams) {
        self.file_name = params.command_value.to_string()
    }
}
