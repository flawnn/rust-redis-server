
    pub struct Message {
        pub command: String,
        pub args: Vec<String>,
    }

    impl Message {
        pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
            let string = match String::from_utf8(bytes.to_vec()) {
                Ok(s) => s,
                Err(_) => return Err("Invalid message: invalid UTF-8 bytes".to_string()),
            };

            Self::from_string(string)
        }

        pub fn from_string(string: String) -> Result<Self, String> {
            let mut parts = string.split("\r\n").filter(|&part| !part.is_empty());

            let command_size = parts.next().unwrap_or_else(|| return "Invalid Message")[1..]
                .parse::<usize>()
                .unwrap();

            let mut args = Vec::with_capacity(command_size);

            for _ in 0..command_size {
                let arg_len = match parts.next() {
                    Some(s) if s.starts_with("$") => {
                        let arg_len = match s[1..].parse::<usize>() {
                            Ok(arg_len) => arg_len,
                            Err(_) => {
                                return Err("Invalid message: invalid argument length".to_string())
                            }
                        };
                        arg_len
                    }
                    _ => return Err("Invalid message: invalid format".to_string()),
                };

                let arg_bytes = match parts.next() {
                    Some(arg_bytes) => arg_bytes.as_bytes(),
                    None => return Err("Invalid message: no argument bytes".to_string()),
                };

                let arg = String::from_utf8_lossy(arg_bytes).to_string();

                if arg_bytes.len() != arg_len {
                    return Err("Invalid message: argument length does not match bytes".to_string());
                }

                args.push(arg);
            }

            let command = match args.first() {
                Some(command) => command.clone(),
                None => return Err("Invalid message: no command".to_string()),
            };

            Ok(Message { command, args })
        }
    }