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
    }

        #[test]
    fn test_addition() {
        let a = Fint::new(10);
        let a_clone = a.clone();
        let b = Fint::new(20);
        assert_eq!(a + b, Fint::new(30), "10 + 20 should equal 30");

        let c = Fint::new(-10);
        assert_eq!(a_clone + c, Fint::new(0), "-10 + 10 should equal 0");
    }

    #[test]
    fn test_subtraction() {
        let a = Fint::new(30);
        let a_clone = a.clone();
        let b = Fint::new(20);
        assert_eq!(a - b, Fint::new(10), "30 - 20 should equal 10");

        let c = Fint::new(-10);
        assert_eq!(a_clone - c, Fint::new(40), "30 - (-10) should equal 40");
    }

    #[test]
    fn test_multiplication() {
        let a = Fint::new(5);
        let a_clone = a.clone();
        let b = Fint::new(4);
        assert_eq!(a * b, Fint::new(20), "5 * 4 should equal 20");

        let c = Fint::new(-5);
        assert_eq!(a_clone * c, Fint::new(-25), "5 * (-5) should equal -25");
    }

    #[test]
    fn test_division() {
        let a = Fint::new(20);
        let a_clone = a.clone();
        let b = Fint::new(4);
        assert_eq!(a / b, Fint::new(5), "20 / 4 should equal 5");

        let c = Fint::new(-5);
        assert_eq!(a_clone / c, Fint::new(-4), "20 / (-5) should equal -4");
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_division_by_zero() {
        let a = Fint::new(10);
        let b = Fint::new(0);
        let _ = a / b; // Should panic
    }

}

