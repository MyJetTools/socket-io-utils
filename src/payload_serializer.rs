#[derive(Debug, Default)]
pub struct SocketIoPayload {
    pub text_frame: String,
    pub binary_frames: Vec<Vec<u8>>,
}

impl SocketIoPayload {
    pub fn new() -> Self {
        SocketIoPayload {
            text_frame: String::new(),
            binary_frames: Vec::new(),
        }
    }
}

pub fn serialize_data(out: &mut SocketIoPayload, namespace: &str, data: Option<(&str, &str)>) {
    if namespace != "/" {
        out.text_frame.push_str(namespace);

        out.text_frame.push(',');
    }

    if let Some(data) = data {
        out.text_frame.push_str("{\"");
        out.text_frame.push_str(data.0);
        out.text_frame.push_str("\":\"");
        out.text_frame.push_str(data.1);
        out.text_frame.push_str("\"}");
    }
}

pub fn serialize_event_data(
    out: &mut SocketIoPayload,
    namespace: &str,
    event_name: &str,
    data: &str,
    ack: Option<i64>,
) {
    if namespace != "/" {
        out.text_frame.push_str(namespace);
        out.text_frame.push(',');
    }
    if let Some(ack) = ack {
        out.text_frame.push_str(&ack.to_string());
    }

    out.text_frame.push_str("[\"");

    out.text_frame.push_str(event_name);
    out.text_frame.push_str("\"");

    if data.len() > 0 {
        out.text_frame.push_str(",");
        out.text_frame.push_str(data);
    }
    out.text_frame.push_str("]");
}
