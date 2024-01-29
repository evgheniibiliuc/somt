use mockall::automock;

use crate::readers::path_reader::PathType::FILE;

#[automock]
pub trait PathReader {
    fn read_dir(self: &mut Self, path: &String) -> &mut Vec<PathInfo>;
}

#[derive(Debug)]
pub struct SimpleRecursivePathReader {
    pub folders: Vec<PathInfo>,
}

#[derive(Debug)]
pub struct PathInfo {
    pub size: f32,
    pub path: String,
    pub path_type: PathType,
}

impl PathInfo {
    pub fn new(size: f32, path: &str) -> PathInfo {
        PathInfo {
            size,
            path: path.to_string(),
            ..Default::default()
        }
    }

    pub fn copy(&self) -> PathInfo {
        PathInfo {
            size: self.size,
            path: self.path.to_string(),
            path_type: self.path_type,
        }
    }
}

impl Default for PathInfo {
    fn default() -> Self {
        PathInfo {
            size: 0.0,
            path: "/".to_string(),
            path_type: FILE,
        }
    }
}


#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PathType {
    FILE,
    FOLDER,
}

impl SimpleRecursivePathReader {
    pub fn new() -> SimpleRecursivePathReader {
        SimpleRecursivePathReader {
            folders: Vec::new(),
        }
    }

    fn _read_dir(self: &mut Self, path: &String) -> &mut Vec<PathInfo> {
        let read_paths = std::fs::read_dir(path);
        let paths;

        if read_paths.is_ok() {
            paths = read_paths.ok().unwrap();
        } else {
            println!("{:?}", read_paths);
            return &mut self.folders;
        }

        for path in paths {
            let path_info = path.unwrap();
            let meta_data = path_info.metadata().unwrap();
            let path = path_info.path().as_os_str().to_str().unwrap().to_string();

            if meta_data.is_dir() {
                self._read_dir(&path);
            } else {
                let size_in_mb = meta_data.len() as f32 / 1000000f32;

                self.folders.push(PathInfo::new(size_in_mb, path.as_str()));
            }
        }

        return &mut self.folders;
    }
}

impl PathReader for SimpleRecursivePathReader {
    fn read_dir(self: &mut Self, path: &String) -> &mut Vec<PathInfo> {
        self._read_dir(path)
    }
}
