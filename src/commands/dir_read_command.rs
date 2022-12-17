use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::{PathInfo, PathReader},
};

#[derive(Debug)]
pub struct DirReadCommand {
    pub path: String,
    pub path_reader: PathReader,
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
mod test {
    use mockall::{mock, automock};

    use super::*;

    #[test]
    fn returns_command_name() {
        let command = DirReadCommand {
            path: "/".to_string(),
            path_reader: PathReader::new(),
        };

        assert_eq!("dir_read".to_string(), command.name());
    }
}

