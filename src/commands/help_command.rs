use crate::{readers::input_command_reader::Command, readers::{path_reader::PathInfo, input_command_reader::CommandParams}};

#[derive(Debug)]
pub struct HelpCommand {}
impl HelpCommand {
    pub fn new() -> Self {
        HelpCommand {}
    }
}

impl Command for HelpCommand {
    fn name(&self) -> String {
        "help".to_string()
    }

    fn apply(&mut self, _payload: &mut Vec<PathInfo>) {
        println!(
            "# Basic operation: 
                      dir_read=/IdeaProjects/somt grouped sort=desc limit=10 print",
        )
    }

    fn parse_params(&mut self, _params: &CommandParams) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_help_as_command_id() {
        let help_command = HelpCommand::new();

        assert_eq!("help", help_command.name());
    }

    #[test]
    fn doesnt_mutate_payload() {
        let mut help_command = HelpCommand::new();
        let mut payload: Vec<PathInfo> = Vec::new();

        help_command.apply(&mut payload);

        assert!(payload.is_empty());
    }
}
