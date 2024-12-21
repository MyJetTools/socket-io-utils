use rust_extensions::StrOrString;

use crate::{SocketIoContract, SocketIoPayload};

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
        event_name: StrOrString<'static>,
        data: StrOrString<'static>,
        ack: Option<i64>,
    },
    Ack {
        namespace: StrOrString<'static>,
        event_name: StrOrString<'static>,
        data: StrOrString<'static>,
        ack: i64,
    },
    ConnectError {
        namespace: StrOrString<'static>,
        message: StrOrString<'static>,
    },
}

impl Into<SocketIoContract> for SocketIoMessage {
    fn into(self) -> SocketIoContract {
        SocketIoContract::Message(self)
    }
}

impl SocketIoMessage {
    pub fn get_namespace(&self) -> &str {
        match self {
            SocketIoMessage::Connect { namespace, .. } => namespace.as_str(),
            SocketIoMessage::Disconnect { namespace, .. } => namespace.as_str(),
            SocketIoMessage::Event { namespace, .. } => namespace.as_str(),
            SocketIoMessage::Ack { namespace, .. } => namespace.as_str(),
            SocketIoMessage::ConnectError { namespace, .. } => namespace.as_str(),
        }
    }
    pub fn deserialize(value: &str) -> Self {
        let first_char = value.chars().next().unwrap();

        match first_char {
            '0' => {
                let payload_data = super::payload_deserializer::deserialize_data(&value[1..]);

                let sid = payload_data.get_field("sid");

                SocketIoMessage::Connect {
                    namespace: payload_data.namespace.to_string().into(),
                    sid: sid.map(|s| s.to_string().into()),
                }
            }

            '1' => {
                let payload_data = super::payload_deserializer::deserialize_data(&value[1..]);

                SocketIoMessage::Disconnect {
                    namespace: payload_data.namespace.to_string().into(),
                }
            }

            '2' => {
                let payload_data = super::payload_deserializer::deserialize_event_data(&value[1..]);

                let event_data = payload_data.get_event_data();

                if event_data.is_none() {
                    panic!("Event data is missing in Event message");
                }

                let event_data = event_data.unwrap();

                SocketIoMessage::Event {
                    namespace: payload_data.namespace.to_string().into(),
                    event_name: event_data.0.into(),
                    data: event_data.1.into(),
                    ack: payload_data.ack,
                }
            }

            '3' => {
                let payload_data = super::payload_deserializer::deserialize_event_data(&value[1..]);

                if payload_data.ack.is_none() {
                    panic!("Ack number is missing in Ack message");
                }

                let event_data = payload_data.get_event_data();

                if event_data.is_none() {
                    panic!("Event data is missing in Ack message");
                }

                let event_data = event_data.unwrap();

                SocketIoMessage::Ack {
                    namespace: payload_data.namespace.to_string().into(),
                    event_name: event_data.0.into(),
                    data: event_data.1.into(),
                    ack: payload_data.ack.unwrap(),
                }
            }

            '4' => {
                let payload_data = super::payload_deserializer::deserialize_data(&value[1..]);

                let message = payload_data.get_field("message");

                SocketIoMessage::ConnectError {
                    namespace: payload_data.namespace.to_string().into(),
                    message: match message {
                        Some(m) => m.to_string().into(),
                        None => format!(
                            "Unknown (no message found in data) [{}]",
                            payload_data.data.unwrap_or_default()
                        )
                        .into(),
                    },
                }
            }

            _ => {
                panic!("Invalid socket.io message {}", value);
            }
        }
    }

