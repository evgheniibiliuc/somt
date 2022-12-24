use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::{PathInfo, PathReader, SimpleRecursivePathReader},
};

pub struct DirReadCommand {
    path: String,
    path_reader: Box<dyn PathReader>,
}

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

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        let result = self.path_reader.read_dir(&self.path);

        payload.append(result);
    }

    fn parse_params(&mut self, params: String) {
        self.path = params;
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
    fn parses_path_argument() {
        let mut dir_read_cmd = DirReadCommand::new();
        dir_read_cmd.parse_params("/home".to_string());

        assert_eq!("/home", dir_read_cmd.path);
    }

    #[test]
    fn returns_available_paths() {
        let mut mock_reader = Box::new(MockPathReader::new());
        mock_reader.expect_read_dir().returning(|_path| {
            let mut path_infos: Vec<PathInfo> = Vec::new();
            path_infos.insert(
                0,
                PathInfo {
                    size: 10.1,
                    path: "/home".to_string(),
                },
            );
            path_infos
        });

        let mut command = DirReadCommand {
            path: "/home".to_string(),
            path_reader: mock_reader,
        };

        let mut result: Vec<PathInfo> = vec![];

        command.apply(&mut result);

        assert_eq!(false, result.is_empty());
    }
}
