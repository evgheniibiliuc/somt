use mockall::automock;

use crate::commands::main::{Command, PayloadContext};
use crate::commands::main::CommandParams;

#[derive(Debug)]
pub struct LimitCommand {
    pub limit: usize,
}


impl LimitCommand {
    pub fn new() -> Self {
        LimitCommand { limit: 0 }
    }
}

#[automock]
impl Command for LimitCommand {
    fn name(&self) -> String {
        "limit".to_string()
    }

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        if self.limit != 0 {
            payload_context.path_infos.truncate(self.limit);
        }
    }

    fn parse_params(&mut self, params: &CommandParams) {
        self.limit = match params.command_value.parse() {
            Ok(val) => val,
            Err(_) => self.limit,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::readers::path_reader::PathInfo;
    use super::*;

    #[test]
    fn returns_limit_as_command_id() {
        let limit_command = LimitCommand::new();

        assert_eq!("limit", limit_command.name());
    }

    #[test]
    fn truncates_payloads_size_to_limit() {
        let mut limit_command = LimitCommand::new();
        limit_command.limit = 1;
        let mut payload_context = PayloadContext {
            path_infos: vec![PathInfo::default(), PathInfo::default()]
        };

        limit_command.apply(&mut payload_context);

        assert_eq!(1, payload_context.path_infos.len());
    }
}
