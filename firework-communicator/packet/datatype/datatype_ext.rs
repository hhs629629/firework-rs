use crate::packet::PacketWriter;

pub trait DataTypeExt {
    // this is helper for write data-type on packet.
    fn write(&self, w: &mut PacketWriter);
}