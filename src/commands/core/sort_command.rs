use std::str::FromStr;

use mockall::automock;

use crate::commands::core::sort_command::Sort::DESC;
use crate::commands::main::{Command, PayloadContext};
use crate::commands::main::CommandParams;

#[derive(Debug)]
pub struct SortCommand {
    sort: Sort,
}

#[automock]
impl Command for SortCommand {
    fn name(&self) -> String {
        "sort".to_string()
    }

    fn apply(&mut self, payload_context: &mut PayloadContext) {
        payload_context.path_infos.sort_by(|a, b| match self.sort {
            Sort::DESC => b.size.total_cmp(&a.size),
            Sort::ASC => a.size.total_cmp(&b.size),
        });
    }

    fn parse_params(&mut self, params: &CommandParams) {
        self.sort = Sort::from_str(&params.command_value)
            .unwrap_or_else(|err| err)
    }
}

impl SortCommand {
    pub fn new() -> Self {
        SortCommand { sort: Sort::ASC }
    }

    pub fn desc() -> Self {
        SortCommand {
            sort: DESC
        }
    }
}

impl FromStr for Sort {
    type Err = Sort;

    fn from_str(input: &str) -> Result<Sort, Self::Err> {
        match input {
            "desc" => Ok(Sort::DESC),
            "asc" => Ok(Sort::ASC),
            _ => Err(Sort::ASC),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Sort {
    DESC,
    ASC,
}

#[cfg(test)]
mod tests {
    use crate::commands::main::{Command, PayloadContext};
    use crate::readers::path_reader::PathInfo;

    use super::Sort;
    use super::SortCommand;

    #[test]
    fn returns_sort_as_command_id() {
        let sort_command = SortCommand::new();
        assert_eq!("sort", sort_command.name());
    }

    #[test]
    fn sorts_by_file_size_desc() {
        let mut sort_command = SortCommand::new();
        let mut payload_context = PayloadContext{
            path_infos: vec![
                PathInfo::new(10.0, "/video"),
                PathInfo::new(20.0, "/pictures"),
                PathInfo::new(30.0, "/music"),
                PathInfo::new(40.0, "/games"),
            ],
        };

        sort_command.sort = Sort::DESC;
        
        
        sort_command.apply(&mut payload_context);
        let mut payload = payload_context.path_infos;
        
        assert_eq!(4, payload.len());
        assert_path_info(0, 40.0, "/games", &mut payload);
        assert_path_info(1, 30.0, "/music", &mut payload);
        assert_path_info(2, 20.0, "/pictures", &mut payload);
        assert_path_info(3, 10.0, "/video", &mut payload);
    }

    #[test]
    fn sorts_by_file_size_asc() {
        let mut sort_command = SortCommand::new();
        let mut payload_context = PayloadContext { 
            path_infos: vec![
                PathInfo::new(40.0, "/games"),
                PathInfo::new(30.0, "/music"),
                PathInfo::new(20.0, "/pictures"),
                PathInfo::new(10.0, "/video"),
            ]
        };
        sort_command.sort = Sort::ASC;
                
        sort_command.apply(&mut payload_context);
        
        let mut payload: Vec<PathInfo> = payload_context.path_infos;
        
        assert_eq!(4, payload.len());
        assert_path_info(0, 10.0, "/video", &mut payload);
        assert_path_info(1, 20.0, "/pictures", &mut payload);
        assert_path_info(2, 30.0, "/music", &mut payload);
        assert_path_info(3, 40.0, "/games", &mut payload);
    }

    fn fill_path_info(index: usize, size: f32, path: &str, paths: &mut Vec<PathInfo>) {
        paths.insert(
            index,
            PathInfo::new(size, path),
        );
    }

    fn assert_path_info(index: usize, size: f32, path: &str, paths: &mut Vec<PathInfo>) {
        match paths.get(index) {
            Some(path_info) => {
                assert_eq!(size, path_info.size);
                assert_eq!(path, path_info.path);
            }
            None => panic!("Element at [{}] index is expected to be present", index),
        }
    }
}
