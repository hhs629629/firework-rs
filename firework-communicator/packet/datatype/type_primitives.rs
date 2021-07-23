use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

impl DataTypeExt for bool {
    fn write(&self, w: &mut PacketWriter) {
        w.write(if *self { &[1u8] } else { &[0u8] });
    }
}

impl DataTypeExt for i8 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for u8 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for i16 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for u16 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for i32 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for i64 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for f32 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}

impl DataTypeExt for f64 {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.to_be_bytes());
    }
}