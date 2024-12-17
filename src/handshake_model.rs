use serde::*;

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
