pub struct CommandParser {
    command_mark: String,
    value_delimeter_mark: String,
}

impl CommandParser {
    pub fn parse(&self, console_line: String) -> Vec<(String, String)> {
        console_line
            .split(&self.command_mark.to_owned())
            .filter(|str| !str.is_empty())
            .map(|str| str.trim())
            .map(|str| {
                let command_params: Vec<&str> = str.split(&self.value_delimeter_mark).collect();

                let command_name = match command_params.get(0) {
                    Some(key) => key,
                    None => panic!("Unable to parse key {:?}", command_params),
                };

                let command_val = match command_params.get(1) {
                    Some(key) => key,
                    None => "",
                };
                (command_name.to_string(), command_val.to_string())
            })
            .collect()
    }

    pub fn new(command_mark: String, value_delimeter_mark: String) -> Self {     
        CommandParser {
            command_mark,
            value_delimeter_mark,
        }
    }
}
