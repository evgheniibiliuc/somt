use std::io;

fn main() {
    loop {
        let path_print_amoount = 8;

        let mut file_path = String::new();
        println!("Please enter the path: ");

        io::stdin()
            .read_line(&mut file_path)
            .expect("Unable to handle reponse");

        file_path = file_path.trim().to_string();

        if !std::path::Path::new(&file_path.trim()).exists() {
            println!("Location [{}] doesnt exists", file_path);
            continue;
        }

        let mut reader = PathReader::new();
        let path_infos = reader.read_dir(&file_path);

        path_infos.sort_by(|a, b| a.size.total_cmp(&b.size).reverse());

        let mut i = 0;
        for info in path_infos {
            i += 1;
            if i > path_print_amoount {
                break;
            }
            println!("{} : {} MB", info.path.to_owned(), info.size);
        }
    }
}
struct PathReader {
    dirs_with_sie: Vec<PathInfo>,
}

struct PathInfo {
    size: f32,
    path: String,
    is_directory: bool,
}

impl PathReader {
    fn new() -> PathReader {
        PathReader {
            dirs_with_sie: Vec::new(),
        }
    }

    fn read_dir(self: &mut Self, path: &String) -> &mut Vec<PathInfo> {
        let paths = match std::fs::read_dir(path) {
            Result::Ok(val) => val,
            Result::Err(err) => panic!("Error [{}] during path [{}] resolution ", err, path),
        };

        for path in paths {
            let path_info = path.unwrap();
            let meta_data = path_info.metadata().unwrap();
            let path = path_info.path().as_os_str().to_str().unwrap().to_string();

            if meta_data.is_dir() {
                self.dirs_with_sie.push(PathInfo {
                    size: 0.0,
                    path: path.to_owned(),
                    is_directory: true,
                });

                self.read_dir(&path);
            }

            let size = meta_data.len() as f32 / 1000000 as f32;

            if meta_data.is_file() {
                self.dirs_with_sie.push(PathInfo {
                    size,
                    path,
                    is_directory: false,
                });
            }
        }

        return &mut self.dirs_with_sie;
    }
}
