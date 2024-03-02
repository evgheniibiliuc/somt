use crate::readers::path_reader::PathInfo;

pub trait Command {
    fn name(&self) -> String;

    fn apply(&mut self, payload: &mut PayloadContext);

    fn parse_params(&mut self, params: &CommandParams);

    fn get_option_keys(&self) -> Vec<String> {
        vec![]
    }
}

#[derive(Debug)]
pub struct CommandOption {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct PayloadContext {
    pub path_infos: Vec<PathInfo>,
}

impl PayloadContext {
    pub fn new() -> Self {
        PayloadContext {
            path_infos: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct CommandParams {
    pub command_value: String,
    pub command_options: Vec<CommandOption>,
}

impl CommandParams {
    pub fn _new(command_value: &str, command_options: Vec<CommandOption>) -> Self {
        CommandParams {
            command_value: command_value.to_string(),
            command_options,
        }
    }
}

impl std::fmt::Debug for dyn Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Command {{ name: {:?} }}", self.name())
    }
}
