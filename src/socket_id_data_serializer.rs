use rust_extensions::date_time::DateTimeAsMicroseconds;

pub struct SocketIoDataSerializer {
    first_data: bool,
    data: String,
}

impl SocketIoDataSerializer {
    pub fn new() -> Self {
        let mut data = String::new();
        data.push('[');
        Self {
            first_data: true,
            data,
        }
    }

    fn add_delimiter(&mut self) {
        if self.first_data {
            self.first_data = false;
        } else {
            self.data.push(',');
        }
    }

    pub fn write_value(&mut self, value: impl SocketIoDataValue) {
        self.add_delimiter();
        value.serialize(&mut self.data);
    }

    pub fn build_into(&self, out: &mut String) {
        out.push_str(&self.data);
        out.push(']');
    }
}

pub trait SocketIoDataValue {
    fn serialize(&self, out: &mut String);
}

impl SocketIoDataValue for String {
    fn serialize(&self, out: &mut String) {
        serialize_str(out, self.as_str());
    }
}

impl SocketIoDataValue for &'_ str {
    fn serialize(&self, out: &mut String) {
        serialize_str(out, self);
    }
}

impl SocketIoDataValue for &'_ String {
    fn serialize(&self, out: &mut String) {
        serialize_str(out, self.as_str());
    }
}

impl SocketIoDataValue for &'_ u8 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ i8 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ u16 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ i16 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ u32 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ i32 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ u64 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ i64 {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ usize {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for &'_ isize {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_string().as_str());
        out.push_str("\"");
    }
}

impl SocketIoDataValue for DateTimeAsMicroseconds {
    fn serialize(&self, out: &mut String) {
        out.push_str("\"");
        out.push_str(self.to_rfc3339().as_str());
        out.push_str("\"");
    }
}

fn serialize_str(out: &mut String, value: &str) {
    out.push_str("\"");

    for c in value.chars() {
        if c == '"' {
            out.push_str("\\\"");
        } else {
            out.push(c);
        }
    }

    out.push_str("\"");
}
