use rust_extensions::StrOrString;

use crate::{SocketIoData, SocketIoDataSerializer};

pub enum SocketIoMessage {
    Connect {
        namespace: StrOrString<'static>,
        sid: Option<StrOrString<'static>>,
    },
    Disconnect {
        namespace: StrOrString<'static>,
    },
    Event {
        namespace: StrOrString<'static>,
        data: Vec<SocketIoData>,
        ack: Option<u64>,
    },
    Ack {
        namespace: StrOrString<'static>,
        data: Vec<SocketIoData>,
        ack: Option<u64>,
    },
    ConnectError {
        namespace: StrOrString<'static>,
        message: StrOrString<'static>,
    },
    BinaryEvent,
    BinaryAck,
}

impl SocketIoMessage {
    pub fn deserialize(value: &str) -> Self {
        let first_char = value.chars().next().unwrap();

        match first_char {
            '0' => {
                let (namespace, values) =
                    super::payload_deserializer::deserialize_data(&value[1..]);

                let sid =
                    values.and_then(|(key, value)| if key == "sid" { Some(value) } else { None });

                SocketIoMessage::Connect {
                    namespace: namespace.to_string().into(),
                    sid: sid.map(|s| s.to_string().into()),
                }
            }

            '1' => {
                let (namespace, _) = super::payload_deserializer::deserialize_data(&value[1..]);

                SocketIoMessage::Disconnect {
                    namespace: namespace.to_string().into(),
                }
            }

            '2' => {
                let (namespace, ack, data) =
                    super::payload_deserializer::deserialize_event_data(&value[1..]);

                SocketIoMessage::Event {
                    namespace: namespace.to_string().into(),
                    data,
                    ack,
                }
            }

            '3' => {
                let (namespace, ack, data) =
                    super::payload_deserializer::deserialize_event_data(&value[1..]);

                SocketIoMessage::Ack {
                    namespace: namespace.to_string().into(),
                    data,
                    ack,
                }
            }

            '4' => {
                let (namespace, message) =
                    super::payload_deserializer::deserialize_data(&value[1..]);

                let message = message.unwrap();

                SocketIoMessage::ConnectError {
                    namespace: namespace.to_string().into(),
                    message: message.1.into(),
                }
            }

            _ => {
                panic!("Invalid socket.io message {}", value);
            }
        }
    }

    pub fn serialize(&self, out: &mut String) {
        match self {
            SocketIoMessage::Connect { namespace, sid } => {
                out.push('0');
                serialize_data(
                    out,
                    namespace.as_str(),
                    sid.as_ref().map(|s| ("sid", s.as_str())),
                );
            }
            SocketIoMessage::Disconnect { namespace } => {
                out.push('1');
                serialize_data(out, namespace.as_str(), None);
            }
            SocketIoMessage::Event {
                namespace,
                data,
                ack,
            } => {
                out.push('2');

                serialize_event_data(out, namespace.as_str(), data, ack.clone());
            }
            SocketIoMessage::Ack {
                namespace,
                data,
                ack,
            } => {
                out.push('3');
                serialize_event_data(out, namespace.as_str(), data, ack.clone());
            }
            SocketIoMessage::ConnectError { namespace, message } => {
                out.push('4');
                serialize_data(out, namespace.as_str(), Some(("message", message.as_str())));
            }
            SocketIoMessage::BinaryEvent => {
                todo!("Not implemented yet")
            }
            SocketIoMessage::BinaryAck => {
                todo!("Not implemented yet")
            }
        }
    }
}

fn serialize_data(out: &mut String, namespace: &str, data: Option<(&str, &str)>) {
    if namespace != "/" {
        out.push_str(namespace);

        out.push(',');
    }

    if let Some(data) = data {
        out.push_str("{\"");
        out.push_str(data.0);
        out.push_str("\":\"");
        out.push_str(data.1);
        out.push_str("\"}");
    }
}

