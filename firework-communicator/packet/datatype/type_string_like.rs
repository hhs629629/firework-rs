use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

use crate::packet::datatype::VarInt;

impl DataTypeExt for &str {
    fn write(&self, w: &mut PacketWriter) {
        let size = VarInt::I32(self.len() as i32);
        size.write(w);

        w.write(self.as_bytes());
    }
}

impl DataTypeExt for String {
    fn write(&self, w: &mut PacketWriter) {
        self.as_str().write(w);
    }
}

struct Chat {
    content: String,
}

impl DataTypeExt for Chat {
    fn write(&self, w: &mut PacketWriter) {
        self.content.write(w);
    }
}

struct Identifier(String);

impl DataTypeExt for Identifier {
    fn write(&self, w: &mut PacketWriter) {
        self.0.write(w);
    }
}