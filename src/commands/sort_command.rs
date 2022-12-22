use std::str::FromStr;

use crate::{readers::input_command_reader::Command, readers::path_reader::PathInfo};

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

    fn parse_params(&mut self, _params: String) {
        self.sort = match Sort::from_str(&_params) {
            Ok(val) => val,
            Err(err) => err,
        }
    }
}
impl SortCommand {
    pub fn new() -> Self {
        SortCommand { sort: Sort::ASC }
    }
}
impl FromStr for Sort {
    fn from_str(input: &str) -> Result<Sort, Self::Err> {
        match input {
            "desc" => Ok(Sort::DESC),
            "asc" => Ok(Sort::ASC),
            _ => Err(Sort::ASC),
        }
    }

    type Err = Sort;
}

#[derive(Debug, PartialEq)]
enum Sort {
    DESC,
    ASC,
}
