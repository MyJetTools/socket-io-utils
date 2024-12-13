use my_json::json_reader::JsonFirstLineIterator;

use crate::SocketIoData;

pub fn deserialize_data(value: &str) -> (&str, Option<(String, String)>) {
    let (namespace, _, data) = read_name_space_and_data_position(value);

    if data.is_none() {
        return (namespace, None);
    }

    let data = data.unwrap();

    let json_object_parser = JsonFirstLineIterator::new(data.as_bytes());

    let next = json_object_parser.get_next();

    if next.is_none() {
        return (namespace, None);
    }

    let (name, value) = next.unwrap().unwrap();

    (
        namespace,
        Some((
            name.as_str().unwrap().as_str().to_string(),
            value.as_str().unwrap().as_str().to_string(),
        )),
    )
}

pub fn deserialize_event_data(value: &str) -> (&str, Option<u64>, Vec<SocketIoData>) {
    let (namespace, ack, data) = read_name_space_and_data_position(value);

    if data.is_none() {
        return (namespace, ack, vec![]);
    }

    let data = data.unwrap();

    let data = SocketIoData::parse(data);

    (namespace, ack, data)
}

fn read_name_space_and_data_position(value: &str) -> (&str, Option<u64>, Option<&str>) {
    if value.len() == 0 {
        return ("/", None, None);
    }

    let index = find_end_of_namespace(value);

    let namespace = if index == 0 { "/" } else { &value[..index] };

    if index == value.len() {
        return (namespace, None, None);
    }

    let mut data = &value[index..];

    if data.starts_with(',') {
        data = &data[1..];
    }

    if data.is_empty() {
        return (namespace, None, None);
    }

    let (ack, data) = get_ack_and_data(data);

    (namespace, ack, data)
}

fn get_ack_and_data(data: &str) -> (Option<u64>, Option<&str>) {
    let first_char = data.chars().next().unwrap();

    if first_char.is_digit(10) {
        let index = find_end_of_ack_id(data);
        let ack = &data[..index];
        let data = &data[index..];
        (Some(ack.parse().unwrap()), Some(data))
    } else {
        (None, Some(data))
    }
}

fn find_end_of_ack_id(value: &str) -> usize {
    let mut index = 0;

    for c in value.chars() {
        if !c.is_digit(10) {
            return index;
        }
        index += 1;
    }

    index
}

fn find_end_of_namespace(value: &str) -> usize {
    let mut index = 0;

    for c in value.chars() {
        if c == ',' {
            return index;
        }

        if c == '{' {
            return index;
        }

        if c == '[' {
            return index;
        }

        if c.is_digit(10) {
            return index;
        }

        index += 1;
    }

    index
}
