// src/utils/div.rs
use crate::Fint;
use std::ops::Div;

impl Div for Fint {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Fint::I8(a), Fint::I8(b)) => Fint::new(a / b),
            (Fint::I16(a), Fint::I16(b)) => Fint::new(a / b),
            (Fint::I32(a), Fint::I32(b)) => Fint::new(a / b),
            (Fint::I64(a), Fint::I64(b)) => Fint::new(a / b),
            (Fint::I128(a), Fint::I128(b)) => Fint::new(a / b),
            (Fint::U8(a), Fint::U8(b)) => Fint::new(a / b),
            (Fint::U16(a), Fint::U16(b)) => Fint::new(a / b),
            (Fint::U32(a), Fint::U32(b)) => Fint::new(a / b),
            (Fint::U64(a), Fint::U64(b)) => Fint::new(a / b),
            (Fint::U128(a), Fint::U128(b)) => Fint::new(a / b),
            _ => panic!("Cannot divide different types or divide by zero"),
        }
    }
}

macro_rules! impl_div_for_Fint {
    ($($t:ty),*) => {
        $(
            impl Div<$t> for Fint {
                type Output = Result<Self, &'static str>;

                fn div(self, other: $t) -> Self::Output {
                    if other == 0 {
                        return Err("division by zero");
                    }
                    match self {
                        Fint::I8(a) => Ok(Fint::new(a as i128 / other as i128)),
                        Fint::I16(a) => Ok(Fint::new(a as i128 / other as i128)),
                        Fint::I32(a) => Ok(Fint::new(a as i128 / other as i128)),
                        Fint::I64(a) => Ok(Fint::new(a as i128 / other as i128)),
                        Fint::I128(a) => Ok(Fint::new(a / other as i128)),
                        Fint::U8(a) => Ok(Fint::new(a as u128 / other as u128)),
                        Fint::U16(a) => Ok(Fint::new(a as u128 / other as u128)),
                        Fint::U32(a) => Ok(Fint::new(a as u128 / other as u128)),
                        Fint::U64(a) => Ok(Fint::new(a as u128 / other as u128)),
                        Fint::U128(a) => Ok(Fint::new(a / other as u128)),
                    }
                }
            }
        )*
    };
}

impl_div_for_Fint!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
