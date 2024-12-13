use std::time::Duration;

use my_json::json_writer::JsonObjectWriter;

use crate::SocketIoSettings;

pub fn compile_connect_to_namespace(namespace: &str, sid: &str) -> Vec<u8> {
    let mut my_json = JsonObjectWriter::new();

    my_json.write("type", "CONNECT");
    my_json.write("namespace", namespace);

    my_json.write_json_object("data", |json_object| {
        json_object.write("sid", sid);
    });

    my_json.build()
}

pub fn compile_connect_payload(sid: &str) -> Vec<u8> {
    let mut content = Vec::new();
    content.extend_from_slice("40{\"sid\":\"".as_bytes());
    content.extend_from_slice(sid.as_bytes());
    content.extend_from_slice("\"}".as_bytes());

    content
}

pub fn compile_negotiate_response(sid: &str, socket_io_settings: &SocketIoSettings) -> String {
    let mut result = Vec::new();

    result.extend_from_slice("0{\"sid\":\"".as_bytes());

    result.extend_from_slice(sid.as_bytes());

    result.extend_from_slice("\",\"upgrades\":[\"websocket\"],\"pingInterval\":".as_bytes());
    write_duration(&mut result, socket_io_settings.get_ping_interval());

    result.extend_from_slice(",\"pingTimeout\":".as_bytes());

    write_duration(&mut result, socket_io_settings.get_ping_timeout());

    result.extend_from_slice(",\"maxPayload\":".as_bytes());

    result.extend_from_slice(
        socket_io_settings
            .get_max_payload_size()
            .to_string()
            .as_bytes(),
    );

    result.extend_from_slice("}".as_bytes());

    String::from_utf8(result).unwrap()
}

fn write_duration(dest: &mut Vec<u8>, duration: Duration) {
    let millis = duration.as_millis();
    let millis = millis.to_string();
    dest.extend_from_slice(millis.as_bytes());
}
