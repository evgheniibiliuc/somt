use crate::{readers::input_command_reader::Command, readers::path_reader::PathInfo};

#[derive(Debug)]
pub struct PayloadPrinterCommand {}

impl PayloadPrinterCommand {
    pub fn new() -> Self {
        PayloadPrinterCommand {}
    }
}
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

#[cfg(test)]
mod tests {
    use crate::readers::{input_command_reader::Command, path_reader::PathInfo};

    use super::PayloadPrinterCommand;

    #[test]
    fn returns_print_as_command_id() {
        let printer = PayloadPrinterCommand::new();
        assert_eq!("print", printer.name());
    }
    #[test]
    fn doesnt_mutate_payload() {
        let mut printer = PayloadPrinterCommand::new();
        let mut payload: Vec<PathInfo> = Vec::new();
        printer.apply(&mut payload);

        assert_eq!(true, payload.is_empty());
    }
}
