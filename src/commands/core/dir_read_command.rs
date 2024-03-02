use mockall::automock;

use crate::{
    commands::main::Command,
    readers::path_reader::{PathInfo, PathReader, SimpleRecursivePathReader},
};
use crate::commands::main::{CommandParams, PayloadContext};

pub struct DirReadCommand {
    pub path: String,
    path_reader: Box<dyn PathReader>,
}

#[automock]
impl DirReadCommand {
    pub fn new() -> Self {
        DirReadCommand {
            path: "/".to_string(),
            path_reader: Box::new(SimpleRecursivePathReader::new()),
        }
    }
}

impl Command for DirReadCommand {
    fn name(&self) -> String {
        "dir_read".to_string()
    }

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        let result = self.path_reader.read_dir(&self.path);

        payload_context.path_infos.append(result);
    }

    fn parse_params(&mut self, params: &CommandParams) {
        self.path = params.command_value.to_string();
    }
}

#[cfg(test)]
mod tests {
    use crate::readers::path_reader::MockPathReader;

    use super::*;

    #[test]
    fn returns_dir_read_as_command_id() {
        let dir_read_cmd = DirReadCommand::new();
        assert_eq!("dir_read", dir_read_cmd.name());
    }

    #[test]
    fn returns_available_paths() {
        let mut mock_reader = Box::new(MockPathReader::new());
        mock_reader.expect_read_dir().returning(|_path| {
            let mut path_infos: Vec<PathInfo> = Vec::new();
            path_infos.insert(
                0,
                PathInfo::new(10.1, "/home"),
            );
            path_infos
        });
        
        let mut payload_context = PayloadContext::new();
        let mut command = DirReadCommand {
            path: "/home".to_string(),
            path_reader: mock_reader,
        };
        
        command.apply(&mut payload_context);

        let result: Vec<PathInfo> = payload_context.path_infos;

        assert_eq!(false, result.is_empty());
    }
}