    pub fn serialize(&self, out: &mut SocketIoPayload) {
        match self {
            SocketIoMessage::Connect { namespace, sid } => {
                out.text_frame.push('0');
                super::payload_serializer::serialize_data(
                    out,
                    namespace.as_str(),
                    sid.as_ref().map(|s| ("sid", s.as_str())),
                );
            }
            SocketIoMessage::Disconnect { namespace } => {
                out.text_frame.push('1');
                super::payload_serializer::serialize_data(out, namespace.as_str(), None);
            }
            SocketIoMessage::Event {
                namespace,
                event_name,
                data,
                ack,
            } => {
                out.text_frame.push('2');

                super::payload_serializer::serialize_event_data(
                    out,
                    namespace.as_str(),
                    event_name.as_str(),
                    data.as_str(),
                    ack.clone(),
                );
            }
            SocketIoMessage::Ack {
                namespace,
                event_name,
                data,
                ack,
            } => {
                out.text_frame.push('3');
                super::payload_serializer::serialize_event_data(
                    out,
                    namespace.as_str(),
                    event_name.as_str(),
                    data.as_str(),
                    Some(*ack),
                );
            }
            SocketIoMessage::ConnectError { namespace, message } => {
                out.text_frame.push('4');
                super::payload_serializer::serialize_data(
                    out,
                    namespace.as_str(),
                    Some(("message", message.as_str())),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::SocketIoMessage;
    use crate::SocketIoPayload;

    #[test]
    fn test_connect_to_default_namespace() {
        let message = SocketIoMessage::Connect {
            namespace: "/".into(),
            sid: None,
        };

        let mut result = SocketIoPayload::new();

        message.serialize(&mut result);

        assert_eq!(result.text_frame, "0");

        let result = SocketIoMessage::deserialize(&result.text_frame);
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

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(
            result.text_frame,
            r#"0/admin,{"sid":"oSO0OpakMV_3jnilAAAA"}"#
        );

        let result = SocketIoMessage::deserialize(&result.text_frame);

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

        let mut result = SocketIoPayload::new();

        message.serialize(&mut result);

        println!("{}", result.text_frame);

        assert_eq!(result.text_frame, r#"4{"message":"Not authorized"}"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

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
        let message = SocketIoMessage::Event {
            namespace: "/".into(),
            event_name: "foo".into(),
            data: "".into(),
            ack: None,
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"2["foo"]"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Event {
                namespace,

                event_name,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/");
                assert_eq!(event_name.as_str(), "foo");
                assert_eq!(data.as_str(), "");
                assert!(ack.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_custom_namespace() {
        let message = SocketIoMessage::Event {
            namespace: "/admin".into(),
            event_name: "foo".into(),
            data: "".into(),
            ack: None,
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"2/admin,["foo"]"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Event {
                namespace,
                event_name,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(event_name.as_str(), "foo");
                assert_eq!(data.as_str(), "");
                assert!(ack.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_custom_namespace_with_params() {
        let message = SocketIoMessage::Event {
            namespace: "/admin".into(),
            event_name: "foo".into(),
            data: "{\"type\":\"AccountStatus\",\"accountId\":\"L#711000\"}".into(),
            ack: None,
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(
            result.text_frame,
            r#"2/admin,["foo",{"type":"AccountStatus","accountId":"L#711000"}]"#
        );

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Event {
                namespace,
                event_name,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(event_name.as_str(), "foo");
                assert_eq!(
                    data.as_str(),
                    "{\"type\":\"AccountStatus\",\"accountId\":\"L#711000\"}"
                );
                assert!(ack.is_none());
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_sending_event_to_default_namespace_with_ack() {
        let message = SocketIoMessage::Event {
            namespace: "/".into(),
            event_name: "foo".into(),
            data: "".into(),
            ack: Some(12),
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"212["foo"]"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Event {
                namespace,
                event_name,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/");
                assert_eq!(event_name.as_str(), "foo");
                assert_eq!(data.as_str(), "");
                assert_eq!(ack.unwrap(), 12);
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_ack_with_custom_namespace() {
        let message = SocketIoMessage::Ack {
            namespace: "/admin".into(),
            event_name: "bar".into(),
            data: "".into(),
            ack: 13,
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"3/admin,13["bar"]"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Ack {
                namespace,
                event_name,
                data,
                ack,
            } => {
                assert_eq!(namespace.as_str(), "/admin");
                assert_eq!(event_name.as_str(), "bar");
                assert_eq!(data.as_str(), "");
                assert_eq!(ack, 13);
            }
            _ => panic!("Invalid message"),
        }
    }

    #[test]
    fn test_disconnect_from_default_namespace() {
        let message = SocketIoMessage::Disconnect {
            namespace: "/".into(),
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"1"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

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

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"1/admin,"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Disconnect { namespace } => {
                assert_eq!(namespace.as_str(), "/admin");
            }
            _ => panic!("Invalid message"),
        }
    }

    /*
    #[test]
    fn test_serialization_with_payload() {
        let message = SocketIoMessage::Event {
            namespace: "/".into(),
            data: (),
            ack: (),
        };

        let mut result = SocketIoPayload::new();
        message.serialize(&mut result);

        assert_eq!(result.text_frame, r#"1/admin,"#);

        let result = SocketIoMessage::deserialize(&result.text_frame);

        match result {
            SocketIoMessage::Disconnect { namespace } => {
                assert_eq!(namespace.as_str(), "/admin");
            }
            _ => panic!("Invalid message"),
        }
    }
     */
}
