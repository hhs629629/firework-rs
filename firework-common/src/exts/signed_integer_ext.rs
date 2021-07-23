pub trait SignedIntegerExt {
    fn right_shift(self, v: u8) -> Self;
}

impl SignedIntegerExt for i8 {
    fn right_shift(self, v: u8) -> Self {
        let s = self as u8;
        (s >> v) as Self
    }
}

impl SignedIntegerExt for i16 {
    fn right_shift(self, v: u8) -> Self {
        let s = self as u16;
        (s >> v) as Self
    }
}

impl SignedIntegerExt for i32 {
    fn right_shift(self, v: u8) -> Self {
        let s = self as u32;
        (s >> v) as Self
    }
}

impl SignedIntegerExt for i64 {
    fn right_shift(self, v: u8) -> Self {
        let s = self as u64;
        (s >> v) as Self
    }
}

#[cfg(test)]
mod test {
    use crate::exts::signed_integer_ext::SignedIntegerExt;

    #[test]
    fn i8_shift_test() {
        let a = 0b1000_1010u8 as i8;
        assert_eq!(a.right_shift(1), 0b100_0101i8);
    }

    #[test]
    fn i64_shift_test() {
        let a = 0b1000_1010_1001_0110u64 as i64;
        assert_eq!(a.right_shift(3), 0b0001_0001_0101_0010i64);
    }
}
