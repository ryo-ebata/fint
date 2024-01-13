use crate::Fint;
use std::ops::{Add, Sub, Mul, Div};

macro_rules! impl_ops_for_Fint {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Fint {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                match (self, other) {
                    (Fint::I8(a), Fint::I8(b)) => Fint::new(a $op b),
                    (Fint::I16(a), Fint::I16(b)) => Fint::new(a $op b),
                    (Fint::I32(a), Fint::I32(b)) => Fint::new(a $op b),
                    (Fint::I64(a), Fint::I64(b)) => Fint::new(a $op b),
                    (Fint::I128(a), Fint::I128(b)) => Fint::new(a $op b),
                    (Fint::U8(a), Fint::U8(b)) => Fint::new(a $op b),
                    (Fint::U16(a), Fint::U16(b)) => Fint::new(a $op b),
                    (Fint::U32(a), Fint::U32(b)) => Fint::new(a $op b),
                    (Fint::U64(a), Fint::U64(b)) => Fint::new(a $op b),
                    (Fint::U128(a), Fint::U128(b)) => Fint::new(a $op b),
                    _ => panic!("Cannot apply operation on different types"),
                }
            }
        }
    };
}

impl_ops_for_Fint!(Add, add, +);
impl_ops_for_Fint!(Sub, sub, -);
impl_ops_for_Fint!(Mul, mul, *);
impl_ops_for_Fint!(Div, div, /);

macro_rules! impl_ops_for_Fint_and_primitive {
    ($trait:ident, $method:ident, $op:tt, $($t:ty),*) => {
        $(
            impl $trait<$t> for Fint {
                type Output = Self;

                fn $method(self, other: $t) -> Self::Output {
                    let other_as_fint = Fint::from(other);
                    match (self, other_as_fint) {
                        (Fint::I8(a), Fint::I8(b)) => Fint::new(a $op b),
                        (Fint::I16(a), Fint::I16(b)) => Fint::new(a $op b),
                        (Fint::I32(a), Fint::I32(b)) => Fint::new(a $op b),
                        (Fint::I64(a), Fint::I64(b)) => Fint::new(a $op b),
                        (Fint::I128(a), Fint::I128(b)) => Fint::new(a $op b),
                        (Fint::U8(a), Fint::U8(b)) => Fint::new(a $op b),
                        (Fint::U16(a), Fint::U16(b)) => Fint::new(a $op b),
                        (Fint::U32(a), Fint::U32(b)) => Fint::new(a $op b),
                        (Fint::U64(a), Fint::U64(b)) => Fint::new(a $op b),
                        (Fint::U128(a), Fint::U128(b)) => Fint::new(a $op b),
                        _ => panic!("Cannot apply operation on different types"),
                    }
                }
            }
        )*
    };
}

impl_ops_for_Fint_and_primitive!(Add, add, +, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl_ops_for_Fint_and_primitive!(Sub, sub, -, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl_ops_for_Fint_and_primitive!(Mul, mul, *, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
impl_ops_for_Fint_and_primitive!(Div, div, /, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);
