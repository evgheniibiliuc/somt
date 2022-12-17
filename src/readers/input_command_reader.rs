use super::path_reader::PathInfo;
pub struct CommandEvaluator {}

impl<'a> CommandEvaluator {
    pub fn evaluate(
        &self,
        commands_with_params: Vec<(&'a mut dyn Command, String)>,
        payload: &mut Vec<PathInfo>,
    ) {
        for command_with_param in commands_with_params {
            let command = command_with_param.0;
            let command_args = command_with_param.1;

            command.parse_params(command_args);
            command.apply(payload);
        }
    }

    pub fn new() -> Self {
        CommandEvaluator {}
    }
}

pub trait Command {
    fn name(&self) -> String;

    fn apply(&mut self, payload: &mut Vec<PathInfo>);

    fn parse_params(&mut self, params: String);
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ name: {:?} }}", self.name())
    }
}
