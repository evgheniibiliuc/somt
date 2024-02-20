use std::collections::HashMap;
use std::str::FromStr;
use crate::commands::dir_read_command::DirReadCommand;
use crate::commands::grouped_command::GroupCommand;
use crate::commands::largest_command::Type::{FILE, FOLDER};
use crate::commands::limit_command::LimitCommand;
use crate::commands::print_command::PrintCommand;
use crate::commands::sort_command::SortCommand;
use crate::readers::input_command_reader::{Command, CommandParams};
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

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        self.dir_read_command.apply(payload);
        self.grouped_command.apply(payload);
        self.sort_command.apply(payload);


        let mut output = payload.iter()
            .filter(|path_info| { Type::from_path_type(&path_info.path_type) == self._type })
            .map(|path_info| path_info.copy())
            .collect::<Vec<PathInfo>>();

        payload.clear();
        payload.append(&mut output);

        self.limit_command.apply(payload);
        self.print_command.apply(payload)
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