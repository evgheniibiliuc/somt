use mockall::automock;
use crate::commands::main::{Command, CommandParams};
use crate::readers::path_reader::PathInfo;


#[derive(Debug)]
pub struct PrintCommand {}

impl PrintCommand {
    pub fn new() -> Self {
        PrintCommand {}
    }
}

#[automock]
impl Command for PrintCommand {
    fn name(&self) -> String {
        "print".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        for ele in payload {
            println!("[{:?}] [{}] - [{}] MB", ele.path_type, ele.path, ele.size)
        }
    }

    fn parse_params(&mut self, _params: &CommandParams) {}
}

#[cfg(test)]
mod tests {
    use crate::commands::main::Command;
    use crate::readers::path_reader::PathInfo;


    use super::PrintCommand;

    #[test]
    fn returns_print_as_command_id() {
        let printer = PrintCommand::new();
        assert_eq!("print", printer.name());
    }

    #[test]
    fn doesnt_mutate_payload() {
        let mut printer = PrintCommand::new();
        let mut payload: Vec<PathInfo> = Vec::new();
        printer.apply(&mut payload);

        assert_eq!(true, payload.is_empty());
    }
}
