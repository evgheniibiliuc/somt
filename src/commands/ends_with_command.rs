use crate::readers::input_command_reader::Command;

pub struct EndsWithCommand {
    end_of_file: String,
}

impl EndsWithCommand {
    pub fn new() -> Self {
        EndsWithCommand {
            end_of_file: "".to_string(),
        }
    }
}

impl Command for EndsWithCommand {
    fn name(&self) -> String {
        "ends_with".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<crate::readers::path_reader::PathInfo>) {
        payload.retain(|path_info| path_info.path.ends_with(self.end_of_file.as_str()));
    }

    fn parse_params(&mut self, params: String) {
        self.end_of_file = params;
    }
}
#[cfg(test)]
mod test {
    use crate::readers::path_reader::PathInfo;

    use super::*;

    #[test]
    fn returns_ends_with_as_command_id() {
        let ends_with_command = EndsWithCommand::new();
        assert_eq!("ends_with", ends_with_command.name());
    }

    #[test]
    fn consume_raw_string_as_end_of_file() {
        let mut command = EndsWithCommand::new();
        command.parse_params("+-1".to_string());

        assert_eq!("+-1", command.end_of_file);
    }

    #[test]
    fn filters_out_non_valid_files() {
        let mut command = EndsWithCommand::new();
        command.end_of_file = ".iso".to_string();
        let mut payload: Vec<PathInfo> = vec![
            PathInfo {
                size: 20.1,
                path: "game.exe".to_string(),
            },
            PathInfo {
                size: 2220.51,
                path: "book.pdf".to_string(),
            },
            PathInfo {
                size: 72.0,
                path: "av.iso".to_string(),
            },
        ];

        command.apply(&mut payload);

        assert_eq!(1, payload.len());
        match payload.get(0) {
            Some(path_info) => {
                assert_eq!(72.0, path_info.size);
                assert_eq!("av.iso", path_info.path);
            }
            None => panic!("av.iso element is expected to be present"),
        }
    }
}
