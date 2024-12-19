use serde::*;

use crate::SocketIoSettings;

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketIoHandshakeOpenModel {
    pub sid: String,
    pub upgrades: Vec<String>,
    #[serde(rename = "pingInterval")]
    pub ping_interval: i32,
    #[serde(rename = "pingTimeout")]
    pub ping_timeout: i32,
    #[serde(rename = "maxPayload")]
    pub max_payload: i32,
}

impl SocketIoHandshakeOpenModel {
    pub fn from_settings(sid: String, socket_io_settings: &SocketIoSettings) -> Self {
        Self {
            sid,
            upgrades: vec![String::from("websocket")],
            ping_interval: socket_io_settings.ping_interval.as_secs() as i32,
            ping_timeout: socket_io_settings.ping_timeout.as_secs() as i32,
            max_payload: socket_io_settings.max_payload_size as i32,
        }
    }
}
