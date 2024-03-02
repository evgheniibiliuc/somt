use mockall::automock;

use crate::commands::main::{Command, CommandParams, PayloadContext};

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

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        for ele in &mut payload_context.path_infos {
            println!("[{:?}] [{}] - [{}] MB", ele.path_type, ele.path, ele.size)
        }
    }

    fn parse_params(&mut self, _params: &CommandParams) {}
}

#[cfg(test)]
mod tests {
    use crate::commands::main::{Command, PayloadContext};

    use super::PrintCommand;

    #[test]
    fn returns_print_as_command_id() {
        let printer = PrintCommand::new();
        assert_eq!("print", printer.name());
    }

    #[test]
    fn doesnt_mutate_payload() {
        let mut printer = PrintCommand::new();
        let mut payload: PayloadContext = PayloadContext::new();
        printer.apply(&mut payload);

        assert_eq!(true, payload.path_infos.is_empty());
    }
}