fn serialize_event_data(
    out: &mut String,
    namespace: &str,
    data: &Vec<SocketIoData>,
    ack: Option<u64>,
) {
    if namespace != "/" {
        out.push_str(namespace);
        out.push(',');
    }
    if let Some(ack) = ack {
        out.push_str(&ack.to_string());
    }

    let mut data_builder = SocketIoDataSerializer::new();

    for value in data {
        match value {
            SocketIoData::String(value) => {
                data_builder.write_value(value.as_str());
            }
        }
    }

    data_builder.build_into(out);
}

#[cfg(test)]
mod tests {

    use super::SocketIoMessage;
    use crate::SocketIoData;

    #[test]
    fn test_connect_to_default_namespace() {
        let message = SocketIoMessage::Connect {
            namespace: "/".into(),
            sid: None,
        };

        let mut result = String::new();

        message.serialize(&mut result);

        assert_eq!(result, "0");

        let result = SocketIoMessage::deserialize(&result);
        match result {
            SocketIoMessage::Connect { namespace, sid } => {
                assert_eq!(namespace.as_str(), "/");
                assert!(sid.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_connect_to_a_custom_namespace() {
        let message = SocketIoMessage::Connect {
            namespace: "/admin".into(),
            sid: Some("oSO0OpakMV_3jnilAAAA".into()),
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"0/admin,{"sid":"oSO0OpakMV_3jnilAAAA"}"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Connect { namespace, sid } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(sid.unwrap().as_str(), "oSO0OpakMV_3jnilAAAA");
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_connect_error_default_namespace() {
        let message = SocketIoMessage::ConnectError {
            namespace: "/".into(),
            message: "Not authorized".into(),
        };

        let mut result = String::new();

        message.serialize(&mut result);

        println!("{}", result);

        assert_eq!(result, r#"4{"message":"Not authorized"}"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::ConnectError { namespace, message } => {
                assert_eq!(namespace.as_str(), "/");
                assert_eq!(message.as_str(), "Not authorized");
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_default_namespace() {
        let data = vec![SocketIoData::String("foo".into())];

        let message = SocketIoMessage::Event {
            namespace: "/".into(),
            data,
            ack: None,
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"2["foo"]"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Event {
                namespace,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/");
                assert_eq!(data.get(0).unwrap().unwrap_as_str(), "foo");
                assert!(ack.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_custom_namespace() {
        let data = vec![SocketIoData::String("foo".into())];

        let message = SocketIoMessage::Event {
            namespace: "/admin".into(),
            data,
            ack: None,
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"2/admin,["foo"]"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Event {
                namespace,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(data.get(0).unwrap().unwrap_as_str(), "foo");
                assert!(ack.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_default_namespace_with_ack() {
        let data = vec![SocketIoData::String("foo".into())];

        let message = SocketIoMessage::Event {
            namespace: "/".into(),
            data,
            ack: Some(12),
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"212["foo"]"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Event {
                namespace,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/");
                assert_eq!(data.get(0).unwrap().unwrap_as_str(), "foo");
                assert_eq!(ack.unwrap(), 12);
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_ack_with_custom_namespace() {
        let data = vec![SocketIoData::String("bar".into())];

        let message = SocketIoMessage::Ack {
            namespace: "/admin".into(),
            data,
            ack: Some(13),
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"3/admin,13["bar"]"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Ack {
                namespace,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(data.get(0).unwrap().unwrap_as_str(), "bar");
                assert_eq!(ack.unwrap(), 13);
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_disconnect_from_default_namespace() {
        let message = SocketIoMessage::Disconnect {
            namespace: "/".into(),
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"1"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Disconnect { namespace } => {
                assert_eq!(namespace.as_str(), "/");
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_disconnect_from_admin_namespace() {
        let message = SocketIoMessage::Disconnect {
            namespace: "/admin".into(),
        };

        let mut result = String::new();
        message.serialize(&mut result);

        assert_eq!(result, r#"1/admin,"#);

        let result = SocketIoMessage::deserialize(&result);

        match result {
            SocketIoMessage::Disconnect { namespace } => {
                assert_eq!(namespace.as_str(), "/admin");
            }
            _ => panic!("Invalid message"),
        }
    }
}
