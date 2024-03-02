use crate::commands::main::{Command, PayloadContext};
use crate::commands::main::CommandParams;

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

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        payload_context.path_infos
            .retain(|path_info| path_info.path.ends_with(self.end_of_file.as_str()));
    }

    fn parse_params(&mut self, params: &CommandParams) {
        self.end_of_file = params.command_value.to_string();
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
    fn filters_out_non_valid_files() {
        let mut command = EndsWithCommand::new();
        command.end_of_file = ".iso".to_string();

        let mut payload_context = PayloadContext {
            path_infos: vec![
                PathInfo::new(20.1, "game.exe"),
                PathInfo::new(2220.51, "book.pdf"),
                PathInfo::new(72.0, "av.iso"),
            ]
        };
        
        command.apply(&mut payload_context);
        
        assert_eq!(1, payload_context.path_infos.len());
        match payload_context.path_infos.get(0) {
            Some(path_info) => {
                assert_eq!(72.0, path_info.size);
                assert_eq!("av.iso", path_info.path);
            }
            None => panic!("av.iso element is expected to be present"),
        }
    }
}
