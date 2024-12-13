use crate::*;

pub enum SocketIoProtocol {
    Open,
    Close,
    Ping { with_probe: bool },
    Pong { with_probe: bool },
    Message(SocketIoMessage),
    Upgrade,
    Noop,
}

impl SocketIoProtocol {
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
                    SocketIoProtocol::Ping { with_probe: true }
                } else {
                    SocketIoProtocol::Ping { with_probe: false }
                }
            }
            '3' => {
                if src.len() > 1 {
                    SocketIoProtocol::Pong { with_probe: true }
                } else {
                    SocketIoProtocol::Pong { with_probe: false }
                }
            }
            '4' => {
                let msg = SocketIoMessage::deserialize(&src[1..]);
                SocketIoProtocol::Message(msg)
            }
            '5' => SocketIoProtocol::Upgrade,
            '6' => SocketIoProtocol::Noop,
            _ => panic!("Invalid socket.io payload {}", src),
        }
    }

    pub fn serialize(&self, out: &mut String) {
        match self {
            SocketIoProtocol::Open => {
                out.push('0');
            }
            SocketIoProtocol::Close => {
                out.push('1');
            }
            SocketIoProtocol::Ping { with_probe } => {
                out.push_str("2");
                if *with_probe {
                    out.push_str("probe");
                }
            }
            SocketIoProtocol::Pong { with_probe } => {
                out.push_str("3");

                if *with_probe {
                    out.push_str("probe");
                }
            }
            SocketIoProtocol::Message(msg) => {
                out.push('4');
                msg.serialize(out);
            }
            SocketIoProtocol::Upgrade => {
                out.push('5');
            }
            SocketIoProtocol::Noop => {
                out.push('6');
            }
        }
    }
}
