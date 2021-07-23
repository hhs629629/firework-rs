use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

pub type Uuid = uuid::Uuid;

impl DataTypeExt for Uuid {
    fn write(&self, w: &mut PacketWriter) {
        w.write(&self.as_u128().to_be_bytes());
    }
}
