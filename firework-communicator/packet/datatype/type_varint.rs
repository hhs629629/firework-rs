use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

enum VarInt {
    I8(i8),
    I16(i16),
    I32(i32)
}

impl From<i8> for VarInt {
    fn from(v: i8) -> Self {
        Self::I8(v)
    }
}

impl From<i16> for VarInt {
    fn from(v: i16) -> Self {
        Self::I16(v)
    }
}

impl From<i32> for VarInt {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}

impl DataTypeExt for VarInt {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}