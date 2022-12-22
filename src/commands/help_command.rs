use crate::{readers::input_command_reader::Command, readers::path_reader::PathInfo};

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
                      dir_read=/home sort=desc limit=200 print",
        )
    }

    fn parse_params(&mut self, _params: String) {}
}
