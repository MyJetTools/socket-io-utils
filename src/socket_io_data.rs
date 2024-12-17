use rust_extensions::StrOrString;

pub enum SocketIoData {
    String(StrOrString<'static>),
    Binary(Vec<u8>),
}

impl SocketIoData {
    pub fn unwrap_as_str(&self) -> &str {
        match self {
            SocketIoData::String(value) => value.as_str(),
            SocketIoData::Binary { .. } => panic!("Expected string, found binary"),
        }
    }
}

impl SocketIoData {
    pub fn parse(value: &str) -> Vec<Self> {
        let mut result = Vec::new();

        let mut value_to_add: Option<String> = None;
        let mut escape_mode = false;

        for c in value.chars() {
            let value_is_ready = match &mut value_to_add {
                Some(value_to_add_ref) => {
                    if escape_mode {
                        value_to_add_ref.push(c);
                        escape_mode = false;
                        false
                    } else {
                        match c {
                            '\\' => {
                                escape_mode = true;
                                false
                            }
                            '"' => true,
                            _ => {
                                value_to_add_ref.push(c);
                                false
                            }
                        }
                    }
                }
                None => {
                    if c == '"' {
                        value_to_add = Some(String::new());
                    }
                    false
                }
            };

            if value_is_ready {
                let value = value_to_add.take().unwrap();
                result.push(SocketIoData::String(value.into()));
            }
        }

        result
    }
}
