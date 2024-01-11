// src/utils/div.rs
use crate::Int;
use std::ops::Div;

impl Div for Int {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Int::I8(a), Int::I8(b)) => Int::new(a / b),
            (Int::I16(a), Int::I16(b)) => Int::new(a / b),
            (Int::I32(a), Int::I32(b)) => Int::new(a / b),
            (Int::I64(a), Int::I64(b)) => Int::new(a / b),
            (Int::I128(a), Int::I128(b)) => Int::new(a / b),
            (Int::U8(a), Int::U8(b)) => Int::new(a / b),
            (Int::U16(a), Int::U16(b)) => Int::new(a / b),
            (Int::U32(a), Int::U32(b)) => Int::new(a / b),
            (Int::U64(a), Int::U64(b)) => Int::new(a / b),
            (Int::U128(a), Int::U128(b)) => Int::new(a / b),
            _ => panic!("Cannot divide different types or divide by zero"),
        }
    }
}

macro_rules! impl_div_for_int {
    ($($t:ty),*) => {
        $(
            impl Div<$t> for Int {
                type Output = Result<Self, &'static str>;

                fn div(self, other: $t) -> Self::Output {
                    if other == 0 {
                        return Err("division by zero");
                    }
                    match self {
                        Int::I8(a) => Ok(Int::new(a as i128 / other as i128)),
                        Int::I16(a) => Ok(Int::new(a as i128 / other as i128)),
                        Int::I32(a) => Ok(Int::new(a as i128 / other as i128)),
                        Int::I64(a) => Ok(Int::new(a as i128 / other as i128)),
                        Int::I128(a) => Ok(Int::new(a / other as i128)),
                        Int::U8(a) => Ok(Int::new(a as u128 / other as u128)),
                        Int::U16(a) => Ok(Int::new(a as u128 / other as u128)),
                        Int::U32(a) => Ok(Int::new(a as u128 / other as u128)),
                        Int::U64(a) => Ok(Int::new(a as u128 / other as u128)),
                        Int::U128(a) => Ok(Int::new(a / other as u128)),
                    }
                }
            }
        )*
    };
}

impl_div_for_int!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
