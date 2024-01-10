#[cfg(test)]
mod tests {
    use rust_int::main::{Int, IntArg};

    #[test]
    fn test_new() {
        assert_eq!(Int::new(IntArg::from(10i128)), Int::I8(10));
        assert_eq!(Int::new(IntArg::from(300i128)), Int::I16(300));
        assert_eq!(Int::new(IntArg::from(70000i128)), Int::I32(70000));
        assert_eq!(Int::new(IntArg::from(5000000000i128)), Int::I64(5000000000));
        assert_eq!(Int::new(IntArg::from(200u128)), Int::U8(200));
        assert_eq!(Int::new(IntArg::from(60000u128)), Int::U16(60000));
        assert_eq!(Int::new(IntArg::from(4000000000u128)), Int::U64(4000000000)); // Change to U64
        assert_eq!(Int::new(IntArg::from(10000000000000000000u128)), Int::U128(10000000000000000000)); // Change to U128
    }

    #[test]
    fn test_add() {
        assert_eq!(Int::I8(10).add(Int::I8(20)), Int::I8(30));
        assert_eq!(Int::I16(300).add(Int::I16(700)), Int::I16(1000));
        assert_eq!(Int::I32(70000).add(Int::I32(30000)), Int::I32(100000));
        assert_eq!(Int::I64(5000000000).add(Int::I64(5000000000)), Int::I64(10000000000));
        assert_eq!(Int::U8(200).add(Int::U8(55)), Int::U8(255));
        assert_eq!(Int::U16(60000).add(Int::U16(5000)), Int::U16(65000));
        assert_eq!(Int::U64(4000000000).add(Int::U64(500000000)), Int::U64(4500000000)); // Change to U64
        assert_eq!(Int::U128(10000000000000000000).add(Int::U128(1000000000000000000)), Int::U128(11000000000000000000)); // Change to U128
    }
}
