pub enum IntArg {
    I(i128),
    U(u128),
}

pub trait ToIntArg {
    fn to_int_arg(self) -> IntArg;
}

impl ToIntArg for i8 {
    fn to_int_arg(self) -> IntArg {
        IntArg::I(self as i128)
    }
}

impl ToIntArg for u8 {
    fn to_int_arg(self) -> IntArg {
        IntArg::U(self as u128)
    }
}

impl ToIntArg for i16 {
    fn to_int_arg(self) -> IntArg {
        IntArg::I(self as i128)
    }
}

impl ToIntArg for u16 {
    fn to_int_arg(self) -> IntArg {
        IntArg::U(self as u128)
    }
}

impl ToIntArg for i32 {
    fn to_int_arg(self) -> IntArg {
        IntArg::I(self as i128)
    }
}

impl ToIntArg for u32 {
    fn to_int_arg(self) -> IntArg {
        IntArg::U(self as u128)
    }
}

impl ToIntArg for i64 {
    fn to_int_arg(self) -> IntArg {
        IntArg::I(self as i128)
    }
}

impl ToIntArg for u64 {
    fn to_int_arg(self) -> IntArg {
        IntArg::U(self as u128)
    }
}

impl ToIntArg for i128 {
    fn to_int_arg(self) -> IntArg {
        IntArg::I(self)
    }
}

impl ToIntArg for u128 {
    fn to_int_arg(self) -> IntArg {
        IntArg::U(self)
    }
}






