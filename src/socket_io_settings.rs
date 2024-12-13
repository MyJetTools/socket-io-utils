use std::time::Duration;

pub struct SocketIoSettings {
    pub ping_interval: Duration,
    pub ping_timeout: Duration,
    pub max_payload_size: usize,
}
