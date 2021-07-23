use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

pub type Uuid = uuid::Uuid;

impl DataTypeExt for Uuid {
    fn write(&self, w: &mut PacketWriter) {
        todo!()
    }
}