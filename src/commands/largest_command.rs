use std::collections::HashMap;
use std::str::FromStr;

use crate::commands::core::dir_read_command::DirReadCommand;
use crate::commands::core::grouped_command::GroupCommand;
use crate::commands::core::limit_command::LimitCommand;
use crate::commands::core::print_command::PrintCommand;
use crate::commands::core::sort_command::SortCommand;
use crate::commands::largest_command::Type::{FILE, FOLDER};
use crate::commands::main::{Command, PayloadContext};
use crate::commands::main::CommandParams;

use crate::readers::path_reader::{PathInfo, PathType};

pub struct LargestCommand {
    _type: Type,
    dir_read_command: DirReadCommand,
    sort_command: SortCommand,
    limit_command: LimitCommand,
    grouped_command: GroupCommand,
    print_command: PrintCommand,
}


impl LargestCommand {
    pub fn new() -> Self {
        LargestCommand {
            _type: FILE,
            dir_read_command: DirReadCommand::new(),
            sort_command: SortCommand::desc(),
            limit_command: LimitCommand::new(),
            grouped_command: GroupCommand::new(),
            print_command: PrintCommand::new(),
        }
    }
}

impl Command for LargestCommand {
    fn name(&self) -> String {
        "largest".to_string()
    }

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        self.dir_read_command.apply(payload_context);
        self.grouped_command.apply(payload_context);
        self.sort_command.apply(payload_context);
        self.limit_command.apply(payload_context);

        let mut output = payload_context.path_infos.iter()
            .filter(|path_info| { Type::from_path_type(&path_info.path_type) == self._type })
            .map(|path_info| path_info.copy())
            .collect::<Vec<PathInfo>>();

        payload_context.path_infos.clear();
        payload_context.path_infos.append(&mut output);

       
        self.print_command.apply(payload_context)
    }

    fn parse_params(&mut self, params: &CommandParams) {
        self._type = Type::from_str(&params.command_value).unwrap_or_default();

        let options: HashMap<&String, &String> = params.command_options.iter()
            .filter(|option| self.get_option_keys().contains(&option.name))
            .map(|option| (&option.name, &option.value))
            .into_iter()
            .collect();


        match options.get(&"location".to_string()) {
            None => {}
            Some(val) => {
                self.dir_read_command.path = val.to_string();
            }
        }

        match options.get(&"limit".to_string()) {
            None => {}
            Some(val) => {
                self.limit_command.limit = val.to_string()
                    .parse::<usize>()
                    .unwrap_or(100);
            }
        }
    }

    fn get_option_keys(&self) -> Vec<String> {
        vec!["location".to_string(), "limit".to_string()]
    }
}

#[derive(Debug, PartialEq)]
enum Type {
    FILE,
    FOLDER,
}

impl Type {
    fn from_path_type(path_type: &PathType) -> Self {
        match path_type {
            PathType::FILE => FILE,
            PathType::FOLDER => FOLDER
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        FILE
    }
}

impl FromStr for Type {
    type Err = Type;

    fn from_str(input: &str) -> Result<Type, Self::Err> {
        match input {
            "file" => Ok(FILE),
            "folder" => Ok(FOLDER),
            _ => Err(FILE)
        }
    }
}