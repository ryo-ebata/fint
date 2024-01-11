#[cfg(test)]
mod tests {
    use optimal_int::Int;

    #[test]
    fn test_new() {
        assert_eq!(Int::new(-10), Int::I8(-10));
        assert_eq!(Int::new(-300), Int::I16(-300));
        assert_eq!(Int::new(-70000), Int::I32(-70000));
        assert_eq!(Int::new(-5000000000i64), Int::I64(-5000000000));
        assert_eq!(Int::new(200), Int::U8(200));
        assert_eq!(Int::new(60000), Int::U16(60000));
        assert_eq!(Int::new(4000000000u64), Int::U32(4000000000));
        assert_eq!(Int::new(10000000000000000000u128), Int::U64(10000000000000000000));
    }

    #[test]
    fn test_from() {
        assert_eq!(Int::from(-10i8), Int::I8(-10));
        assert_eq!(Int::from(-300i16), Int::I16(-300));
        assert_eq!(Int::from(-70000i32), Int::I32(-70000));
        assert_eq!(Int::from(-5000000000i64), Int::I64(-5000000000));
        assert_eq!(Int::from(200u8), Int::U8(200));
        assert_eq!(Int::from(60000u16), Int::U16(60000));
        assert_eq!(Int::from(4000000000u32), Int::U32(4000000000));
        assert_eq!(Int::from(10000000000000000000u128), Int::U64(10000000000000000000));

        // let a = Int::new(1);

        // let b = a + 0;
    }
}
