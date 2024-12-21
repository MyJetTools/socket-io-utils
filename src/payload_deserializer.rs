use core::str;

use my_json::json_reader::{AsJsonSlice, JsonArrayIterator, JsonFirstLineIterator};

pub struct SocketIoPayloadData<'s> {
    pub namespace: &'s str,
    pub data: Option<&'s str>,
    pub ack: Option<i64>,
}

impl<'s> SocketIoPayloadData<'s> {
    pub fn get_field(&self, field_name: &str) -> Option<String> {
        let data = self.data?;

        let first_line_iterator = JsonFirstLineIterator::new(data.as_bytes());

        while let Some(itm) = first_line_iterator.get_next() {
            if itm.is_err() {
                panic!("Can not extract '{field_name}' field from data [{}]", data);
            }
            let (name, value) = itm.unwrap();

            if name.as_str().unwrap().as_str() == field_name {
                return Some(value.as_str().unwrap().to_string());
            }
        }

        None
    }

    pub fn get_event_data(&self) -> Option<(String, String)> {
        let data = self.data?;

        let array_iterator = JsonArrayIterator::new(data.as_slice());

        if array_iterator.is_err() {
            panic!("Can not extract event data from data [{}]", data);
        }

        let array_iterator = array_iterator.unwrap();

        let name = array_iterator.get_next();

        if name.is_none() {
            panic!(
                "No name found during extracting event name from data [{}]",
                data
            );
        }

        let name = name.unwrap();

        if let Err(err) = name {
            panic!(
                "Can not extract event name from data [{}]. Error: {:?}",
                data, err
            );
        }

        let name = name.unwrap();

        let name = name.as_str();

        if name.is_none() {
            panic!(
                "Can not extract event name from data [{}]. Event name must be String",
                data
            );
        }

        let name = name.unwrap().to_string();

        let payload = array_iterator.get_next();

        if payload.is_none() {
            return Some((name, String::new()));
        }

        let payload = payload.unwrap();

        if let Err(err) = payload {
            panic!(
                "Can not extract event payload from data [{}]. Error: {:?}",
                data, err
            );
        }

        let payload = payload.unwrap().as_raw_str().map(|x| x.to_string());

        Some((name, payload.unwrap_or_else(|| String::new())))
    }
}

pub fn deserialize_data(value: &str) -> SocketIoPayloadData {
    let (namespace, _, data) = read_name_space_and_data_position(value);

    if data.is_none() {
        return SocketIoPayloadData {
            namespace,
            data: None,
            ack: None,
        };
    }

    let data = data.unwrap();

    SocketIoPayloadData {
        namespace,
        data: Some(data),
        ack: None,
    }
}

pub fn deserialize_event_data(value: &str) -> SocketIoPayloadData {
    let (namespace, ack, data) = read_name_space_and_data_position(value);

    if data.is_none() {
        return SocketIoPayloadData {
            namespace,
            data: None,
            ack,
        };
    }

    let data = data.unwrap();

    SocketIoPayloadData {
        namespace,
        data: Some(data),
        ack,
    }
}

fn read_name_space_and_data_position(value: &str) -> (&str, Option<i64>, Option<&str>) {
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

fn get_ack_and_data(data: &str) -> (Option<i64>, Option<&str>) {
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
