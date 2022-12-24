use mockall::automock;

#[automock]
pub trait PathReader {
    fn read_dir(self: &mut Self, path: &String) -> &mut Vec<PathInfo>;
}

#[derive(Debug)]
pub struct SimpleRecursivePathReader {
    pub folders: Vec<PathInfo>,
}

#[derive(Debug, Default)]
pub struct PathInfo {
    pub size: f32,
    pub path: String,
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
                let size_in_mb = meta_data.len() as f32 / 1000000 as f32;

                self.folders.push(PathInfo {
                    size: size_in_mb,
                    path,
                });
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
