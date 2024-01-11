#[macro_export]
macro_rules! int_ops {
    ($self:expr, $other:expr, $op:tt) => {
        match ($self, $other) {
            (Int::I8(a), Int::I8(b)) => Int::new(a $op b),
            (Int::I16(a), Int::I16(b)) => Int::new(a $op b),
            (Int::I32(a), Int::I32(b)) => Int::new(a $op b),
            (Int::I64(a), Int::I64(b)) => Int::new(a $op b),
            (Int::I128(a), Int::I128(b)) => Int::new(a $op b),
            (Int::U8(a), Int::U8(b)) => Int::new(a $op b),
            (Int::U16(a), Int::U16(b)) => Int::new(a $op b),
            (Int::U32(a), Int::U32(b)) => Int::new(a $op b),
            (Int::U64(a), Int::U64(b)) => Int::new(a $op b),
            (Int::U128(a), Int::U128(b)) => Int::new(a $op b),
            _ => panic!("Cannot perform operation on different types"),
        }
    };
}
