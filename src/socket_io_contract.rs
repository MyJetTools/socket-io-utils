use crate::*;

pub enum SocketIoContract {
    Open,
    Close,
    Ping { with_probe: bool },
    Pong { with_probe: bool },
    Message(SocketIoMessage),
    Upgrade,
    Noop,
}

impl SocketIoContract {
    pub fn deserialize(src: &str) -> Self {
        if src.is_empty() {
            panic!("Empty string");
        }

        let first_char = src.chars().next().unwrap();

        match first_char {
            '0' => Self::Open,
            '1' => Self::Close,
            '2' => {
                if src.len() > 1 {
                    Self::Ping { with_probe: true }
                } else {
                    Self::Ping { with_probe: false }
                }
            }
            '3' => {
                if src.len() > 1 {
                    Self::Pong { with_probe: true }
                } else {
                    Self::Pong { with_probe: false }
                }
            }
            '4' => {
                let msg = SocketIoMessage::deserialize(&src[1..]);
                Self::Message(msg)
            }
            '5' => Self::Upgrade,
            '6' => Self::Noop,
            _ => panic!("Invalid socket.io payload {}", src),
        }
    }

    pub fn serialize(&self, out: &mut SocketIoPayload) {
        match self {
            Self::Open => {
                out.text_frame.push('0');
            }
            Self::Close => {
                out.text_frame.push('1');
            }
            Self::Ping { with_probe } => {
                out.text_frame.push('2');
                if *with_probe {
                    out.text_frame.push_str("probe");
                }
            }
            Self::Pong { with_probe } => {
                out.text_frame.push('3');

                if *with_probe {
                    out.text_frame.push_str("probe");
                }
            }
            Self::Message(msg) => {
                out.text_frame.push('4');
                msg.serialize(out);
            }
            Self::Upgrade => {
                out.text_frame.push('5');
            }
            Self::Noop => {
                out.text_frame.push('6');
            }
        }
    }
}