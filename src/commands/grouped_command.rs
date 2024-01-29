use std::collections::HashMap;
use mockall::automock;

use crate::readers::input_command_reader::{Command, CommandParams};
use crate::readers::path_reader::{PathInfo, PathType};
use crate::readers::path_reader::PathType::{FILE, FOLDER};

#[derive(Debug)]
pub struct GroupCommand {
    line_separator: char,
}

#[automock]
impl Command for GroupCommand {
    fn name(&self) -> String {
        "grouped".to_string()
    }

    fn apply(&mut self, payload: &mut Vec<PathInfo>) {
        let mut path_size_map: HashMap<String, (f32, PathType)> = HashMap::new();
        let mut result = Vec::new();

        for path_info in &mut *payload {
            for partial_path in path_info.path.split(self.line_separator) {
                if partial_path.is_empty() {
                    continue;
                }

                let full_path = path_info.path.find(partial_path)
                    .and_then(|val| Some(&path_info.path[..val]))
                    .and_then(|path| {
                        let mut str = path.to_string();
                        str.push_str(partial_path);
                        Some(str)
                    });

                match full_path {
                    None => {}
                    Some(val) => {
                        let file_type = if val == path_info.path { FILE } else { FOLDER };
                        let entry = path_size_map.entry(val).or_insert((0.0f32, file_type));
                        entry.0 += path_info.size;
                    }
                }
            }
        }

        for (key, val) in path_size_map {
            result.push(PathInfo {
                path: key,
                size: val.0,
                path_type: val.1,
            });
        }

        payload.clear();
        payload.append(&mut result);
    }

    fn parse_params(&mut self, _params: &CommandParams) {
        // no params
    }
}

impl GroupCommand {
    pub fn new() -> Self {
        GroupCommand { line_separator: std::path::MAIN_SEPARATOR }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::commands::grouped_command::GroupCommand;
    use crate::readers::input_command_reader::Command;
    use crate::readers::path_reader::{PathInfo, PathType};
    use crate::readers::path_reader::PathType::{FILE, FOLDER};

    #[test]
    fn returns_grouped_as_command_id() {
        let grouped = GroupCommand::new();

        assert_eq!("grouped", grouped.name());
    }

    #[test]
    pub fn returns_size_for_each_path() {
        let mut group_command = GroupCommand {
            line_separator: '/'
        };
        let mut payload = vec![
            PathInfo::new(1.0, "/d/test/files/file_one"),
            PathInfo::new(1.0, "/d/test/files/file_two"),
            PathInfo::new(1.0, "/d/test/files/file_three"),
        ];

        group_command.apply(&mut payload);

        let result: HashMap<String, &PathInfo> = payload.iter().map(|path_info: &PathInfo| (path_info.path.to_string(), path_info)).collect();

        let file_one = result.get(&"/d/test/files/file_one".to_string()).unwrap();
        let file_two = result.get(&"/d/test/files/file_two".to_string()).unwrap();
        let file_three = result.get(&"/d/test/files/file_three".to_string()).unwrap();
        let files = result.get(&"/d/test/files".to_string()).unwrap();
        let test = result.get(&"/d/test".to_string()).unwrap();
        let d = result.get(&"/d".to_string()).unwrap();

        println!("{:?}", result);
        assert_eq!(6, result.len());

        assert_path_info(1f32, "/d/test/files/file_one", FILE, file_one.to_owned());
        assert_path_info(1f32, "/d/test/files/file_two", FILE, file_two.to_owned());
        assert_path_info(1f32, "/d/test/files/file_three", FILE, file_three.to_owned());
        assert_path_info(3f32, "/d/test/files", FOLDER, files.to_owned());
        assert_path_info(3f32, "/d/test", FOLDER, test.to_owned());
        assert_path_info(3f32, "/d", FOLDER, d.to_owned());
    }

    fn assert_path_info(size: f32, path: &str, path_type: PathType, path_info: &PathInfo) {
        assert_eq!(size, path_info.size);
        assert_eq!(path, path_info.path);
        assert_eq!(path_type, path_info.path_type);
    }
}