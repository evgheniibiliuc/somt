use std::str::FromStr;

use crate::{readers::input_command_reader::Command, readers::{path_reader::PathInfo, input_command_reader::CommandParams}};

#[derive(Debug)]
pub struct SortCommand {
    sort: Sort,
}

impl Command for SortCommand {
    fn name(&self) -> String {
        "sort".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        payload.sort_by(|a, b| match self.sort {
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
    use crate::readers::input_command_reader::Command;
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
        sort_command.sort = Sort::DESC;

        let mut payload: Vec<PathInfo> = vec![];

        fill_path_info(0, 10.0, "/video", &mut payload);
        fill_path_info(1, 20.0, "/pictures", &mut payload);
        fill_path_info(2, 30.0, "/music", &mut payload);
        fill_path_info(3, 40.0, "/games", &mut payload);

        sort_command.apply(&mut payload);

        assert_eq!(4, payload.len());
        assert_path_info(0, 40.0, "/games", &mut payload);
        assert_path_info(1, 30.0, "/music", &mut payload);
        assert_path_info(2, 20.0, "/pictures", &mut payload);
        assert_path_info(3, 10.0, "/video", &mut payload);
    }

    #[test]
    fn sorts_by_file_size_asc() {
        let mut sort_command = SortCommand::new();
        sort_command.sort = Sort::ASC;

        let mut payload: Vec<PathInfo> = vec![];

        fill_path_info(0, 40.0, "/games", &mut payload);
        fill_path_info(1, 30.0, "/music", &mut payload);
        fill_path_info(2, 20.0, "/pictures", &mut payload);
        fill_path_info(3, 10.0, "/video", &mut payload);

        sort_command.apply(&mut payload);

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
