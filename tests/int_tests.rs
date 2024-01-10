#[cfg(test)]
mod tests {
    use rust_int::Int;

    #[test]
    fn test_new() {
        assert_eq!(Int::new(-10), Int::I8(-10));
        assert_eq!(Int::new(-300), Int::I16(-300));
        assert_eq!(Int::new(-70000), Int::I32(-70000));
        assert_eq!(Int::new(-5000000000i64), Int::I64(-5000000000)); // Add type annotation
        assert_eq!(Int::new(200), Int::U8(200));
        assert_eq!(Int::new(60000), Int::U16(60000));
        assert_eq!(Int::new(4000000000u64), Int::U32(4000000000)); // Add type annotation
        assert_eq!(Int::new(10000000000000000000u128), Int::U64(10000000000000000000)); // Add type annotation
    }
}
