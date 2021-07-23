use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

enum VarLong {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64)
}

impl From<i8> for VarLong {
    fn from(v: i8) -> Self {
        Self::I8(v)
    }
}

impl From<i16> for VarLong {
    fn from(v: i16) -> Self {
        Self::I16(v)
    }
}

impl From<i32> for VarLong {
    fn from(v: i32) -> Self {
        Self::I32(v)
    }
}

impl From<i64> for VarLong {
    fn from(v: i64) -> Self {
        Self::I64(v)
    }
}

impl DataTypeExt for VarLong {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}