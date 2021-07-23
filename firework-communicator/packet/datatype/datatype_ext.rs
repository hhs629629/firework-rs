use crate::packet::PacketWriter;

pub trait DataTypeExt {
    // this is helper for write data-type on packet.
    fn write(&self, w: &mut PacketWriter);
}


impl DataTypeExt for i8 {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}

impl DataTypeExt for i16 {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}

impl DataTypeExt for i32 {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}