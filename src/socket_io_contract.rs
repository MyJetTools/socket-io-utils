use crate::*;

pub enum SocketIoWsContract {
    Open,
    Close,
    Ping { with_probe: bool },
    Pong { with_probe: bool },
    Message(SocketIoMessage),
    Upgrade,
    Noop,
}

impl SocketIoWsContract {
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
                    SocketIoWsContract::Ping { with_probe: true }
                } else {
                    SocketIoWsContract::Ping { with_probe: false }
                }
            }
            '3' => {
                if src.len() > 1 {
                    SocketIoWsContract::Pong { with_probe: true }
                } else {
                    SocketIoWsContract::Pong { with_probe: false }
                }
            }
            '4' => {
                let msg = SocketIoMessage::deserialize(&src[1..]);
                SocketIoWsContract::Message(msg)
            }
            '5' => SocketIoWsContract::Upgrade,
            '6' => SocketIoWsContract::Noop,
            _ => panic!("Invalid socket.io payload {}", src),
        }
    }

    pub fn serialize(&self, out: &mut SocketIoPayload) {
        match self {
            SocketIoWsContract::Open => {
                out.text_frame.push('0');
            }
            SocketIoWsContract::Close => {
                out.text_frame.push('1');
            }
            SocketIoWsContract::Ping { with_probe } => {
                out.text_frame.push('2');
                if *with_probe {
                    out.text_frame.push_str("probe");
                }
            }
            SocketIoWsContract::Pong { with_probe } => {
                out.text_frame.push('3');

                if *with_probe {
                    out.text_frame.push_str("probe");
                }
            }
            SocketIoWsContract::Message(msg) => {
                out.text_frame.push('4');
                msg.serialize(out);
            }
            SocketIoWsContract::Upgrade => {
                out.text_frame.push('5');
            }
            SocketIoWsContract::Noop => {
                out.text_frame.push('6');
            }
        }
    }
}
