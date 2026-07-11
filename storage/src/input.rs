pub enum Command {
    GET,
    SET,
    NONE,
}

pub struct Input {
    pub command: Command,
    pub args: [Option<String>; 2],
}

impl Input {
    pub fn new(value: String) -> Self {
        let mut parsed = value.splitn(3, ' ');
        let command_str = parsed.next().unwrap_or("");
        let key_arg = parsed.next().map(str::to_string);
        let value_arg = parsed.next().map(str::to_string);
        let command = match command_str {
            "get" => Command::GET,
            "set" => Command::SET,
            _ => Command::NONE,
        };

        Self {
            command,
            args: [key_arg, value_arg],
        }
    }
}
