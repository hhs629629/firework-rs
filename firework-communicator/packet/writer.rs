use bytes::Bytes;
use bytes::BytesMut;

pub struct PacketWriter {
    buffer: BytesMut,
}

impl PacketWriter {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn freeze(self) -> Bytes {
        self.buffer.freeze()
    }
}
