use crate::packet::datatype::DataTypeExt;
use crate::packet::PacketWriter;

use firework_common::exts::SignedIntegerExt;

enum VarLong {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
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
        let mut value = match self {
            VarLong::I8(v) => *v as i64,
            VarLong::I16(v) => *v as i64,
            VarLong::I32(v) => *v as i64,
            VarLong::I64(v) => *v as i64,
        };

        let mut data = [0u8; 5];
        let mut i = 0;

        loop {
            let mut currentByte = (value & 0b0111_1111) as u8;
            value = value.right_shift(7);
            if value != 0 {
                currentByte |= 0b1000_0000;
            }

            data[i] = currentByte;
            i += 1;
            if value == 0 {
                break;
            }
        }

        w.write(&data[..i]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn write_test() {
        let mut w = PacketWriter::new();
        let var_long = VarLong::I32(20210723);

        var_long.write(&mut w);

        let value = w.freeze();

        assert_eq!(value[0], 0b1010_0011u8);
        assert_eq!(value[1], 0b1100_1000u8);
        assert_eq!(value[2], 0b1101_0001u8);
        assert_eq!(value[3], 0b0000_1001u8);
    }
}
