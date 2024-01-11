mod utils;
use utils::to_int_arg::{ToIntArg, IntArg};

#[derive(PartialEq, Debug)]
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl Int {
    pub fn new<T: ToIntArg>(value: T) -> Self {
        let value = value.to_int_arg();
        match value {
            IntArg::I(i) => {
                if i < 0 {
                    match i {
                        v if v >= i8::MIN as i128 && v <= i8::MAX as i128 => Int::I8(v as i8),
                        v if v >= i16::MIN as i128 && v <= i16::MAX as i128 => Int::I16(v as i16),
                        v if v >= i32::MIN as i128 && v <= i32::MAX as i128 => Int::I32(v as i32),
                        v if v >= i64::MIN as i128 && v <= i64::MAX as i128 => Int::I64(v as i64),
                        _ => panic!("Value out of range for Int types"),
                    }
                } else {
                    match i {
                        v if v <= u8::MAX as i128 => Int::U8(v as u8),
                        v if v <= u16::MAX as i128 => Int::U16(v as u16),
                        v if v <= u32::MAX as i128 => Int::U32(v as u32),
                        v if v <= u64::MAX as i128 => Int::U64(v as u64),
                        v => Int::U128(v as u128),
                    }
                }
            },
            IntArg::U(u) => {
                match u {
                    v if v <= u8::MAX as u128 => Int::U8(v as u8),
                    v if v <= u16::MAX as u128 => Int::U16(v as u16),
                    v if v <= u32::MAX as u128 => Int::U32(v as u32),
                    v if v <= u64::MAX as u128 => Int::U64(v as u64),
                    v => Int::U128(v),
                }
            },
        }
    }
}

