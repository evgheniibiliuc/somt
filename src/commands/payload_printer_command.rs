use crate::{
    readers::input_command_reader::Command,
    readers::path_reader::PathInfo,
};

#[derive(Debug)]
pub struct PayloadPrinterCommand {}

impl Command for PayloadPrinterCommand {
    fn name(&self) -> String {
        "print".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        for ele in payload {
            println!("File [{}] - [{}] MB", ele.path, ele.size)
        }
    }

    fn parse_params(&mut self, _params: String) {}
}
