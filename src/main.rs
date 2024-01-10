#[derive(PartialEq, Debug)]
pub enum Int {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

pub enum IntArg {
    I(i128),
    U(u128),
}

impl From<i128> for IntArg {
    fn from(item: i128) -> Self {
        IntArg::I(item)
    }
}

impl From<u128> for IntArg {
    fn from(item: u128) -> Self {
        IntArg::U(item)
    }
}

impl Int {
    pub fn new(value: IntArg) -> Self {
        match value {
            IntArg::I(i) => {
                if i < 0 {
                    match i {
                        v if v >= i8::MIN as i128 && v <= i8::MAX as i128 => Int::I8(v as i8),
                        v if v >= i16::MIN as i128 && v <= i16::MAX as i128 => Int::I16(v as i16),
                        v if v >= i32::MIN as i128 && v <= i32::MAX as i128 => Int::I32(v as i32),
                        v if v >= i64::MIN as i128 && v <= i64::MAX as i128 => Int::I64(v as i64),
                        _ => panic!("Value out of range for Int types"),
                    }
                } else {
                    match i as u128 {
                        v if v <= u8::MAX as u128 => Int::U8(v as u8),
                        v if v <= u16::MAX as u128 => Int::U16(v as u16),
                        v if v <= u32::MAX as u128 => Int::U32(v as u32),
                        v if v <= u64::MAX as u128 => Int::U64(v as u64),
                        v => Int::U128(v),
                    }
                }
            },
            IntArg::U(u) => {
                match u {
                    v if v <= u8::MAX as u128 => Int::U8(v as u8),
                    v if v <= u16::MAX as u128 => Int::U16(v as u16),
                    v if v <= u32::MAX as u128 => Int::U32(v as u32),
                    v if v <= u64::MAX as u128 => Int::U64(v as u64),
                    v => Int::U128(v),
                }
            },
        }
    }

    pub fn add(self, other: Int) -> Int {
        match (self, other) {
            (Int::I8(a), Int::I8(b)) => Int::new(IntArg::I(a as i128 + b as i128)),
            (Int::I16(a), Int::I16(b)) => Int::new(IntArg::I(a as i128 + b as i128)),
            (Int::I32(a), Int::I32(b)) => Int::new(IntArg::I(a as i128 + b as i128)),
            (Int::I64(a), Int::I64(b)) => Int::new(IntArg::I(a as i128 + b as i128)),
            (Int::U8(a), Int::U8(b)) => Int::new(IntArg::U(a as u128 + b as u128)),
            (Int::U16(a), Int::U16(b)) => Int::new(IntArg::U(a as u128 + b as u128)),
            (Int::U32(a), Int::U32(b)) => Int::new(IntArg::U(a as u128 + b as u128)),
            (Int::U64(a), Int::U64(b)) => Int::new(IntArg::U(a as u128 + b as u128)),
            (Int::U128(a), Int::U128(b)) => Int::new(IntArg::U(a as u128 + b as u128)),
            _ => panic!("Mismatched types"),
        }
    }
}

fn main() {
    let a = Int::new(IntArg::from(10i128)); // i8範囲内
    let b = Int::new(IntArg::from(300i128)); // i16範囲内
    let result = a.add(b);

    // 結果の表示
    match result {
        Int::I8(val) => println!("Result is i8: {}", val),
        Int::I16(val) => println!("Result is i16: {}", val),
        Int::I32(val) => println!("Result is i32: {}", val),
        Int::I64(val) => println!("Result is i64: {}", val),
        Int::U8(val) => println!("Result is u8: {}", val),
        Int::U16(val) => println!("Result is u16: {}", val),
        Int::U32(val) => println!("Result is u32: {}", val),
        Int::U64(val) => println!("Result is u64: {}", val),
        Int::U128(val) => println!("Result is u128: {}", val),
    }
}
