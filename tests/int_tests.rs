#[cfg(test)]
mod tests {
    use fint::Fint;

    #[test]
    fn test_new() {
        assert_eq!(Fint::new(-10), Fint::I8(-10));
        assert_eq!(Fint::new(-300), Fint::I16(-300));
        assert_eq!(Fint::new(-70000), Fint::I32(-70000));
        assert_eq!(Fint::new(-5000000000i64), Fint::I64(-5000000000));
        assert_eq!(Fint::new(200), Fint::U8(200));
        assert_eq!(Fint::new(60000), Fint::U16(60000));
        assert_eq!(Fint::new(4000000000u64), Fint::U32(4000000000));
        assert_eq!(Fint::new(10000000000000000000u128), Fint::U64(10000000000000000000));
    }

    #[test]
    fn test_from() {
        assert_eq!(Fint::from(-10i8), Fint::I8(-10));
        assert_eq!(Fint::from(-300i16), Fint::I16(-300));
        assert_eq!(Fint::from(-70000i32), Fint::I32(-70000));
        assert_eq!(Fint::from(-5000000000i64), Fint::I64(-5000000000));
        assert_eq!(Fint::from(200u8), Fint::U8(200));
        assert_eq!(Fint::from(60000u16), Fint::U16(60000));
        assert_eq!(Fint::from(4000000000u32), Fint::U32(4000000000));
        assert_eq!(Fint::from(10000000000000000000u128), Fint::U64(10000000000000000000));

        // let a = Fint::new(1);

        // let b = a + 0;
        println!("{:?}", Fint::new(10000000 + 1000000000));
    }
}
