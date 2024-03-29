use crate::commands::main::{Command, PayloadContext};
use crate::commands::main::CommandParams;

#[derive(Debug)]
pub struct HelpCommand {}

impl HelpCommand {
    pub fn _new() -> Self {
        HelpCommand {}
    }
}

impl Command for HelpCommand {
    fn name(&self) -> String {
        "help".to_string()
    }

    fn apply(&mut self, _payload_context: &mut PayloadContext) {
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
        let help_command = HelpCommand::_new();

        assert_eq!("help", help_command.name());
    }

    #[test]
    fn doesnt_mutate_payload() {
        let mut help_command = HelpCommand::_new();
        let mut payload: PayloadContext = PayloadContext::new();

        help_command.apply(&mut payload);

        assert!(payload.path_infos.is_empty());
    }
}
