use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

use bytes::Bytes;

impl DataTypeExt for &[u8] {
    fn write(&self, w: &mut PacketWriter) {
        w.write(self);
    }
}

impl DataTypeExt for Vec<u8> {
    fn write(&self, w: &mut PacketWriter) {
        self.as_slice().write(w);
    }
}

impl DataTypeExt for Bytes {
    fn write(&self, w: &mut PacketWriter) {
        (&self[..]).write(w);
    }
}