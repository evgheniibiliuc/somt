// pub mod path_reader;

use crate::{readers::input_command_reader::Command, readers::{path_reader::PathInfo, input_command_reader::CommandParams}};

#[derive(Debug)]
pub struct LimitCommand {
    pub limit: usize,
}

impl LimitCommand {
    pub fn new() -> Self {
        LimitCommand { limit: 100 }
    }
}
impl Command for LimitCommand {
    fn name(&self) -> String {
        "limit".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        payload.truncate(self.limit);
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
    use super::*;

    #[test]
    fn returns_limit_as_command_id() {
        let limit_command = LimitCommand::new();

        assert_eq!("limit", limit_command.name());
    }

    // #[test]
    // fn parsers_limit_param_no_err() {
    //     let mut limit_command = LimitCommand::new();
    //     limit_command.parse_params();

    //     assert_eq!(30, limit_command.limit);
    // }

    // #[test]
    // fn pareses_limit_param_gets_default_if_err() {
    //     let mut limit_command = LimitCommand::new();
    //     limit_command.parse_params();

    //     assert_eq!(100, limit_command.limit);
    // }


    #[test]
    fn truncates_payloads_size_to_limit() {
        let mut limit_command = LimitCommand::new();
        limit_command.limit = 1;

        let mut payload: Vec<PathInfo> = Vec::new();
        payload.insert(0, PathInfo::default());
        payload.insert(1, PathInfo::default());

        limit_command.apply(&mut payload);

        assert_eq!(1, payload.len());
    }
}
