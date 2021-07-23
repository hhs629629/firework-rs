use bytes::BytesMut;
use bytes::Bytes;

pub struct PacketWriter {
    buffer: BytesMut
}

impl PacketWriter {
    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn freeze(self) -> Bytes {
        self.buffer.freeze()
    }
}