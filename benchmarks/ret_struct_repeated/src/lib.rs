#[derive(Debug)]
pub enum NumEnum0 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers0 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_0(a: u32) -> NumEnum0 {
    if a == 1 {
        NumEnum0::A(a)
    } else if a == 2 {
        NumEnum0::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum0::C(a as u8 * 3)
    } else {
        NumEnum0::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_0(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers0 {
    Numbers0 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum1 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers1 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_1(a: u32) -> NumEnum1 {
    if a == 1 {
        NumEnum1::A(a)
    } else if a == 2 {
        NumEnum1::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum1::C(a as u8 * 3)
    } else {
        NumEnum1::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_1(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers1 {
    Numbers1 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum2 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers2 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_2(a: u32) -> NumEnum2 {
    if a == 1 {
        NumEnum2::A(a)
    } else if a == 2 {
        NumEnum2::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum2::C(a as u8 * 3)
    } else {
        NumEnum2::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_2(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers2 {
    Numbers2 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum3 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers3 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_3(a: u32) -> NumEnum3 {
    if a == 1 {
        NumEnum3::A(a)
    } else if a == 2 {
        NumEnum3::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum3::C(a as u8 * 3)
    } else {
        NumEnum3::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_3(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers3 {
    Numbers3 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum4 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers4 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_4(a: u32) -> NumEnum4 {
    if a == 1 {
        NumEnum4::A(a)
    } else if a == 2 {
        NumEnum4::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum4::C(a as u8 * 3)
    } else {
        NumEnum4::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_4(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers4 {
    Numbers4 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum5 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers5 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_5(a: u32) -> NumEnum5 {
    if a == 1 {
        NumEnum5::A(a)
    } else if a == 2 {
        NumEnum5::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum5::C(a as u8 * 3)
    } else {
        NumEnum5::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_5(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers5 {
    Numbers5 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum6 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers6 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_6(a: u32) -> NumEnum6 {
    if a == 1 {
        NumEnum6::A(a)
    } else if a == 2 {
        NumEnum6::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum6::C(a as u8 * 3)
    } else {
        NumEnum6::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_6(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers6 {
    Numbers6 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum7 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers7 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_7(a: u32) -> NumEnum7 {
    if a == 1 {
        NumEnum7::A(a)
    } else if a == 2 {
        NumEnum7::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum7::C(a as u8 * 3)
    } else {
        NumEnum7::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_7(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers7 {
    Numbers7 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum8 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers8 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_8(a: u32) -> NumEnum8 {
    if a == 1 {
        NumEnum8::A(a)
    } else if a == 2 {
        NumEnum8::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum8::C(a as u8 * 3)
    } else {
        NumEnum8::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_8(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers8 {
    Numbers8 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum9 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers9 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_9(a: u32) -> NumEnum9 {
    if a == 1 {
        NumEnum9::A(a)
    } else if a == 2 {
        NumEnum9::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum9::C(a as u8 * 3)
    } else {
        NumEnum9::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_9(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers9 {
    Numbers9 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum10 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers10 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_10(a: u32) -> NumEnum10 {
    if a == 1 {
        NumEnum10::A(a)
    } else if a == 2 {
        NumEnum10::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum10::C(a as u8 * 3)
    } else {
        NumEnum10::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_10(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers10 {
    Numbers10 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum11 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers11 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_11(a: u32) -> NumEnum11 {
    if a == 1 {
        NumEnum11::A(a)
    } else if a == 2 {
        NumEnum11::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum11::C(a as u8 * 3)
    } else {
        NumEnum11::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_11(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers11 {
    Numbers11 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum12 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers12 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_12(a: u32) -> NumEnum12 {
    if a == 1 {
        NumEnum12::A(a)
    } else if a == 2 {
        NumEnum12::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum12::C(a as u8 * 3)
    } else {
        NumEnum12::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_12(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers12 {
    Numbers12 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum13 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers13 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_13(a: u32) -> NumEnum13 {
    if a == 1 {
        NumEnum13::A(a)
    } else if a == 2 {
        NumEnum13::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum13::C(a as u8 * 3)
    } else {
        NumEnum13::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_13(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers13 {
    Numbers13 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum14 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers14 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_14(a: u32) -> NumEnum14 {
    if a == 1 {
        NumEnum14::A(a)
    } else if a == 2 {
        NumEnum14::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum14::C(a as u8 * 3)
    } else {
        NumEnum14::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_14(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers14 {
    Numbers14 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum15 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers15 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_15(a: u32) -> NumEnum15 {
    if a == 1 {
        NumEnum15::A(a)
    } else if a == 2 {
        NumEnum15::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum15::C(a as u8 * 3)
    } else {
        NumEnum15::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_15(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers15 {
    Numbers15 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum16 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers16 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_16(a: u32) -> NumEnum16 {
    if a == 1 {
        NumEnum16::A(a)
    } else if a == 2 {
        NumEnum16::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum16::C(a as u8 * 3)
    } else {
        NumEnum16::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_16(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers16 {
    Numbers16 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum17 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers17 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_17(a: u32) -> NumEnum17 {
    if a == 1 {
        NumEnum17::A(a)
    } else if a == 2 {
        NumEnum17::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum17::C(a as u8 * 3)
    } else {
        NumEnum17::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_17(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers17 {
    Numbers17 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum18 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers18 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_18(a: u32) -> NumEnum18 {
    if a == 1 {
        NumEnum18::A(a)
    } else if a == 2 {
        NumEnum18::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum18::C(a as u8 * 3)
    } else {
        NumEnum18::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_18(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers18 {
    Numbers18 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum19 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers19 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_19(a: u32) -> NumEnum19 {
    if a == 1 {
        NumEnum19::A(a)
    } else if a == 2 {
        NumEnum19::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum19::C(a as u8 * 3)
    } else {
        NumEnum19::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_19(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers19 {
    Numbers19 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum20 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers20 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_20(a: u32) -> NumEnum20 {
    if a == 1 {
        NumEnum20::A(a)
    } else if a == 2 {
        NumEnum20::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum20::C(a as u8 * 3)
    } else {
        NumEnum20::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_20(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers20 {
    Numbers20 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum21 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers21 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_21(a: u32) -> NumEnum21 {
    if a == 1 {
        NumEnum21::A(a)
    } else if a == 2 {
        NumEnum21::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum21::C(a as u8 * 3)
    } else {
        NumEnum21::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_21(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers21 {
    Numbers21 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum22 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers22 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_22(a: u32) -> NumEnum22 {
    if a == 1 {
        NumEnum22::A(a)
    } else if a == 2 {
        NumEnum22::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum22::C(a as u8 * 3)
    } else {
        NumEnum22::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_22(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers22 {
    Numbers22 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum23 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers23 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_23(a: u32) -> NumEnum23 {
    if a == 1 {
        NumEnum23::A(a)
    } else if a == 2 {
        NumEnum23::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum23::C(a as u8 * 3)
    } else {
        NumEnum23::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_23(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers23 {
    Numbers23 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum24 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers24 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_24(a: u32) -> NumEnum24 {
    if a == 1 {
        NumEnum24::A(a)
    } else if a == 2 {
        NumEnum24::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum24::C(a as u8 * 3)
    } else {
        NumEnum24::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_24(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers24 {
    Numbers24 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum25 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers25 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_25(a: u32) -> NumEnum25 {
    if a == 1 {
        NumEnum25::A(a)
    } else if a == 2 {
        NumEnum25::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum25::C(a as u8 * 3)
    } else {
        NumEnum25::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_25(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers25 {
    Numbers25 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum26 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers26 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_26(a: u32) -> NumEnum26 {
    if a == 1 {
        NumEnum26::A(a)
    } else if a == 2 {
        NumEnum26::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum26::C(a as u8 * 3)
    } else {
        NumEnum26::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_26(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers26 {
    Numbers26 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum27 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers27 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_27(a: u32) -> NumEnum27 {
    if a == 1 {
        NumEnum27::A(a)
    } else if a == 2 {
        NumEnum27::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum27::C(a as u8 * 3)
    } else {
        NumEnum27::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_27(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers27 {
    Numbers27 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum28 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers28 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_28(a: u32) -> NumEnum28 {
    if a == 1 {
        NumEnum28::A(a)
    } else if a == 2 {
        NumEnum28::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum28::C(a as u8 * 3)
    } else {
        NumEnum28::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_28(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers28 {
    Numbers28 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum29 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers29 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_29(a: u32) -> NumEnum29 {
    if a == 1 {
        NumEnum29::A(a)
    } else if a == 2 {
        NumEnum29::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum29::C(a as u8 * 3)
    } else {
        NumEnum29::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_29(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers29 {
    Numbers29 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum30 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers30 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_30(a: u32) -> NumEnum30 {
    if a == 1 {
        NumEnum30::A(a)
    } else if a == 2 {
        NumEnum30::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum30::C(a as u8 * 3)
    } else {
        NumEnum30::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_30(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers30 {
    Numbers30 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum31 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers31 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_31(a: u32) -> NumEnum31 {
    if a == 1 {
        NumEnum31::A(a)
    } else if a == 2 {
        NumEnum31::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum31::C(a as u8 * 3)
    } else {
        NumEnum31::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_31(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers31 {
    Numbers31 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum32 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers32 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_32(a: u32) -> NumEnum32 {
    if a == 1 {
        NumEnum32::A(a)
    } else if a == 2 {
        NumEnum32::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum32::C(a as u8 * 3)
    } else {
        NumEnum32::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_32(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers32 {
    Numbers32 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum33 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers33 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_33(a: u32) -> NumEnum33 {
    if a == 1 {
        NumEnum33::A(a)
    } else if a == 2 {
        NumEnum33::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum33::C(a as u8 * 3)
    } else {
        NumEnum33::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_33(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers33 {
    Numbers33 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum34 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers34 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_34(a: u32) -> NumEnum34 {
    if a == 1 {
        NumEnum34::A(a)
    } else if a == 2 {
        NumEnum34::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum34::C(a as u8 * 3)
    } else {
        NumEnum34::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_34(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers34 {
    Numbers34 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum35 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers35 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_35(a: u32) -> NumEnum35 {
    if a == 1 {
        NumEnum35::A(a)
    } else if a == 2 {
        NumEnum35::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum35::C(a as u8 * 3)
    } else {
        NumEnum35::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_35(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers35 {
    Numbers35 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum36 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers36 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_36(a: u32) -> NumEnum36 {
    if a == 1 {
        NumEnum36::A(a)
    } else if a == 2 {
        NumEnum36::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum36::C(a as u8 * 3)
    } else {
        NumEnum36::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_36(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers36 {
    Numbers36 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum37 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers37 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_37(a: u32) -> NumEnum37 {
    if a == 1 {
        NumEnum37::A(a)
    } else if a == 2 {
        NumEnum37::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum37::C(a as u8 * 3)
    } else {
        NumEnum37::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_37(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers37 {
    Numbers37 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum38 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers38 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_38(a: u32) -> NumEnum38 {
    if a == 1 {
        NumEnum38::A(a)
    } else if a == 2 {
        NumEnum38::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum38::C(a as u8 * 3)
    } else {
        NumEnum38::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_38(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers38 {
    Numbers38 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum39 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers39 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_39(a: u32) -> NumEnum39 {
    if a == 1 {
        NumEnum39::A(a)
    } else if a == 2 {
        NumEnum39::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum39::C(a as u8 * 3)
    } else {
        NumEnum39::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_39(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers39 {
    Numbers39 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum40 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers40 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_40(a: u32) -> NumEnum40 {
    if a == 1 {
        NumEnum40::A(a)
    } else if a == 2 {
        NumEnum40::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum40::C(a as u8 * 3)
    } else {
        NumEnum40::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_40(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers40 {
    Numbers40 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum41 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers41 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_41(a: u32) -> NumEnum41 {
    if a == 1 {
        NumEnum41::A(a)
    } else if a == 2 {
        NumEnum41::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum41::C(a as u8 * 3)
    } else {
        NumEnum41::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_41(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers41 {
    Numbers41 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum42 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers42 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_42(a: u32) -> NumEnum42 {
    if a == 1 {
        NumEnum42::A(a)
    } else if a == 2 {
        NumEnum42::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum42::C(a as u8 * 3)
    } else {
        NumEnum42::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_42(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers42 {
    Numbers42 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum43 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers43 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_43(a: u32) -> NumEnum43 {
    if a == 1 {
        NumEnum43::A(a)
    } else if a == 2 {
        NumEnum43::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum43::C(a as u8 * 3)
    } else {
        NumEnum43::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_43(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers43 {
    Numbers43 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum44 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers44 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_44(a: u32) -> NumEnum44 {
    if a == 1 {
        NumEnum44::A(a)
    } else if a == 2 {
        NumEnum44::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum44::C(a as u8 * 3)
    } else {
        NumEnum44::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_44(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers44 {
    Numbers44 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum45 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers45 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_45(a: u32) -> NumEnum45 {
    if a == 1 {
        NumEnum45::A(a)
    } else if a == 2 {
        NumEnum45::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum45::C(a as u8 * 3)
    } else {
        NumEnum45::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_45(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers45 {
    Numbers45 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum46 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers46 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_46(a: u32) -> NumEnum46 {
    if a == 1 {
        NumEnum46::A(a)
    } else if a == 2 {
        NumEnum46::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum46::C(a as u8 * 3)
    } else {
        NumEnum46::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_46(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers46 {
    Numbers46 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum47 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers47 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_47(a: u32) -> NumEnum47 {
    if a == 1 {
        NumEnum47::A(a)
    } else if a == 2 {
        NumEnum47::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum47::C(a as u8 * 3)
    } else {
        NumEnum47::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_47(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers47 {
    Numbers47 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum48 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers48 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_48(a: u32) -> NumEnum48 {
    if a == 1 {
        NumEnum48::A(a)
    } else if a == 2 {
        NumEnum48::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum48::C(a as u8 * 3)
    } else {
        NumEnum48::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_48(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers48 {
    Numbers48 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum49 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers49 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_49(a: u32) -> NumEnum49 {
    if a == 1 {
        NumEnum49::A(a)
    } else if a == 2 {
        NumEnum49::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum49::C(a as u8 * 3)
    } else {
        NumEnum49::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_49(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers49 {
    Numbers49 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum50 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers50 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_50(a: u32) -> NumEnum50 {
    if a == 1 {
        NumEnum50::A(a)
    } else if a == 2 {
        NumEnum50::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum50::C(a as u8 * 3)
    } else {
        NumEnum50::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_50(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers50 {
    Numbers50 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum51 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers51 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_51(a: u32) -> NumEnum51 {
    if a == 1 {
        NumEnum51::A(a)
    } else if a == 2 {
        NumEnum51::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum51::C(a as u8 * 3)
    } else {
        NumEnum51::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_51(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers51 {
    Numbers51 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum52 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers52 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_52(a: u32) -> NumEnum52 {
    if a == 1 {
        NumEnum52::A(a)
    } else if a == 2 {
        NumEnum52::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum52::C(a as u8 * 3)
    } else {
        NumEnum52::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_52(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers52 {
    Numbers52 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum53 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers53 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_53(a: u32) -> NumEnum53 {
    if a == 1 {
        NumEnum53::A(a)
    } else if a == 2 {
        NumEnum53::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum53::C(a as u8 * 3)
    } else {
        NumEnum53::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_53(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers53 {
    Numbers53 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum54 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers54 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_54(a: u32) -> NumEnum54 {
    if a == 1 {
        NumEnum54::A(a)
    } else if a == 2 {
        NumEnum54::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum54::C(a as u8 * 3)
    } else {
        NumEnum54::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_54(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers54 {
    Numbers54 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum55 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers55 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_55(a: u32) -> NumEnum55 {
    if a == 1 {
        NumEnum55::A(a)
    } else if a == 2 {
        NumEnum55::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum55::C(a as u8 * 3)
    } else {
        NumEnum55::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_55(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers55 {
    Numbers55 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum56 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers56 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_56(a: u32) -> NumEnum56 {
    if a == 1 {
        NumEnum56::A(a)
    } else if a == 2 {
        NumEnum56::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum56::C(a as u8 * 3)
    } else {
        NumEnum56::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_56(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers56 {
    Numbers56 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum57 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers57 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_57(a: u32) -> NumEnum57 {
    if a == 1 {
        NumEnum57::A(a)
    } else if a == 2 {
        NumEnum57::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum57::C(a as u8 * 3)
    } else {
        NumEnum57::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_57(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers57 {
    Numbers57 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum58 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers58 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_58(a: u32) -> NumEnum58 {
    if a == 1 {
        NumEnum58::A(a)
    } else if a == 2 {
        NumEnum58::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum58::C(a as u8 * 3)
    } else {
        NumEnum58::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_58(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers58 {
    Numbers58 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum59 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers59 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_59(a: u32) -> NumEnum59 {
    if a == 1 {
        NumEnum59::A(a)
    } else if a == 2 {
        NumEnum59::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum59::C(a as u8 * 3)
    } else {
        NumEnum59::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_59(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers59 {
    Numbers59 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum60 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers60 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_60(a: u32) -> NumEnum60 {
    if a == 1 {
        NumEnum60::A(a)
    } else if a == 2 {
        NumEnum60::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum60::C(a as u8 * 3)
    } else {
        NumEnum60::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_60(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers60 {
    Numbers60 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum61 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers61 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_61(a: u32) -> NumEnum61 {
    if a == 1 {
        NumEnum61::A(a)
    } else if a == 2 {
        NumEnum61::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum61::C(a as u8 * 3)
    } else {
        NumEnum61::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_61(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers61 {
    Numbers61 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum62 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers62 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_62(a: u32) -> NumEnum62 {
    if a == 1 {
        NumEnum62::A(a)
    } else if a == 2 {
        NumEnum62::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum62::C(a as u8 * 3)
    } else {
        NumEnum62::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_62(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers62 {
    Numbers62 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum63 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers63 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_63(a: u32) -> NumEnum63 {
    if a == 1 {
        NumEnum63::A(a)
    } else if a == 2 {
        NumEnum63::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum63::C(a as u8 * 3)
    } else {
        NumEnum63::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_63(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers63 {
    Numbers63 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum64 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers64 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_64(a: u32) -> NumEnum64 {
    if a == 1 {
        NumEnum64::A(a)
    } else if a == 2 {
        NumEnum64::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum64::C(a as u8 * 3)
    } else {
        NumEnum64::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_64(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers64 {
    Numbers64 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum65 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers65 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_65(a: u32) -> NumEnum65 {
    if a == 1 {
        NumEnum65::A(a)
    } else if a == 2 {
        NumEnum65::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum65::C(a as u8 * 3)
    } else {
        NumEnum65::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_65(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers65 {
    Numbers65 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum66 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers66 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_66(a: u32) -> NumEnum66 {
    if a == 1 {
        NumEnum66::A(a)
    } else if a == 2 {
        NumEnum66::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum66::C(a as u8 * 3)
    } else {
        NumEnum66::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_66(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers66 {
    Numbers66 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum67 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers67 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_67(a: u32) -> NumEnum67 {
    if a == 1 {
        NumEnum67::A(a)
    } else if a == 2 {
        NumEnum67::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum67::C(a as u8 * 3)
    } else {
        NumEnum67::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_67(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers67 {
    Numbers67 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum68 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers68 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_68(a: u32) -> NumEnum68 {
    if a == 1 {
        NumEnum68::A(a)
    } else if a == 2 {
        NumEnum68::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum68::C(a as u8 * 3)
    } else {
        NumEnum68::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_68(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers68 {
    Numbers68 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum69 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers69 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_69(a: u32) -> NumEnum69 {
    if a == 1 {
        NumEnum69::A(a)
    } else if a == 2 {
        NumEnum69::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum69::C(a as u8 * 3)
    } else {
        NumEnum69::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_69(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers69 {
    Numbers69 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum70 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers70 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_70(a: u32) -> NumEnum70 {
    if a == 1 {
        NumEnum70::A(a)
    } else if a == 2 {
        NumEnum70::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum70::C(a as u8 * 3)
    } else {
        NumEnum70::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_70(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers70 {
    Numbers70 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum71 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers71 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_71(a: u32) -> NumEnum71 {
    if a == 1 {
        NumEnum71::A(a)
    } else if a == 2 {
        NumEnum71::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum71::C(a as u8 * 3)
    } else {
        NumEnum71::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_71(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers71 {
    Numbers71 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum72 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers72 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_72(a: u32) -> NumEnum72 {
    if a == 1 {
        NumEnum72::A(a)
    } else if a == 2 {
        NumEnum72::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum72::C(a as u8 * 3)
    } else {
        NumEnum72::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_72(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers72 {
    Numbers72 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum73 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers73 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_73(a: u32) -> NumEnum73 {
    if a == 1 {
        NumEnum73::A(a)
    } else if a == 2 {
        NumEnum73::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum73::C(a as u8 * 3)
    } else {
        NumEnum73::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_73(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers73 {
    Numbers73 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum74 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers74 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_74(a: u32) -> NumEnum74 {
    if a == 1 {
        NumEnum74::A(a)
    } else if a == 2 {
        NumEnum74::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum74::C(a as u8 * 3)
    } else {
        NumEnum74::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_74(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers74 {
    Numbers74 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum75 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers75 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_75(a: u32) -> NumEnum75 {
    if a == 1 {
        NumEnum75::A(a)
    } else if a == 2 {
        NumEnum75::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum75::C(a as u8 * 3)
    } else {
        NumEnum75::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_75(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers75 {
    Numbers75 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum76 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers76 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_76(a: u32) -> NumEnum76 {
    if a == 1 {
        NumEnum76::A(a)
    } else if a == 2 {
        NumEnum76::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum76::C(a as u8 * 3)
    } else {
        NumEnum76::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_76(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers76 {
    Numbers76 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum77 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers77 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_77(a: u32) -> NumEnum77 {
    if a == 1 {
        NumEnum77::A(a)
    } else if a == 2 {
        NumEnum77::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum77::C(a as u8 * 3)
    } else {
        NumEnum77::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_77(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers77 {
    Numbers77 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum78 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers78 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_78(a: u32) -> NumEnum78 {
    if a == 1 {
        NumEnum78::A(a)
    } else if a == 2 {
        NumEnum78::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum78::C(a as u8 * 3)
    } else {
        NumEnum78::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_78(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers78 {
    Numbers78 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum79 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers79 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_79(a: u32) -> NumEnum79 {
    if a == 1 {
        NumEnum79::A(a)
    } else if a == 2 {
        NumEnum79::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum79::C(a as u8 * 3)
    } else {
        NumEnum79::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_79(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers79 {
    Numbers79 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum80 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers80 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_80(a: u32) -> NumEnum80 {
    if a == 1 {
        NumEnum80::A(a)
    } else if a == 2 {
        NumEnum80::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum80::C(a as u8 * 3)
    } else {
        NumEnum80::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_80(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers80 {
    Numbers80 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum81 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers81 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_81(a: u32) -> NumEnum81 {
    if a == 1 {
        NumEnum81::A(a)
    } else if a == 2 {
        NumEnum81::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum81::C(a as u8 * 3)
    } else {
        NumEnum81::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_81(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers81 {
    Numbers81 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum82 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers82 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_82(a: u32) -> NumEnum82 {
    if a == 1 {
        NumEnum82::A(a)
    } else if a == 2 {
        NumEnum82::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum82::C(a as u8 * 3)
    } else {
        NumEnum82::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_82(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers82 {
    Numbers82 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum83 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers83 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_83(a: u32) -> NumEnum83 {
    if a == 1 {
        NumEnum83::A(a)
    } else if a == 2 {
        NumEnum83::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum83::C(a as u8 * 3)
    } else {
        NumEnum83::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_83(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers83 {
    Numbers83 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum84 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers84 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_84(a: u32) -> NumEnum84 {
    if a == 1 {
        NumEnum84::A(a)
    } else if a == 2 {
        NumEnum84::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum84::C(a as u8 * 3)
    } else {
        NumEnum84::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_84(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers84 {
    Numbers84 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum85 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers85 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_85(a: u32) -> NumEnum85 {
    if a == 1 {
        NumEnum85::A(a)
    } else if a == 2 {
        NumEnum85::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum85::C(a as u8 * 3)
    } else {
        NumEnum85::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_85(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers85 {
    Numbers85 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum86 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers86 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_86(a: u32) -> NumEnum86 {
    if a == 1 {
        NumEnum86::A(a)
    } else if a == 2 {
        NumEnum86::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum86::C(a as u8 * 3)
    } else {
        NumEnum86::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_86(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers86 {
    Numbers86 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum87 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers87 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_87(a: u32) -> NumEnum87 {
    if a == 1 {
        NumEnum87::A(a)
    } else if a == 2 {
        NumEnum87::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum87::C(a as u8 * 3)
    } else {
        NumEnum87::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_87(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers87 {
    Numbers87 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum88 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers88 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_88(a: u32) -> NumEnum88 {
    if a == 1 {
        NumEnum88::A(a)
    } else if a == 2 {
        NumEnum88::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum88::C(a as u8 * 3)
    } else {
        NumEnum88::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_88(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers88 {
    Numbers88 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum89 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers89 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_89(a: u32) -> NumEnum89 {
    if a == 1 {
        NumEnum89::A(a)
    } else if a == 2 {
        NumEnum89::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum89::C(a as u8 * 3)
    } else {
        NumEnum89::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_89(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers89 {
    Numbers89 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum90 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers90 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_90(a: u32) -> NumEnum90 {
    if a == 1 {
        NumEnum90::A(a)
    } else if a == 2 {
        NumEnum90::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum90::C(a as u8 * 3)
    } else {
        NumEnum90::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_90(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers90 {
    Numbers90 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum91 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers91 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_91(a: u32) -> NumEnum91 {
    if a == 1 {
        NumEnum91::A(a)
    } else if a == 2 {
        NumEnum91::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum91::C(a as u8 * 3)
    } else {
        NumEnum91::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_91(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers91 {
    Numbers91 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum92 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers92 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_92(a: u32) -> NumEnum92 {
    if a == 1 {
        NumEnum92::A(a)
    } else if a == 2 {
        NumEnum92::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum92::C(a as u8 * 3)
    } else {
        NumEnum92::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_92(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers92 {
    Numbers92 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum93 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers93 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_93(a: u32) -> NumEnum93 {
    if a == 1 {
        NumEnum93::A(a)
    } else if a == 2 {
        NumEnum93::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum93::C(a as u8 * 3)
    } else {
        NumEnum93::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_93(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers93 {
    Numbers93 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum94 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers94 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_94(a: u32) -> NumEnum94 {
    if a == 1 {
        NumEnum94::A(a)
    } else if a == 2 {
        NumEnum94::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum94::C(a as u8 * 3)
    } else {
        NumEnum94::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_94(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers94 {
    Numbers94 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum95 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers95 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_95(a: u32) -> NumEnum95 {
    if a == 1 {
        NumEnum95::A(a)
    } else if a == 2 {
        NumEnum95::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum95::C(a as u8 * 3)
    } else {
        NumEnum95::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_95(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers95 {
    Numbers95 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum96 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers96 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_96(a: u32) -> NumEnum96 {
    if a == 1 {
        NumEnum96::A(a)
    } else if a == 2 {
        NumEnum96::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum96::C(a as u8 * 3)
    } else {
        NumEnum96::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_96(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers96 {
    Numbers96 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum97 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers97 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_97(a: u32) -> NumEnum97 {
    if a == 1 {
        NumEnum97::A(a)
    } else if a == 2 {
        NumEnum97::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum97::C(a as u8 * 3)
    } else {
        NumEnum97::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_97(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers97 {
    Numbers97 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum98 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers98 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_98(a: u32) -> NumEnum98 {
    if a == 1 {
        NumEnum98::A(a)
    } else if a == 2 {
        NumEnum98::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum98::C(a as u8 * 3)
    } else {
        NumEnum98::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_98(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers98 {
    Numbers98 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum99 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers99 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_99(a: u32) -> NumEnum99 {
    if a == 1 {
        NumEnum99::A(a)
    } else if a == 2 {
        NumEnum99::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum99::C(a as u8 * 3)
    } else {
        NumEnum99::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_99(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers99 {
    Numbers99 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum100 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers100 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_100(a: u32) -> NumEnum100 {
    if a == 1 {
        NumEnum100::A(a)
    } else if a == 2 {
        NumEnum100::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum100::C(a as u8 * 3)
    } else {
        NumEnum100::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_100(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers100 {
    Numbers100 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum101 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers101 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_101(a: u32) -> NumEnum101 {
    if a == 1 {
        NumEnum101::A(a)
    } else if a == 2 {
        NumEnum101::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum101::C(a as u8 * 3)
    } else {
        NumEnum101::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_101(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers101 {
    Numbers101 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum102 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers102 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_102(a: u32) -> NumEnum102 {
    if a == 1 {
        NumEnum102::A(a)
    } else if a == 2 {
        NumEnum102::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum102::C(a as u8 * 3)
    } else {
        NumEnum102::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_102(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers102 {
    Numbers102 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum103 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers103 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_103(a: u32) -> NumEnum103 {
    if a == 1 {
        NumEnum103::A(a)
    } else if a == 2 {
        NumEnum103::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum103::C(a as u8 * 3)
    } else {
        NumEnum103::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_103(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers103 {
    Numbers103 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum104 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers104 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_104(a: u32) -> NumEnum104 {
    if a == 1 {
        NumEnum104::A(a)
    } else if a == 2 {
        NumEnum104::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum104::C(a as u8 * 3)
    } else {
        NumEnum104::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_104(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers104 {
    Numbers104 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum105 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers105 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_105(a: u32) -> NumEnum105 {
    if a == 1 {
        NumEnum105::A(a)
    } else if a == 2 {
        NumEnum105::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum105::C(a as u8 * 3)
    } else {
        NumEnum105::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_105(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers105 {
    Numbers105 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum106 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers106 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_106(a: u32) -> NumEnum106 {
    if a == 1 {
        NumEnum106::A(a)
    } else if a == 2 {
        NumEnum106::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum106::C(a as u8 * 3)
    } else {
        NumEnum106::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_106(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers106 {
    Numbers106 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum107 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers107 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_107(a: u32) -> NumEnum107 {
    if a == 1 {
        NumEnum107::A(a)
    } else if a == 2 {
        NumEnum107::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum107::C(a as u8 * 3)
    } else {
        NumEnum107::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_107(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers107 {
    Numbers107 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum108 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers108 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_108(a: u32) -> NumEnum108 {
    if a == 1 {
        NumEnum108::A(a)
    } else if a == 2 {
        NumEnum108::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum108::C(a as u8 * 3)
    } else {
        NumEnum108::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_108(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers108 {
    Numbers108 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum109 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers109 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_109(a: u32) -> NumEnum109 {
    if a == 1 {
        NumEnum109::A(a)
    } else if a == 2 {
        NumEnum109::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum109::C(a as u8 * 3)
    } else {
        NumEnum109::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_109(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers109 {
    Numbers109 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum110 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers110 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_110(a: u32) -> NumEnum110 {
    if a == 1 {
        NumEnum110::A(a)
    } else if a == 2 {
        NumEnum110::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum110::C(a as u8 * 3)
    } else {
        NumEnum110::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_110(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers110 {
    Numbers110 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum111 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers111 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_111(a: u32) -> NumEnum111 {
    if a == 1 {
        NumEnum111::A(a)
    } else if a == 2 {
        NumEnum111::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum111::C(a as u8 * 3)
    } else {
        NumEnum111::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_111(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers111 {
    Numbers111 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum112 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers112 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_112(a: u32) -> NumEnum112 {
    if a == 1 {
        NumEnum112::A(a)
    } else if a == 2 {
        NumEnum112::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum112::C(a as u8 * 3)
    } else {
        NumEnum112::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_112(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers112 {
    Numbers112 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum113 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers113 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_113(a: u32) -> NumEnum113 {
    if a == 1 {
        NumEnum113::A(a)
    } else if a == 2 {
        NumEnum113::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum113::C(a as u8 * 3)
    } else {
        NumEnum113::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_113(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers113 {
    Numbers113 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum114 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers114 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_114(a: u32) -> NumEnum114 {
    if a == 1 {
        NumEnum114::A(a)
    } else if a == 2 {
        NumEnum114::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum114::C(a as u8 * 3)
    } else {
        NumEnum114::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_114(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers114 {
    Numbers114 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum115 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers115 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_115(a: u32) -> NumEnum115 {
    if a == 1 {
        NumEnum115::A(a)
    } else if a == 2 {
        NumEnum115::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum115::C(a as u8 * 3)
    } else {
        NumEnum115::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_115(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers115 {
    Numbers115 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum116 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers116 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_116(a: u32) -> NumEnum116 {
    if a == 1 {
        NumEnum116::A(a)
    } else if a == 2 {
        NumEnum116::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum116::C(a as u8 * 3)
    } else {
        NumEnum116::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_116(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers116 {
    Numbers116 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum117 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers117 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_117(a: u32) -> NumEnum117 {
    if a == 1 {
        NumEnum117::A(a)
    } else if a == 2 {
        NumEnum117::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum117::C(a as u8 * 3)
    } else {
        NumEnum117::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_117(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers117 {
    Numbers117 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum118 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers118 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_118(a: u32) -> NumEnum118 {
    if a == 1 {
        NumEnum118::A(a)
    } else if a == 2 {
        NumEnum118::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum118::C(a as u8 * 3)
    } else {
        NumEnum118::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_118(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers118 {
    Numbers118 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum119 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers119 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_119(a: u32) -> NumEnum119 {
    if a == 1 {
        NumEnum119::A(a)
    } else if a == 2 {
        NumEnum119::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum119::C(a as u8 * 3)
    } else {
        NumEnum119::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_119(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers119 {
    Numbers119 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum120 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers120 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_120(a: u32) -> NumEnum120 {
    if a == 1 {
        NumEnum120::A(a)
    } else if a == 2 {
        NumEnum120::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum120::C(a as u8 * 3)
    } else {
        NumEnum120::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_120(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers120 {
    Numbers120 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum121 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers121 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_121(a: u32) -> NumEnum121 {
    if a == 1 {
        NumEnum121::A(a)
    } else if a == 2 {
        NumEnum121::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum121::C(a as u8 * 3)
    } else {
        NumEnum121::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_121(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers121 {
    Numbers121 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum122 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers122 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_122(a: u32) -> NumEnum122 {
    if a == 1 {
        NumEnum122::A(a)
    } else if a == 2 {
        NumEnum122::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum122::C(a as u8 * 3)
    } else {
        NumEnum122::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_122(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers122 {
    Numbers122 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum123 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers123 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_123(a: u32) -> NumEnum123 {
    if a == 1 {
        NumEnum123::A(a)
    } else if a == 2 {
        NumEnum123::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum123::C(a as u8 * 3)
    } else {
        NumEnum123::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_123(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers123 {
    Numbers123 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum124 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers124 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_124(a: u32) -> NumEnum124 {
    if a == 1 {
        NumEnum124::A(a)
    } else if a == 2 {
        NumEnum124::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum124::C(a as u8 * 3)
    } else {
        NumEnum124::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_124(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers124 {
    Numbers124 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum125 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers125 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_125(a: u32) -> NumEnum125 {
    if a == 1 {
        NumEnum125::A(a)
    } else if a == 2 {
        NumEnum125::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum125::C(a as u8 * 3)
    } else {
        NumEnum125::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_125(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers125 {
    Numbers125 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum126 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers126 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_126(a: u32) -> NumEnum126 {
    if a == 1 {
        NumEnum126::A(a)
    } else if a == 2 {
        NumEnum126::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum126::C(a as u8 * 3)
    } else {
        NumEnum126::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_126(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers126 {
    Numbers126 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum127 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers127 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_127(a: u32) -> NumEnum127 {
    if a == 1 {
        NumEnum127::A(a)
    } else if a == 2 {
        NumEnum127::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum127::C(a as u8 * 3)
    } else {
        NumEnum127::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_127(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers127 {
    Numbers127 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum128 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers128 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_128(a: u32) -> NumEnum128 {
    if a == 1 {
        NumEnum128::A(a)
    } else if a == 2 {
        NumEnum128::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum128::C(a as u8 * 3)
    } else {
        NumEnum128::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_128(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers128 {
    Numbers128 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum129 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers129 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_129(a: u32) -> NumEnum129 {
    if a == 1 {
        NumEnum129::A(a)
    } else if a == 2 {
        NumEnum129::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum129::C(a as u8 * 3)
    } else {
        NumEnum129::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_129(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers129 {
    Numbers129 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum130 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers130 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_130(a: u32) -> NumEnum130 {
    if a == 1 {
        NumEnum130::A(a)
    } else if a == 2 {
        NumEnum130::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum130::C(a as u8 * 3)
    } else {
        NumEnum130::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_130(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers130 {
    Numbers130 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum131 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers131 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_131(a: u32) -> NumEnum131 {
    if a == 1 {
        NumEnum131::A(a)
    } else if a == 2 {
        NumEnum131::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum131::C(a as u8 * 3)
    } else {
        NumEnum131::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_131(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers131 {
    Numbers131 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum132 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers132 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_132(a: u32) -> NumEnum132 {
    if a == 1 {
        NumEnum132::A(a)
    } else if a == 2 {
        NumEnum132::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum132::C(a as u8 * 3)
    } else {
        NumEnum132::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_132(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers132 {
    Numbers132 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum133 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers133 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_133(a: u32) -> NumEnum133 {
    if a == 1 {
        NumEnum133::A(a)
    } else if a == 2 {
        NumEnum133::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum133::C(a as u8 * 3)
    } else {
        NumEnum133::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_133(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers133 {
    Numbers133 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum134 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers134 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_134(a: u32) -> NumEnum134 {
    if a == 1 {
        NumEnum134::A(a)
    } else if a == 2 {
        NumEnum134::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum134::C(a as u8 * 3)
    } else {
        NumEnum134::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_134(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers134 {
    Numbers134 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum135 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers135 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_135(a: u32) -> NumEnum135 {
    if a == 1 {
        NumEnum135::A(a)
    } else if a == 2 {
        NumEnum135::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum135::C(a as u8 * 3)
    } else {
        NumEnum135::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_135(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers135 {
    Numbers135 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum136 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers136 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_136(a: u32) -> NumEnum136 {
    if a == 1 {
        NumEnum136::A(a)
    } else if a == 2 {
        NumEnum136::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum136::C(a as u8 * 3)
    } else {
        NumEnum136::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_136(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers136 {
    Numbers136 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum137 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers137 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_137(a: u32) -> NumEnum137 {
    if a == 1 {
        NumEnum137::A(a)
    } else if a == 2 {
        NumEnum137::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum137::C(a as u8 * 3)
    } else {
        NumEnum137::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_137(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers137 {
    Numbers137 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum138 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers138 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_138(a: u32) -> NumEnum138 {
    if a == 1 {
        NumEnum138::A(a)
    } else if a == 2 {
        NumEnum138::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum138::C(a as u8 * 3)
    } else {
        NumEnum138::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_138(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers138 {
    Numbers138 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum139 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers139 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_139(a: u32) -> NumEnum139 {
    if a == 1 {
        NumEnum139::A(a)
    } else if a == 2 {
        NumEnum139::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum139::C(a as u8 * 3)
    } else {
        NumEnum139::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_139(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers139 {
    Numbers139 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum140 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers140 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_140(a: u32) -> NumEnum140 {
    if a == 1 {
        NumEnum140::A(a)
    } else if a == 2 {
        NumEnum140::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum140::C(a as u8 * 3)
    } else {
        NumEnum140::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_140(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers140 {
    Numbers140 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum141 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers141 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_141(a: u32) -> NumEnum141 {
    if a == 1 {
        NumEnum141::A(a)
    } else if a == 2 {
        NumEnum141::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum141::C(a as u8 * 3)
    } else {
        NumEnum141::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_141(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers141 {
    Numbers141 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum142 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers142 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_142(a: u32) -> NumEnum142 {
    if a == 1 {
        NumEnum142::A(a)
    } else if a == 2 {
        NumEnum142::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum142::C(a as u8 * 3)
    } else {
        NumEnum142::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_142(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers142 {
    Numbers142 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum143 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers143 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_143(a: u32) -> NumEnum143 {
    if a == 1 {
        NumEnum143::A(a)
    } else if a == 2 {
        NumEnum143::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum143::C(a as u8 * 3)
    } else {
        NumEnum143::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_143(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers143 {
    Numbers143 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum144 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers144 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_144(a: u32) -> NumEnum144 {
    if a == 1 {
        NumEnum144::A(a)
    } else if a == 2 {
        NumEnum144::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum144::C(a as u8 * 3)
    } else {
        NumEnum144::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_144(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers144 {
    Numbers144 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum145 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers145 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_145(a: u32) -> NumEnum145 {
    if a == 1 {
        NumEnum145::A(a)
    } else if a == 2 {
        NumEnum145::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum145::C(a as u8 * 3)
    } else {
        NumEnum145::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_145(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers145 {
    Numbers145 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum146 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers146 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_146(a: u32) -> NumEnum146 {
    if a == 1 {
        NumEnum146::A(a)
    } else if a == 2 {
        NumEnum146::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum146::C(a as u8 * 3)
    } else {
        NumEnum146::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_146(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers146 {
    Numbers146 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum147 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers147 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_147(a: u32) -> NumEnum147 {
    if a == 1 {
        NumEnum147::A(a)
    } else if a == 2 {
        NumEnum147::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum147::C(a as u8 * 3)
    } else {
        NumEnum147::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_147(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers147 {
    Numbers147 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum148 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers148 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_148(a: u32) -> NumEnum148 {
    if a == 1 {
        NumEnum148::A(a)
    } else if a == 2 {
        NumEnum148::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum148::C(a as u8 * 3)
    } else {
        NumEnum148::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_148(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers148 {
    Numbers148 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum149 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers149 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_149(a: u32) -> NumEnum149 {
    if a == 1 {
        NumEnum149::A(a)
    } else if a == 2 {
        NumEnum149::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum149::C(a as u8 * 3)
    } else {
        NumEnum149::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_149(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers149 {
    Numbers149 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum150 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers150 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_150(a: u32) -> NumEnum150 {
    if a == 1 {
        NumEnum150::A(a)
    } else if a == 2 {
        NumEnum150::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum150::C(a as u8 * 3)
    } else {
        NumEnum150::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_150(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers150 {
    Numbers150 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum151 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers151 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_151(a: u32) -> NumEnum151 {
    if a == 1 {
        NumEnum151::A(a)
    } else if a == 2 {
        NumEnum151::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum151::C(a as u8 * 3)
    } else {
        NumEnum151::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_151(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers151 {
    Numbers151 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum152 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers152 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_152(a: u32) -> NumEnum152 {
    if a == 1 {
        NumEnum152::A(a)
    } else if a == 2 {
        NumEnum152::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum152::C(a as u8 * 3)
    } else {
        NumEnum152::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_152(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers152 {
    Numbers152 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum153 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers153 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_153(a: u32) -> NumEnum153 {
    if a == 1 {
        NumEnum153::A(a)
    } else if a == 2 {
        NumEnum153::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum153::C(a as u8 * 3)
    } else {
        NumEnum153::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_153(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers153 {
    Numbers153 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum154 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers154 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_154(a: u32) -> NumEnum154 {
    if a == 1 {
        NumEnum154::A(a)
    } else if a == 2 {
        NumEnum154::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum154::C(a as u8 * 3)
    } else {
        NumEnum154::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_154(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers154 {
    Numbers154 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum155 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers155 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_155(a: u32) -> NumEnum155 {
    if a == 1 {
        NumEnum155::A(a)
    } else if a == 2 {
        NumEnum155::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum155::C(a as u8 * 3)
    } else {
        NumEnum155::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_155(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers155 {
    Numbers155 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum156 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers156 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_156(a: u32) -> NumEnum156 {
    if a == 1 {
        NumEnum156::A(a)
    } else if a == 2 {
        NumEnum156::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum156::C(a as u8 * 3)
    } else {
        NumEnum156::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_156(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers156 {
    Numbers156 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum157 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers157 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_157(a: u32) -> NumEnum157 {
    if a == 1 {
        NumEnum157::A(a)
    } else if a == 2 {
        NumEnum157::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum157::C(a as u8 * 3)
    } else {
        NumEnum157::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_157(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers157 {
    Numbers157 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum158 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers158 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_158(a: u32) -> NumEnum158 {
    if a == 1 {
        NumEnum158::A(a)
    } else if a == 2 {
        NumEnum158::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum158::C(a as u8 * 3)
    } else {
        NumEnum158::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_158(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers158 {
    Numbers158 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum159 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers159 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_159(a: u32) -> NumEnum159 {
    if a == 1 {
        NumEnum159::A(a)
    } else if a == 2 {
        NumEnum159::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum159::C(a as u8 * 3)
    } else {
        NumEnum159::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_159(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers159 {
    Numbers159 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum160 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers160 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_160(a: u32) -> NumEnum160 {
    if a == 1 {
        NumEnum160::A(a)
    } else if a == 2 {
        NumEnum160::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum160::C(a as u8 * 3)
    } else {
        NumEnum160::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_160(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers160 {
    Numbers160 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum161 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers161 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_161(a: u32) -> NumEnum161 {
    if a == 1 {
        NumEnum161::A(a)
    } else if a == 2 {
        NumEnum161::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum161::C(a as u8 * 3)
    } else {
        NumEnum161::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_161(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers161 {
    Numbers161 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum162 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers162 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_162(a: u32) -> NumEnum162 {
    if a == 1 {
        NumEnum162::A(a)
    } else if a == 2 {
        NumEnum162::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum162::C(a as u8 * 3)
    } else {
        NumEnum162::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_162(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers162 {
    Numbers162 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum163 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers163 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_163(a: u32) -> NumEnum163 {
    if a == 1 {
        NumEnum163::A(a)
    } else if a == 2 {
        NumEnum163::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum163::C(a as u8 * 3)
    } else {
        NumEnum163::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_163(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers163 {
    Numbers163 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum164 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers164 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_164(a: u32) -> NumEnum164 {
    if a == 1 {
        NumEnum164::A(a)
    } else if a == 2 {
        NumEnum164::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum164::C(a as u8 * 3)
    } else {
        NumEnum164::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_164(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers164 {
    Numbers164 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum165 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers165 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_165(a: u32) -> NumEnum165 {
    if a == 1 {
        NumEnum165::A(a)
    } else if a == 2 {
        NumEnum165::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum165::C(a as u8 * 3)
    } else {
        NumEnum165::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_165(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers165 {
    Numbers165 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum166 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers166 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_166(a: u32) -> NumEnum166 {
    if a == 1 {
        NumEnum166::A(a)
    } else if a == 2 {
        NumEnum166::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum166::C(a as u8 * 3)
    } else {
        NumEnum166::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_166(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers166 {
    Numbers166 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum167 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers167 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_167(a: u32) -> NumEnum167 {
    if a == 1 {
        NumEnum167::A(a)
    } else if a == 2 {
        NumEnum167::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum167::C(a as u8 * 3)
    } else {
        NumEnum167::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_167(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers167 {
    Numbers167 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum168 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers168 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_168(a: u32) -> NumEnum168 {
    if a == 1 {
        NumEnum168::A(a)
    } else if a == 2 {
        NumEnum168::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum168::C(a as u8 * 3)
    } else {
        NumEnum168::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_168(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers168 {
    Numbers168 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum169 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers169 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_169(a: u32) -> NumEnum169 {
    if a == 1 {
        NumEnum169::A(a)
    } else if a == 2 {
        NumEnum169::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum169::C(a as u8 * 3)
    } else {
        NumEnum169::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_169(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers169 {
    Numbers169 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum170 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers170 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_170(a: u32) -> NumEnum170 {
    if a == 1 {
        NumEnum170::A(a)
    } else if a == 2 {
        NumEnum170::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum170::C(a as u8 * 3)
    } else {
        NumEnum170::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_170(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers170 {
    Numbers170 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum171 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers171 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_171(a: u32) -> NumEnum171 {
    if a == 1 {
        NumEnum171::A(a)
    } else if a == 2 {
        NumEnum171::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum171::C(a as u8 * 3)
    } else {
        NumEnum171::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_171(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers171 {
    Numbers171 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum172 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers172 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_172(a: u32) -> NumEnum172 {
    if a == 1 {
        NumEnum172::A(a)
    } else if a == 2 {
        NumEnum172::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum172::C(a as u8 * 3)
    } else {
        NumEnum172::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_172(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers172 {
    Numbers172 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum173 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers173 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_173(a: u32) -> NumEnum173 {
    if a == 1 {
        NumEnum173::A(a)
    } else if a == 2 {
        NumEnum173::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum173::C(a as u8 * 3)
    } else {
        NumEnum173::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_173(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers173 {
    Numbers173 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum174 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers174 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_174(a: u32) -> NumEnum174 {
    if a == 1 {
        NumEnum174::A(a)
    } else if a == 2 {
        NumEnum174::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum174::C(a as u8 * 3)
    } else {
        NumEnum174::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_174(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers174 {
    Numbers174 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum175 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers175 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_175(a: u32) -> NumEnum175 {
    if a == 1 {
        NumEnum175::A(a)
    } else if a == 2 {
        NumEnum175::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum175::C(a as u8 * 3)
    } else {
        NumEnum175::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_175(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers175 {
    Numbers175 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum176 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers176 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_176(a: u32) -> NumEnum176 {
    if a == 1 {
        NumEnum176::A(a)
    } else if a == 2 {
        NumEnum176::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum176::C(a as u8 * 3)
    } else {
        NumEnum176::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_176(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers176 {
    Numbers176 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum177 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers177 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_177(a: u32) -> NumEnum177 {
    if a == 1 {
        NumEnum177::A(a)
    } else if a == 2 {
        NumEnum177::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum177::C(a as u8 * 3)
    } else {
        NumEnum177::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_177(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers177 {
    Numbers177 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum178 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers178 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_178(a: u32) -> NumEnum178 {
    if a == 1 {
        NumEnum178::A(a)
    } else if a == 2 {
        NumEnum178::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum178::C(a as u8 * 3)
    } else {
        NumEnum178::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_178(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers178 {
    Numbers178 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum179 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers179 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_179(a: u32) -> NumEnum179 {
    if a == 1 {
        NumEnum179::A(a)
    } else if a == 2 {
        NumEnum179::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum179::C(a as u8 * 3)
    } else {
        NumEnum179::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_179(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers179 {
    Numbers179 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum180 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers180 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_180(a: u32) -> NumEnum180 {
    if a == 1 {
        NumEnum180::A(a)
    } else if a == 2 {
        NumEnum180::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum180::C(a as u8 * 3)
    } else {
        NumEnum180::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_180(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers180 {
    Numbers180 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum181 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers181 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_181(a: u32) -> NumEnum181 {
    if a == 1 {
        NumEnum181::A(a)
    } else if a == 2 {
        NumEnum181::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum181::C(a as u8 * 3)
    } else {
        NumEnum181::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_181(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers181 {
    Numbers181 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum182 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers182 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_182(a: u32) -> NumEnum182 {
    if a == 1 {
        NumEnum182::A(a)
    } else if a == 2 {
        NumEnum182::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum182::C(a as u8 * 3)
    } else {
        NumEnum182::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_182(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers182 {
    Numbers182 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum183 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers183 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_183(a: u32) -> NumEnum183 {
    if a == 1 {
        NumEnum183::A(a)
    } else if a == 2 {
        NumEnum183::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum183::C(a as u8 * 3)
    } else {
        NumEnum183::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_183(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers183 {
    Numbers183 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum184 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers184 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_184(a: u32) -> NumEnum184 {
    if a == 1 {
        NumEnum184::A(a)
    } else if a == 2 {
        NumEnum184::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum184::C(a as u8 * 3)
    } else {
        NumEnum184::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_184(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers184 {
    Numbers184 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum185 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers185 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_185(a: u32) -> NumEnum185 {
    if a == 1 {
        NumEnum185::A(a)
    } else if a == 2 {
        NumEnum185::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum185::C(a as u8 * 3)
    } else {
        NumEnum185::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_185(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers185 {
    Numbers185 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum186 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers186 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_186(a: u32) -> NumEnum186 {
    if a == 1 {
        NumEnum186::A(a)
    } else if a == 2 {
        NumEnum186::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum186::C(a as u8 * 3)
    } else {
        NumEnum186::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_186(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers186 {
    Numbers186 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum187 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers187 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_187(a: u32) -> NumEnum187 {
    if a == 1 {
        NumEnum187::A(a)
    } else if a == 2 {
        NumEnum187::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum187::C(a as u8 * 3)
    } else {
        NumEnum187::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_187(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers187 {
    Numbers187 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum188 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers188 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_188(a: u32) -> NumEnum188 {
    if a == 1 {
        NumEnum188::A(a)
    } else if a == 2 {
        NumEnum188::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum188::C(a as u8 * 3)
    } else {
        NumEnum188::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_188(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers188 {
    Numbers188 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum189 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers189 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_189(a: u32) -> NumEnum189 {
    if a == 1 {
        NumEnum189::A(a)
    } else if a == 2 {
        NumEnum189::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum189::C(a as u8 * 3)
    } else {
        NumEnum189::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_189(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers189 {
    Numbers189 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum190 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers190 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_190(a: u32) -> NumEnum190 {
    if a == 1 {
        NumEnum190::A(a)
    } else if a == 2 {
        NumEnum190::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum190::C(a as u8 * 3)
    } else {
        NumEnum190::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_190(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers190 {
    Numbers190 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum191 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers191 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_191(a: u32) -> NumEnum191 {
    if a == 1 {
        NumEnum191::A(a)
    } else if a == 2 {
        NumEnum191::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum191::C(a as u8 * 3)
    } else {
        NumEnum191::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_191(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers191 {
    Numbers191 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum192 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers192 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_192(a: u32) -> NumEnum192 {
    if a == 1 {
        NumEnum192::A(a)
    } else if a == 2 {
        NumEnum192::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum192::C(a as u8 * 3)
    } else {
        NumEnum192::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_192(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers192 {
    Numbers192 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum193 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers193 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_193(a: u32) -> NumEnum193 {
    if a == 1 {
        NumEnum193::A(a)
    } else if a == 2 {
        NumEnum193::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum193::C(a as u8 * 3)
    } else {
        NumEnum193::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_193(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers193 {
    Numbers193 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum194 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers194 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_194(a: u32) -> NumEnum194 {
    if a == 1 {
        NumEnum194::A(a)
    } else if a == 2 {
        NumEnum194::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum194::C(a as u8 * 3)
    } else {
        NumEnum194::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_194(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers194 {
    Numbers194 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum195 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers195 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_195(a: u32) -> NumEnum195 {
    if a == 1 {
        NumEnum195::A(a)
    } else if a == 2 {
        NumEnum195::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum195::C(a as u8 * 3)
    } else {
        NumEnum195::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_195(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers195 {
    Numbers195 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum196 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers196 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_196(a: u32) -> NumEnum196 {
    if a == 1 {
        NumEnum196::A(a)
    } else if a == 2 {
        NumEnum196::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum196::C(a as u8 * 3)
    } else {
        NumEnum196::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_196(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers196 {
    Numbers196 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum197 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers197 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_197(a: u32) -> NumEnum197 {
    if a == 1 {
        NumEnum197::A(a)
    } else if a == 2 {
        NumEnum197::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum197::C(a as u8 * 3)
    } else {
        NumEnum197::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_197(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers197 {
    Numbers197 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum198 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers198 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_198(a: u32) -> NumEnum198 {
    if a == 1 {
        NumEnum198::A(a)
    } else if a == 2 {
        NumEnum198::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum198::C(a as u8 * 3)
    } else {
        NumEnum198::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_198(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers198 {
    Numbers198 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum199 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers199 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_199(a: u32) -> NumEnum199 {
    if a == 1 {
        NumEnum199::A(a)
    } else if a == 2 {
        NumEnum199::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum199::C(a as u8 * 3)
    } else {
        NumEnum199::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_199(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers199 {
    Numbers199 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum200 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers200 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_200(a: u32) -> NumEnum200 {
    if a == 1 {
        NumEnum200::A(a)
    } else if a == 2 {
        NumEnum200::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum200::C(a as u8 * 3)
    } else {
        NumEnum200::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_200(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers200 {
    Numbers200 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum201 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers201 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_201(a: u32) -> NumEnum201 {
    if a == 1 {
        NumEnum201::A(a)
    } else if a == 2 {
        NumEnum201::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum201::C(a as u8 * 3)
    } else {
        NumEnum201::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_201(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers201 {
    Numbers201 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum202 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers202 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_202(a: u32) -> NumEnum202 {
    if a == 1 {
        NumEnum202::A(a)
    } else if a == 2 {
        NumEnum202::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum202::C(a as u8 * 3)
    } else {
        NumEnum202::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_202(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers202 {
    Numbers202 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum203 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers203 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_203(a: u32) -> NumEnum203 {
    if a == 1 {
        NumEnum203::A(a)
    } else if a == 2 {
        NumEnum203::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum203::C(a as u8 * 3)
    } else {
        NumEnum203::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_203(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers203 {
    Numbers203 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum204 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers204 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_204(a: u32) -> NumEnum204 {
    if a == 1 {
        NumEnum204::A(a)
    } else if a == 2 {
        NumEnum204::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum204::C(a as u8 * 3)
    } else {
        NumEnum204::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_204(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers204 {
    Numbers204 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum205 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers205 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_205(a: u32) -> NumEnum205 {
    if a == 1 {
        NumEnum205::A(a)
    } else if a == 2 {
        NumEnum205::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum205::C(a as u8 * 3)
    } else {
        NumEnum205::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_205(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers205 {
    Numbers205 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum206 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers206 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_206(a: u32) -> NumEnum206 {
    if a == 1 {
        NumEnum206::A(a)
    } else if a == 2 {
        NumEnum206::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum206::C(a as u8 * 3)
    } else {
        NumEnum206::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_206(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers206 {
    Numbers206 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum207 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers207 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_207(a: u32) -> NumEnum207 {
    if a == 1 {
        NumEnum207::A(a)
    } else if a == 2 {
        NumEnum207::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum207::C(a as u8 * 3)
    } else {
        NumEnum207::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_207(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers207 {
    Numbers207 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum208 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers208 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_208(a: u32) -> NumEnum208 {
    if a == 1 {
        NumEnum208::A(a)
    } else if a == 2 {
        NumEnum208::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum208::C(a as u8 * 3)
    } else {
        NumEnum208::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_208(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers208 {
    Numbers208 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum209 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers209 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_209(a: u32) -> NumEnum209 {
    if a == 1 {
        NumEnum209::A(a)
    } else if a == 2 {
        NumEnum209::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum209::C(a as u8 * 3)
    } else {
        NumEnum209::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_209(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers209 {
    Numbers209 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum210 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers210 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_210(a: u32) -> NumEnum210 {
    if a == 1 {
        NumEnum210::A(a)
    } else if a == 2 {
        NumEnum210::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum210::C(a as u8 * 3)
    } else {
        NumEnum210::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_210(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers210 {
    Numbers210 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum211 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers211 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_211(a: u32) -> NumEnum211 {
    if a == 1 {
        NumEnum211::A(a)
    } else if a == 2 {
        NumEnum211::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum211::C(a as u8 * 3)
    } else {
        NumEnum211::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_211(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers211 {
    Numbers211 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum212 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers212 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_212(a: u32) -> NumEnum212 {
    if a == 1 {
        NumEnum212::A(a)
    } else if a == 2 {
        NumEnum212::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum212::C(a as u8 * 3)
    } else {
        NumEnum212::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_212(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers212 {
    Numbers212 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum213 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers213 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_213(a: u32) -> NumEnum213 {
    if a == 1 {
        NumEnum213::A(a)
    } else if a == 2 {
        NumEnum213::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum213::C(a as u8 * 3)
    } else {
        NumEnum213::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_213(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers213 {
    Numbers213 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum214 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers214 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_214(a: u32) -> NumEnum214 {
    if a == 1 {
        NumEnum214::A(a)
    } else if a == 2 {
        NumEnum214::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum214::C(a as u8 * 3)
    } else {
        NumEnum214::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_214(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers214 {
    Numbers214 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum215 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers215 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_215(a: u32) -> NumEnum215 {
    if a == 1 {
        NumEnum215::A(a)
    } else if a == 2 {
        NumEnum215::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum215::C(a as u8 * 3)
    } else {
        NumEnum215::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_215(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers215 {
    Numbers215 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum216 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers216 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_216(a: u32) -> NumEnum216 {
    if a == 1 {
        NumEnum216::A(a)
    } else if a == 2 {
        NumEnum216::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum216::C(a as u8 * 3)
    } else {
        NumEnum216::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_216(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers216 {
    Numbers216 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum217 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers217 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_217(a: u32) -> NumEnum217 {
    if a == 1 {
        NumEnum217::A(a)
    } else if a == 2 {
        NumEnum217::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum217::C(a as u8 * 3)
    } else {
        NumEnum217::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_217(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers217 {
    Numbers217 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum218 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers218 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_218(a: u32) -> NumEnum218 {
    if a == 1 {
        NumEnum218::A(a)
    } else if a == 2 {
        NumEnum218::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum218::C(a as u8 * 3)
    } else {
        NumEnum218::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_218(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers218 {
    Numbers218 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum219 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers219 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_219(a: u32) -> NumEnum219 {
    if a == 1 {
        NumEnum219::A(a)
    } else if a == 2 {
        NumEnum219::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum219::C(a as u8 * 3)
    } else {
        NumEnum219::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_219(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers219 {
    Numbers219 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum220 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers220 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_220(a: u32) -> NumEnum220 {
    if a == 1 {
        NumEnum220::A(a)
    } else if a == 2 {
        NumEnum220::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum220::C(a as u8 * 3)
    } else {
        NumEnum220::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_220(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers220 {
    Numbers220 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum221 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers221 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_221(a: u32) -> NumEnum221 {
    if a == 1 {
        NumEnum221::A(a)
    } else if a == 2 {
        NumEnum221::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum221::C(a as u8 * 3)
    } else {
        NumEnum221::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_221(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers221 {
    Numbers221 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum222 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers222 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_222(a: u32) -> NumEnum222 {
    if a == 1 {
        NumEnum222::A(a)
    } else if a == 2 {
        NumEnum222::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum222::C(a as u8 * 3)
    } else {
        NumEnum222::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_222(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers222 {
    Numbers222 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum223 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers223 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_223(a: u32) -> NumEnum223 {
    if a == 1 {
        NumEnum223::A(a)
    } else if a == 2 {
        NumEnum223::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum223::C(a as u8 * 3)
    } else {
        NumEnum223::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_223(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers223 {
    Numbers223 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum224 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers224 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_224(a: u32) -> NumEnum224 {
    if a == 1 {
        NumEnum224::A(a)
    } else if a == 2 {
        NumEnum224::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum224::C(a as u8 * 3)
    } else {
        NumEnum224::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_224(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers224 {
    Numbers224 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum225 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers225 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_225(a: u32) -> NumEnum225 {
    if a == 1 {
        NumEnum225::A(a)
    } else if a == 2 {
        NumEnum225::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum225::C(a as u8 * 3)
    } else {
        NumEnum225::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_225(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers225 {
    Numbers225 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum226 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers226 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_226(a: u32) -> NumEnum226 {
    if a == 1 {
        NumEnum226::A(a)
    } else if a == 2 {
        NumEnum226::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum226::C(a as u8 * 3)
    } else {
        NumEnum226::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_226(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers226 {
    Numbers226 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum227 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers227 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_227(a: u32) -> NumEnum227 {
    if a == 1 {
        NumEnum227::A(a)
    } else if a == 2 {
        NumEnum227::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum227::C(a as u8 * 3)
    } else {
        NumEnum227::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_227(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers227 {
    Numbers227 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum228 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers228 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_228(a: u32) -> NumEnum228 {
    if a == 1 {
        NumEnum228::A(a)
    } else if a == 2 {
        NumEnum228::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum228::C(a as u8 * 3)
    } else {
        NumEnum228::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_228(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers228 {
    Numbers228 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum229 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers229 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_229(a: u32) -> NumEnum229 {
    if a == 1 {
        NumEnum229::A(a)
    } else if a == 2 {
        NumEnum229::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum229::C(a as u8 * 3)
    } else {
        NumEnum229::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_229(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers229 {
    Numbers229 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum230 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers230 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_230(a: u32) -> NumEnum230 {
    if a == 1 {
        NumEnum230::A(a)
    } else if a == 2 {
        NumEnum230::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum230::C(a as u8 * 3)
    } else {
        NumEnum230::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_230(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers230 {
    Numbers230 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum231 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers231 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_231(a: u32) -> NumEnum231 {
    if a == 1 {
        NumEnum231::A(a)
    } else if a == 2 {
        NumEnum231::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum231::C(a as u8 * 3)
    } else {
        NumEnum231::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_231(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers231 {
    Numbers231 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum232 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers232 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_232(a: u32) -> NumEnum232 {
    if a == 1 {
        NumEnum232::A(a)
    } else if a == 2 {
        NumEnum232::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum232::C(a as u8 * 3)
    } else {
        NumEnum232::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_232(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers232 {
    Numbers232 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum233 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers233 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_233(a: u32) -> NumEnum233 {
    if a == 1 {
        NumEnum233::A(a)
    } else if a == 2 {
        NumEnum233::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum233::C(a as u8 * 3)
    } else {
        NumEnum233::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_233(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers233 {
    Numbers233 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum234 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers234 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_234(a: u32) -> NumEnum234 {
    if a == 1 {
        NumEnum234::A(a)
    } else if a == 2 {
        NumEnum234::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum234::C(a as u8 * 3)
    } else {
        NumEnum234::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_234(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers234 {
    Numbers234 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum235 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers235 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_235(a: u32) -> NumEnum235 {
    if a == 1 {
        NumEnum235::A(a)
    } else if a == 2 {
        NumEnum235::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum235::C(a as u8 * 3)
    } else {
        NumEnum235::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_235(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers235 {
    Numbers235 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum236 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers236 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_236(a: u32) -> NumEnum236 {
    if a == 1 {
        NumEnum236::A(a)
    } else if a == 2 {
        NumEnum236::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum236::C(a as u8 * 3)
    } else {
        NumEnum236::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_236(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers236 {
    Numbers236 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum237 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers237 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_237(a: u32) -> NumEnum237 {
    if a == 1 {
        NumEnum237::A(a)
    } else if a == 2 {
        NumEnum237::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum237::C(a as u8 * 3)
    } else {
        NumEnum237::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_237(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers237 {
    Numbers237 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum238 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers238 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_238(a: u32) -> NumEnum238 {
    if a == 1 {
        NumEnum238::A(a)
    } else if a == 2 {
        NumEnum238::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum238::C(a as u8 * 3)
    } else {
        NumEnum238::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_238(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers238 {
    Numbers238 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum239 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers239 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_239(a: u32) -> NumEnum239 {
    if a == 1 {
        NumEnum239::A(a)
    } else if a == 2 {
        NumEnum239::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum239::C(a as u8 * 3)
    } else {
        NumEnum239::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_239(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers239 {
    Numbers239 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum240 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers240 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_240(a: u32) -> NumEnum240 {
    if a == 1 {
        NumEnum240::A(a)
    } else if a == 2 {
        NumEnum240::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum240::C(a as u8 * 3)
    } else {
        NumEnum240::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_240(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers240 {
    Numbers240 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum241 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers241 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_241(a: u32) -> NumEnum241 {
    if a == 1 {
        NumEnum241::A(a)
    } else if a == 2 {
        NumEnum241::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum241::C(a as u8 * 3)
    } else {
        NumEnum241::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_241(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers241 {
    Numbers241 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum242 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers242 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_242(a: u32) -> NumEnum242 {
    if a == 1 {
        NumEnum242::A(a)
    } else if a == 2 {
        NumEnum242::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum242::C(a as u8 * 3)
    } else {
        NumEnum242::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_242(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers242 {
    Numbers242 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum243 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers243 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_243(a: u32) -> NumEnum243 {
    if a == 1 {
        NumEnum243::A(a)
    } else if a == 2 {
        NumEnum243::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum243::C(a as u8 * 3)
    } else {
        NumEnum243::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_243(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers243 {
    Numbers243 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum244 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers244 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_244(a: u32) -> NumEnum244 {
    if a == 1 {
        NumEnum244::A(a)
    } else if a == 2 {
        NumEnum244::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum244::C(a as u8 * 3)
    } else {
        NumEnum244::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_244(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers244 {
    Numbers244 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum245 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers245 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_245(a: u32) -> NumEnum245 {
    if a == 1 {
        NumEnum245::A(a)
    } else if a == 2 {
        NumEnum245::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum245::C(a as u8 * 3)
    } else {
        NumEnum245::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_245(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers245 {
    Numbers245 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum246 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers246 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_246(a: u32) -> NumEnum246 {
    if a == 1 {
        NumEnum246::A(a)
    } else if a == 2 {
        NumEnum246::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum246::C(a as u8 * 3)
    } else {
        NumEnum246::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_246(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers246 {
    Numbers246 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum247 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers247 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_247(a: u32) -> NumEnum247 {
    if a == 1 {
        NumEnum247::A(a)
    } else if a == 2 {
        NumEnum247::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum247::C(a as u8 * 3)
    } else {
        NumEnum247::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_247(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers247 {
    Numbers247 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum248 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers248 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_248(a: u32) -> NumEnum248 {
    if a == 1 {
        NumEnum248::A(a)
    } else if a == 2 {
        NumEnum248::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum248::C(a as u8 * 3)
    } else {
        NumEnum248::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_248(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers248 {
    Numbers248 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum249 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers249 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_249(a: u32) -> NumEnum249 {
    if a == 1 {
        NumEnum249::A(a)
    } else if a == 2 {
        NumEnum249::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum249::C(a as u8 * 3)
    } else {
        NumEnum249::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_249(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers249 {
    Numbers249 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum250 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers250 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_250(a: u32) -> NumEnum250 {
    if a == 1 {
        NumEnum250::A(a)
    } else if a == 2 {
        NumEnum250::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum250::C(a as u8 * 3)
    } else {
        NumEnum250::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_250(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers250 {
    Numbers250 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum251 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers251 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_251(a: u32) -> NumEnum251 {
    if a == 1 {
        NumEnum251::A(a)
    } else if a == 2 {
        NumEnum251::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum251::C(a as u8 * 3)
    } else {
        NumEnum251::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_251(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers251 {
    Numbers251 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum252 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers252 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_252(a: u32) -> NumEnum252 {
    if a == 1 {
        NumEnum252::A(a)
    } else if a == 2 {
        NumEnum252::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum252::C(a as u8 * 3)
    } else {
        NumEnum252::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_252(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers252 {
    Numbers252 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum253 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers253 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_253(a: u32) -> NumEnum253 {
    if a == 1 {
        NumEnum253::A(a)
    } else if a == 2 {
        NumEnum253::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum253::C(a as u8 * 3)
    } else {
        NumEnum253::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_253(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers253 {
    Numbers253 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum254 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers254 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_254(a: u32) -> NumEnum254 {
    if a == 1 {
        NumEnum254::A(a)
    } else if a == 2 {
        NumEnum254::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum254::C(a as u8 * 3)
    } else {
        NumEnum254::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_254(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers254 {
    Numbers254 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum255 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers255 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_255(a: u32) -> NumEnum255 {
    if a == 1 {
        NumEnum255::A(a)
    } else if a == 2 {
        NumEnum255::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum255::C(a as u8 * 3)
    } else {
        NumEnum255::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_255(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers255 {
    Numbers255 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum256 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers256 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_256(a: u32) -> NumEnum256 {
    if a == 1 {
        NumEnum256::A(a)
    } else if a == 2 {
        NumEnum256::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum256::C(a as u8 * 3)
    } else {
        NumEnum256::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_256(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers256 {
    Numbers256 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum257 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers257 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_257(a: u32) -> NumEnum257 {
    if a == 1 {
        NumEnum257::A(a)
    } else if a == 2 {
        NumEnum257::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum257::C(a as u8 * 3)
    } else {
        NumEnum257::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_257(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers257 {
    Numbers257 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum258 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers258 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_258(a: u32) -> NumEnum258 {
    if a == 1 {
        NumEnum258::A(a)
    } else if a == 2 {
        NumEnum258::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum258::C(a as u8 * 3)
    } else {
        NumEnum258::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_258(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers258 {
    Numbers258 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum259 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers259 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_259(a: u32) -> NumEnum259 {
    if a == 1 {
        NumEnum259::A(a)
    } else if a == 2 {
        NumEnum259::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum259::C(a as u8 * 3)
    } else {
        NumEnum259::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_259(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers259 {
    Numbers259 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum260 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers260 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_260(a: u32) -> NumEnum260 {
    if a == 1 {
        NumEnum260::A(a)
    } else if a == 2 {
        NumEnum260::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum260::C(a as u8 * 3)
    } else {
        NumEnum260::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_260(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers260 {
    Numbers260 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum261 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers261 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_261(a: u32) -> NumEnum261 {
    if a == 1 {
        NumEnum261::A(a)
    } else if a == 2 {
        NumEnum261::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum261::C(a as u8 * 3)
    } else {
        NumEnum261::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_261(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers261 {
    Numbers261 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum262 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers262 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_262(a: u32) -> NumEnum262 {
    if a == 1 {
        NumEnum262::A(a)
    } else if a == 2 {
        NumEnum262::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum262::C(a as u8 * 3)
    } else {
        NumEnum262::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_262(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers262 {
    Numbers262 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum263 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers263 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_263(a: u32) -> NumEnum263 {
    if a == 1 {
        NumEnum263::A(a)
    } else if a == 2 {
        NumEnum263::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum263::C(a as u8 * 3)
    } else {
        NumEnum263::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_263(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers263 {
    Numbers263 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum264 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers264 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_264(a: u32) -> NumEnum264 {
    if a == 1 {
        NumEnum264::A(a)
    } else if a == 2 {
        NumEnum264::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum264::C(a as u8 * 3)
    } else {
        NumEnum264::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_264(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers264 {
    Numbers264 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum265 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers265 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_265(a: u32) -> NumEnum265 {
    if a == 1 {
        NumEnum265::A(a)
    } else if a == 2 {
        NumEnum265::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum265::C(a as u8 * 3)
    } else {
        NumEnum265::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_265(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers265 {
    Numbers265 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum266 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers266 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_266(a: u32) -> NumEnum266 {
    if a == 1 {
        NumEnum266::A(a)
    } else if a == 2 {
        NumEnum266::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum266::C(a as u8 * 3)
    } else {
        NumEnum266::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_266(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers266 {
    Numbers266 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum267 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers267 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_267(a: u32) -> NumEnum267 {
    if a == 1 {
        NumEnum267::A(a)
    } else if a == 2 {
        NumEnum267::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum267::C(a as u8 * 3)
    } else {
        NumEnum267::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_267(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers267 {
    Numbers267 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum268 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers268 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_268(a: u32) -> NumEnum268 {
    if a == 1 {
        NumEnum268::A(a)
    } else if a == 2 {
        NumEnum268::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum268::C(a as u8 * 3)
    } else {
        NumEnum268::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_268(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers268 {
    Numbers268 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum269 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers269 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_269(a: u32) -> NumEnum269 {
    if a == 1 {
        NumEnum269::A(a)
    } else if a == 2 {
        NumEnum269::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum269::C(a as u8 * 3)
    } else {
        NumEnum269::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_269(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers269 {
    Numbers269 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum270 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers270 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_270(a: u32) -> NumEnum270 {
    if a == 1 {
        NumEnum270::A(a)
    } else if a == 2 {
        NumEnum270::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum270::C(a as u8 * 3)
    } else {
        NumEnum270::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_270(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers270 {
    Numbers270 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum271 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers271 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_271(a: u32) -> NumEnum271 {
    if a == 1 {
        NumEnum271::A(a)
    } else if a == 2 {
        NumEnum271::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum271::C(a as u8 * 3)
    } else {
        NumEnum271::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_271(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers271 {
    Numbers271 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum272 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers272 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_272(a: u32) -> NumEnum272 {
    if a == 1 {
        NumEnum272::A(a)
    } else if a == 2 {
        NumEnum272::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum272::C(a as u8 * 3)
    } else {
        NumEnum272::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_272(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers272 {
    Numbers272 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum273 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers273 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_273(a: u32) -> NumEnum273 {
    if a == 1 {
        NumEnum273::A(a)
    } else if a == 2 {
        NumEnum273::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum273::C(a as u8 * 3)
    } else {
        NumEnum273::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_273(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers273 {
    Numbers273 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum274 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers274 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_274(a: u32) -> NumEnum274 {
    if a == 1 {
        NumEnum274::A(a)
    } else if a == 2 {
        NumEnum274::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum274::C(a as u8 * 3)
    } else {
        NumEnum274::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_274(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers274 {
    Numbers274 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum275 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers275 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_275(a: u32) -> NumEnum275 {
    if a == 1 {
        NumEnum275::A(a)
    } else if a == 2 {
        NumEnum275::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum275::C(a as u8 * 3)
    } else {
        NumEnum275::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_275(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers275 {
    Numbers275 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum276 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers276 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_276(a: u32) -> NumEnum276 {
    if a == 1 {
        NumEnum276::A(a)
    } else if a == 2 {
        NumEnum276::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum276::C(a as u8 * 3)
    } else {
        NumEnum276::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_276(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers276 {
    Numbers276 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum277 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers277 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_277(a: u32) -> NumEnum277 {
    if a == 1 {
        NumEnum277::A(a)
    } else if a == 2 {
        NumEnum277::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum277::C(a as u8 * 3)
    } else {
        NumEnum277::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_277(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers277 {
    Numbers277 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum278 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers278 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_278(a: u32) -> NumEnum278 {
    if a == 1 {
        NumEnum278::A(a)
    } else if a == 2 {
        NumEnum278::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum278::C(a as u8 * 3)
    } else {
        NumEnum278::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_278(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers278 {
    Numbers278 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum279 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers279 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_279(a: u32) -> NumEnum279 {
    if a == 1 {
        NumEnum279::A(a)
    } else if a == 2 {
        NumEnum279::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum279::C(a as u8 * 3)
    } else {
        NumEnum279::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_279(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers279 {
    Numbers279 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum280 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers280 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_280(a: u32) -> NumEnum280 {
    if a == 1 {
        NumEnum280::A(a)
    } else if a == 2 {
        NumEnum280::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum280::C(a as u8 * 3)
    } else {
        NumEnum280::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_280(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers280 {
    Numbers280 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum281 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers281 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_281(a: u32) -> NumEnum281 {
    if a == 1 {
        NumEnum281::A(a)
    } else if a == 2 {
        NumEnum281::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum281::C(a as u8 * 3)
    } else {
        NumEnum281::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_281(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers281 {
    Numbers281 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum282 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers282 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_282(a: u32) -> NumEnum282 {
    if a == 1 {
        NumEnum282::A(a)
    } else if a == 2 {
        NumEnum282::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum282::C(a as u8 * 3)
    } else {
        NumEnum282::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_282(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers282 {
    Numbers282 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum283 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers283 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_283(a: u32) -> NumEnum283 {
    if a == 1 {
        NumEnum283::A(a)
    } else if a == 2 {
        NumEnum283::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum283::C(a as u8 * 3)
    } else {
        NumEnum283::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_283(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers283 {
    Numbers283 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum284 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers284 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_284(a: u32) -> NumEnum284 {
    if a == 1 {
        NumEnum284::A(a)
    } else if a == 2 {
        NumEnum284::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum284::C(a as u8 * 3)
    } else {
        NumEnum284::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_284(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers284 {
    Numbers284 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum285 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers285 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_285(a: u32) -> NumEnum285 {
    if a == 1 {
        NumEnum285::A(a)
    } else if a == 2 {
        NumEnum285::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum285::C(a as u8 * 3)
    } else {
        NumEnum285::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_285(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers285 {
    Numbers285 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum286 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers286 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_286(a: u32) -> NumEnum286 {
    if a == 1 {
        NumEnum286::A(a)
    } else if a == 2 {
        NumEnum286::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum286::C(a as u8 * 3)
    } else {
        NumEnum286::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_286(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers286 {
    Numbers286 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum287 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers287 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_287(a: u32) -> NumEnum287 {
    if a == 1 {
        NumEnum287::A(a)
    } else if a == 2 {
        NumEnum287::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum287::C(a as u8 * 3)
    } else {
        NumEnum287::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_287(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers287 {
    Numbers287 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum288 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers288 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_288(a: u32) -> NumEnum288 {
    if a == 1 {
        NumEnum288::A(a)
    } else if a == 2 {
        NumEnum288::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum288::C(a as u8 * 3)
    } else {
        NumEnum288::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_288(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers288 {
    Numbers288 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum289 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers289 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_289(a: u32) -> NumEnum289 {
    if a == 1 {
        NumEnum289::A(a)
    } else if a == 2 {
        NumEnum289::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum289::C(a as u8 * 3)
    } else {
        NumEnum289::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_289(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers289 {
    Numbers289 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum290 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers290 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_290(a: u32) -> NumEnum290 {
    if a == 1 {
        NumEnum290::A(a)
    } else if a == 2 {
        NumEnum290::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum290::C(a as u8 * 3)
    } else {
        NumEnum290::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_290(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers290 {
    Numbers290 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum291 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers291 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_291(a: u32) -> NumEnum291 {
    if a == 1 {
        NumEnum291::A(a)
    } else if a == 2 {
        NumEnum291::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum291::C(a as u8 * 3)
    } else {
        NumEnum291::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_291(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers291 {
    Numbers291 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum292 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers292 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_292(a: u32) -> NumEnum292 {
    if a == 1 {
        NumEnum292::A(a)
    } else if a == 2 {
        NumEnum292::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum292::C(a as u8 * 3)
    } else {
        NumEnum292::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_292(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers292 {
    Numbers292 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum293 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers293 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_293(a: u32) -> NumEnum293 {
    if a == 1 {
        NumEnum293::A(a)
    } else if a == 2 {
        NumEnum293::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum293::C(a as u8 * 3)
    } else {
        NumEnum293::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_293(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers293 {
    Numbers293 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum294 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers294 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_294(a: u32) -> NumEnum294 {
    if a == 1 {
        NumEnum294::A(a)
    } else if a == 2 {
        NumEnum294::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum294::C(a as u8 * 3)
    } else {
        NumEnum294::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_294(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers294 {
    Numbers294 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum295 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers295 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_295(a: u32) -> NumEnum295 {
    if a == 1 {
        NumEnum295::A(a)
    } else if a == 2 {
        NumEnum295::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum295::C(a as u8 * 3)
    } else {
        NumEnum295::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_295(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers295 {
    Numbers295 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum296 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers296 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_296(a: u32) -> NumEnum296 {
    if a == 1 {
        NumEnum296::A(a)
    } else if a == 2 {
        NumEnum296::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum296::C(a as u8 * 3)
    } else {
        NumEnum296::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_296(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers296 {
    Numbers296 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum297 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers297 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_297(a: u32) -> NumEnum297 {
    if a == 1 {
        NumEnum297::A(a)
    } else if a == 2 {
        NumEnum297::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum297::C(a as u8 * 3)
    } else {
        NumEnum297::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_297(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers297 {
    Numbers297 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum298 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers298 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_298(a: u32) -> NumEnum298 {
    if a == 1 {
        NumEnum298::A(a)
    } else if a == 2 {
        NumEnum298::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum298::C(a as u8 * 3)
    } else {
        NumEnum298::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_298(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers298 {
    Numbers298 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum299 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers299 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_299(a: u32) -> NumEnum299 {
    if a == 1 {
        NumEnum299::A(a)
    } else if a == 2 {
        NumEnum299::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum299::C(a as u8 * 3)
    } else {
        NumEnum299::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_299(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers299 {
    Numbers299 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum300 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers300 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_300(a: u32) -> NumEnum300 {
    if a == 1 {
        NumEnum300::A(a)
    } else if a == 2 {
        NumEnum300::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum300::C(a as u8 * 3)
    } else {
        NumEnum300::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_300(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers300 {
    Numbers300 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum301 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers301 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_301(a: u32) -> NumEnum301 {
    if a == 1 {
        NumEnum301::A(a)
    } else if a == 2 {
        NumEnum301::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum301::C(a as u8 * 3)
    } else {
        NumEnum301::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_301(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers301 {
    Numbers301 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum302 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers302 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_302(a: u32) -> NumEnum302 {
    if a == 1 {
        NumEnum302::A(a)
    } else if a == 2 {
        NumEnum302::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum302::C(a as u8 * 3)
    } else {
        NumEnum302::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_302(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers302 {
    Numbers302 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum303 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers303 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_303(a: u32) -> NumEnum303 {
    if a == 1 {
        NumEnum303::A(a)
    } else if a == 2 {
        NumEnum303::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum303::C(a as u8 * 3)
    } else {
        NumEnum303::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_303(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers303 {
    Numbers303 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum304 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers304 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_304(a: u32) -> NumEnum304 {
    if a == 1 {
        NumEnum304::A(a)
    } else if a == 2 {
        NumEnum304::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum304::C(a as u8 * 3)
    } else {
        NumEnum304::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_304(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers304 {
    Numbers304 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum305 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers305 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_305(a: u32) -> NumEnum305 {
    if a == 1 {
        NumEnum305::A(a)
    } else if a == 2 {
        NumEnum305::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum305::C(a as u8 * 3)
    } else {
        NumEnum305::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_305(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers305 {
    Numbers305 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum306 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers306 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_306(a: u32) -> NumEnum306 {
    if a == 1 {
        NumEnum306::A(a)
    } else if a == 2 {
        NumEnum306::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum306::C(a as u8 * 3)
    } else {
        NumEnum306::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_306(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers306 {
    Numbers306 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum307 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers307 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_307(a: u32) -> NumEnum307 {
    if a == 1 {
        NumEnum307::A(a)
    } else if a == 2 {
        NumEnum307::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum307::C(a as u8 * 3)
    } else {
        NumEnum307::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_307(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers307 {
    Numbers307 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum308 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers308 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_308(a: u32) -> NumEnum308 {
    if a == 1 {
        NumEnum308::A(a)
    } else if a == 2 {
        NumEnum308::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum308::C(a as u8 * 3)
    } else {
        NumEnum308::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_308(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers308 {
    Numbers308 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum309 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers309 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_309(a: u32) -> NumEnum309 {
    if a == 1 {
        NumEnum309::A(a)
    } else if a == 2 {
        NumEnum309::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum309::C(a as u8 * 3)
    } else {
        NumEnum309::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_309(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers309 {
    Numbers309 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum310 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers310 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_310(a: u32) -> NumEnum310 {
    if a == 1 {
        NumEnum310::A(a)
    } else if a == 2 {
        NumEnum310::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum310::C(a as u8 * 3)
    } else {
        NumEnum310::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_310(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers310 {
    Numbers310 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum311 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers311 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_311(a: u32) -> NumEnum311 {
    if a == 1 {
        NumEnum311::A(a)
    } else if a == 2 {
        NumEnum311::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum311::C(a as u8 * 3)
    } else {
        NumEnum311::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_311(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers311 {
    Numbers311 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum312 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers312 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_312(a: u32) -> NumEnum312 {
    if a == 1 {
        NumEnum312::A(a)
    } else if a == 2 {
        NumEnum312::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum312::C(a as u8 * 3)
    } else {
        NumEnum312::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_312(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers312 {
    Numbers312 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum313 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers313 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_313(a: u32) -> NumEnum313 {
    if a == 1 {
        NumEnum313::A(a)
    } else if a == 2 {
        NumEnum313::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum313::C(a as u8 * 3)
    } else {
        NumEnum313::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_313(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers313 {
    Numbers313 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum314 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers314 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_314(a: u32) -> NumEnum314 {
    if a == 1 {
        NumEnum314::A(a)
    } else if a == 2 {
        NumEnum314::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum314::C(a as u8 * 3)
    } else {
        NumEnum314::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_314(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers314 {
    Numbers314 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum315 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers315 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_315(a: u32) -> NumEnum315 {
    if a == 1 {
        NumEnum315::A(a)
    } else if a == 2 {
        NumEnum315::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum315::C(a as u8 * 3)
    } else {
        NumEnum315::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_315(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers315 {
    Numbers315 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum316 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers316 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_316(a: u32) -> NumEnum316 {
    if a == 1 {
        NumEnum316::A(a)
    } else if a == 2 {
        NumEnum316::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum316::C(a as u8 * 3)
    } else {
        NumEnum316::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_316(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers316 {
    Numbers316 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum317 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers317 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_317(a: u32) -> NumEnum317 {
    if a == 1 {
        NumEnum317::A(a)
    } else if a == 2 {
        NumEnum317::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum317::C(a as u8 * 3)
    } else {
        NumEnum317::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_317(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers317 {
    Numbers317 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum318 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers318 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_318(a: u32) -> NumEnum318 {
    if a == 1 {
        NumEnum318::A(a)
    } else if a == 2 {
        NumEnum318::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum318::C(a as u8 * 3)
    } else {
        NumEnum318::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_318(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers318 {
    Numbers318 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum319 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers319 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_319(a: u32) -> NumEnum319 {
    if a == 1 {
        NumEnum319::A(a)
    } else if a == 2 {
        NumEnum319::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum319::C(a as u8 * 3)
    } else {
        NumEnum319::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_319(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers319 {
    Numbers319 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum320 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers320 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_320(a: u32) -> NumEnum320 {
    if a == 1 {
        NumEnum320::A(a)
    } else if a == 2 {
        NumEnum320::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum320::C(a as u8 * 3)
    } else {
        NumEnum320::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_320(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers320 {
    Numbers320 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum321 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers321 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_321(a: u32) -> NumEnum321 {
    if a == 1 {
        NumEnum321::A(a)
    } else if a == 2 {
        NumEnum321::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum321::C(a as u8 * 3)
    } else {
        NumEnum321::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_321(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers321 {
    Numbers321 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum322 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers322 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_322(a: u32) -> NumEnum322 {
    if a == 1 {
        NumEnum322::A(a)
    } else if a == 2 {
        NumEnum322::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum322::C(a as u8 * 3)
    } else {
        NumEnum322::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_322(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers322 {
    Numbers322 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum323 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers323 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_323(a: u32) -> NumEnum323 {
    if a == 1 {
        NumEnum323::A(a)
    } else if a == 2 {
        NumEnum323::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum323::C(a as u8 * 3)
    } else {
        NumEnum323::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_323(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers323 {
    Numbers323 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum324 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers324 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_324(a: u32) -> NumEnum324 {
    if a == 1 {
        NumEnum324::A(a)
    } else if a == 2 {
        NumEnum324::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum324::C(a as u8 * 3)
    } else {
        NumEnum324::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_324(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers324 {
    Numbers324 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum325 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers325 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_325(a: u32) -> NumEnum325 {
    if a == 1 {
        NumEnum325::A(a)
    } else if a == 2 {
        NumEnum325::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum325::C(a as u8 * 3)
    } else {
        NumEnum325::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_325(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers325 {
    Numbers325 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum326 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers326 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_326(a: u32) -> NumEnum326 {
    if a == 1 {
        NumEnum326::A(a)
    } else if a == 2 {
        NumEnum326::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum326::C(a as u8 * 3)
    } else {
        NumEnum326::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_326(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers326 {
    Numbers326 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum327 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers327 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_327(a: u32) -> NumEnum327 {
    if a == 1 {
        NumEnum327::A(a)
    } else if a == 2 {
        NumEnum327::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum327::C(a as u8 * 3)
    } else {
        NumEnum327::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_327(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers327 {
    Numbers327 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum328 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers328 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_328(a: u32) -> NumEnum328 {
    if a == 1 {
        NumEnum328::A(a)
    } else if a == 2 {
        NumEnum328::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum328::C(a as u8 * 3)
    } else {
        NumEnum328::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_328(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers328 {
    Numbers328 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum329 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers329 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_329(a: u32) -> NumEnum329 {
    if a == 1 {
        NumEnum329::A(a)
    } else if a == 2 {
        NumEnum329::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum329::C(a as u8 * 3)
    } else {
        NumEnum329::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_329(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers329 {
    Numbers329 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum330 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers330 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_330(a: u32) -> NumEnum330 {
    if a == 1 {
        NumEnum330::A(a)
    } else if a == 2 {
        NumEnum330::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum330::C(a as u8 * 3)
    } else {
        NumEnum330::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_330(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers330 {
    Numbers330 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum331 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers331 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_331(a: u32) -> NumEnum331 {
    if a == 1 {
        NumEnum331::A(a)
    } else if a == 2 {
        NumEnum331::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum331::C(a as u8 * 3)
    } else {
        NumEnum331::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_331(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers331 {
    Numbers331 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum332 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers332 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_332(a: u32) -> NumEnum332 {
    if a == 1 {
        NumEnum332::A(a)
    } else if a == 2 {
        NumEnum332::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum332::C(a as u8 * 3)
    } else {
        NumEnum332::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_332(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers332 {
    Numbers332 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum333 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers333 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_333(a: u32) -> NumEnum333 {
    if a == 1 {
        NumEnum333::A(a)
    } else if a == 2 {
        NumEnum333::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum333::C(a as u8 * 3)
    } else {
        NumEnum333::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_333(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers333 {
    Numbers333 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum334 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers334 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_334(a: u32) -> NumEnum334 {
    if a == 1 {
        NumEnum334::A(a)
    } else if a == 2 {
        NumEnum334::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum334::C(a as u8 * 3)
    } else {
        NumEnum334::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_334(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers334 {
    Numbers334 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum335 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers335 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_335(a: u32) -> NumEnum335 {
    if a == 1 {
        NumEnum335::A(a)
    } else if a == 2 {
        NumEnum335::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum335::C(a as u8 * 3)
    } else {
        NumEnum335::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_335(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers335 {
    Numbers335 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum336 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers336 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_336(a: u32) -> NumEnum336 {
    if a == 1 {
        NumEnum336::A(a)
    } else if a == 2 {
        NumEnum336::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum336::C(a as u8 * 3)
    } else {
        NumEnum336::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_336(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers336 {
    Numbers336 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum337 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers337 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_337(a: u32) -> NumEnum337 {
    if a == 1 {
        NumEnum337::A(a)
    } else if a == 2 {
        NumEnum337::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum337::C(a as u8 * 3)
    } else {
        NumEnum337::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_337(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers337 {
    Numbers337 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum338 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers338 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_338(a: u32) -> NumEnum338 {
    if a == 1 {
        NumEnum338::A(a)
    } else if a == 2 {
        NumEnum338::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum338::C(a as u8 * 3)
    } else {
        NumEnum338::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_338(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers338 {
    Numbers338 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum339 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers339 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_339(a: u32) -> NumEnum339 {
    if a == 1 {
        NumEnum339::A(a)
    } else if a == 2 {
        NumEnum339::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum339::C(a as u8 * 3)
    } else {
        NumEnum339::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_339(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers339 {
    Numbers339 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum340 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers340 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_340(a: u32) -> NumEnum340 {
    if a == 1 {
        NumEnum340::A(a)
    } else if a == 2 {
        NumEnum340::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum340::C(a as u8 * 3)
    } else {
        NumEnum340::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_340(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers340 {
    Numbers340 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum341 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers341 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_341(a: u32) -> NumEnum341 {
    if a == 1 {
        NumEnum341::A(a)
    } else if a == 2 {
        NumEnum341::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum341::C(a as u8 * 3)
    } else {
        NumEnum341::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_341(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers341 {
    Numbers341 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum342 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers342 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_342(a: u32) -> NumEnum342 {
    if a == 1 {
        NumEnum342::A(a)
    } else if a == 2 {
        NumEnum342::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum342::C(a as u8 * 3)
    } else {
        NumEnum342::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_342(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers342 {
    Numbers342 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum343 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers343 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_343(a: u32) -> NumEnum343 {
    if a == 1 {
        NumEnum343::A(a)
    } else if a == 2 {
        NumEnum343::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum343::C(a as u8 * 3)
    } else {
        NumEnum343::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_343(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers343 {
    Numbers343 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum344 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers344 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_344(a: u32) -> NumEnum344 {
    if a == 1 {
        NumEnum344::A(a)
    } else if a == 2 {
        NumEnum344::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum344::C(a as u8 * 3)
    } else {
        NumEnum344::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_344(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers344 {
    Numbers344 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum345 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers345 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_345(a: u32) -> NumEnum345 {
    if a == 1 {
        NumEnum345::A(a)
    } else if a == 2 {
        NumEnum345::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum345::C(a as u8 * 3)
    } else {
        NumEnum345::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_345(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers345 {
    Numbers345 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum346 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers346 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_346(a: u32) -> NumEnum346 {
    if a == 1 {
        NumEnum346::A(a)
    } else if a == 2 {
        NumEnum346::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum346::C(a as u8 * 3)
    } else {
        NumEnum346::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_346(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers346 {
    Numbers346 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum347 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers347 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_347(a: u32) -> NumEnum347 {
    if a == 1 {
        NumEnum347::A(a)
    } else if a == 2 {
        NumEnum347::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum347::C(a as u8 * 3)
    } else {
        NumEnum347::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_347(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers347 {
    Numbers347 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum348 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers348 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_348(a: u32) -> NumEnum348 {
    if a == 1 {
        NumEnum348::A(a)
    } else if a == 2 {
        NumEnum348::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum348::C(a as u8 * 3)
    } else {
        NumEnum348::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_348(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers348 {
    Numbers348 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum349 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers349 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_349(a: u32) -> NumEnum349 {
    if a == 1 {
        NumEnum349::A(a)
    } else if a == 2 {
        NumEnum349::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum349::C(a as u8 * 3)
    } else {
        NumEnum349::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_349(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers349 {
    Numbers349 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum350 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers350 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_350(a: u32) -> NumEnum350 {
    if a == 1 {
        NumEnum350::A(a)
    } else if a == 2 {
        NumEnum350::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum350::C(a as u8 * 3)
    } else {
        NumEnum350::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_350(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers350 {
    Numbers350 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum351 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers351 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_351(a: u32) -> NumEnum351 {
    if a == 1 {
        NumEnum351::A(a)
    } else if a == 2 {
        NumEnum351::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum351::C(a as u8 * 3)
    } else {
        NumEnum351::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_351(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers351 {
    Numbers351 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum352 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers352 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_352(a: u32) -> NumEnum352 {
    if a == 1 {
        NumEnum352::A(a)
    } else if a == 2 {
        NumEnum352::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum352::C(a as u8 * 3)
    } else {
        NumEnum352::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_352(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers352 {
    Numbers352 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum353 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers353 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_353(a: u32) -> NumEnum353 {
    if a == 1 {
        NumEnum353::A(a)
    } else if a == 2 {
        NumEnum353::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum353::C(a as u8 * 3)
    } else {
        NumEnum353::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_353(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers353 {
    Numbers353 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum354 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers354 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_354(a: u32) -> NumEnum354 {
    if a == 1 {
        NumEnum354::A(a)
    } else if a == 2 {
        NumEnum354::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum354::C(a as u8 * 3)
    } else {
        NumEnum354::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_354(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers354 {
    Numbers354 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum355 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers355 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_355(a: u32) -> NumEnum355 {
    if a == 1 {
        NumEnum355::A(a)
    } else if a == 2 {
        NumEnum355::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum355::C(a as u8 * 3)
    } else {
        NumEnum355::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_355(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers355 {
    Numbers355 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum356 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers356 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_356(a: u32) -> NumEnum356 {
    if a == 1 {
        NumEnum356::A(a)
    } else if a == 2 {
        NumEnum356::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum356::C(a as u8 * 3)
    } else {
        NumEnum356::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_356(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers356 {
    Numbers356 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum357 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers357 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_357(a: u32) -> NumEnum357 {
    if a == 1 {
        NumEnum357::A(a)
    } else if a == 2 {
        NumEnum357::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum357::C(a as u8 * 3)
    } else {
        NumEnum357::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_357(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers357 {
    Numbers357 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum358 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers358 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_358(a: u32) -> NumEnum358 {
    if a == 1 {
        NumEnum358::A(a)
    } else if a == 2 {
        NumEnum358::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum358::C(a as u8 * 3)
    } else {
        NumEnum358::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_358(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers358 {
    Numbers358 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum359 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers359 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_359(a: u32) -> NumEnum359 {
    if a == 1 {
        NumEnum359::A(a)
    } else if a == 2 {
        NumEnum359::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum359::C(a as u8 * 3)
    } else {
        NumEnum359::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_359(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers359 {
    Numbers359 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum360 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers360 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_360(a: u32) -> NumEnum360 {
    if a == 1 {
        NumEnum360::A(a)
    } else if a == 2 {
        NumEnum360::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum360::C(a as u8 * 3)
    } else {
        NumEnum360::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_360(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers360 {
    Numbers360 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum361 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers361 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_361(a: u32) -> NumEnum361 {
    if a == 1 {
        NumEnum361::A(a)
    } else if a == 2 {
        NumEnum361::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum361::C(a as u8 * 3)
    } else {
        NumEnum361::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_361(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers361 {
    Numbers361 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum362 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers362 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_362(a: u32) -> NumEnum362 {
    if a == 1 {
        NumEnum362::A(a)
    } else if a == 2 {
        NumEnum362::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum362::C(a as u8 * 3)
    } else {
        NumEnum362::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_362(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers362 {
    Numbers362 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum363 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers363 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_363(a: u32) -> NumEnum363 {
    if a == 1 {
        NumEnum363::A(a)
    } else if a == 2 {
        NumEnum363::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum363::C(a as u8 * 3)
    } else {
        NumEnum363::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_363(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers363 {
    Numbers363 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum364 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers364 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_364(a: u32) -> NumEnum364 {
    if a == 1 {
        NumEnum364::A(a)
    } else if a == 2 {
        NumEnum364::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum364::C(a as u8 * 3)
    } else {
        NumEnum364::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_364(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers364 {
    Numbers364 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum365 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers365 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_365(a: u32) -> NumEnum365 {
    if a == 1 {
        NumEnum365::A(a)
    } else if a == 2 {
        NumEnum365::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum365::C(a as u8 * 3)
    } else {
        NumEnum365::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_365(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers365 {
    Numbers365 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum366 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers366 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_366(a: u32) -> NumEnum366 {
    if a == 1 {
        NumEnum366::A(a)
    } else if a == 2 {
        NumEnum366::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum366::C(a as u8 * 3)
    } else {
        NumEnum366::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_366(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers366 {
    Numbers366 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum367 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers367 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_367(a: u32) -> NumEnum367 {
    if a == 1 {
        NumEnum367::A(a)
    } else if a == 2 {
        NumEnum367::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum367::C(a as u8 * 3)
    } else {
        NumEnum367::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_367(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers367 {
    Numbers367 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum368 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers368 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_368(a: u32) -> NumEnum368 {
    if a == 1 {
        NumEnum368::A(a)
    } else if a == 2 {
        NumEnum368::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum368::C(a as u8 * 3)
    } else {
        NumEnum368::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_368(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers368 {
    Numbers368 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum369 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers369 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_369(a: u32) -> NumEnum369 {
    if a == 1 {
        NumEnum369::A(a)
    } else if a == 2 {
        NumEnum369::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum369::C(a as u8 * 3)
    } else {
        NumEnum369::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_369(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers369 {
    Numbers369 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum370 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers370 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_370(a: u32) -> NumEnum370 {
    if a == 1 {
        NumEnum370::A(a)
    } else if a == 2 {
        NumEnum370::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum370::C(a as u8 * 3)
    } else {
        NumEnum370::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_370(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers370 {
    Numbers370 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum371 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers371 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_371(a: u32) -> NumEnum371 {
    if a == 1 {
        NumEnum371::A(a)
    } else if a == 2 {
        NumEnum371::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum371::C(a as u8 * 3)
    } else {
        NumEnum371::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_371(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers371 {
    Numbers371 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum372 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers372 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_372(a: u32) -> NumEnum372 {
    if a == 1 {
        NumEnum372::A(a)
    } else if a == 2 {
        NumEnum372::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum372::C(a as u8 * 3)
    } else {
        NumEnum372::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_372(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers372 {
    Numbers372 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum373 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers373 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_373(a: u32) -> NumEnum373 {
    if a == 1 {
        NumEnum373::A(a)
    } else if a == 2 {
        NumEnum373::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum373::C(a as u8 * 3)
    } else {
        NumEnum373::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_373(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers373 {
    Numbers373 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum374 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers374 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_374(a: u32) -> NumEnum374 {
    if a == 1 {
        NumEnum374::A(a)
    } else if a == 2 {
        NumEnum374::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum374::C(a as u8 * 3)
    } else {
        NumEnum374::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_374(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers374 {
    Numbers374 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum375 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers375 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_375(a: u32) -> NumEnum375 {
    if a == 1 {
        NumEnum375::A(a)
    } else if a == 2 {
        NumEnum375::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum375::C(a as u8 * 3)
    } else {
        NumEnum375::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_375(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers375 {
    Numbers375 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum376 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers376 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_376(a: u32) -> NumEnum376 {
    if a == 1 {
        NumEnum376::A(a)
    } else if a == 2 {
        NumEnum376::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum376::C(a as u8 * 3)
    } else {
        NumEnum376::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_376(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers376 {
    Numbers376 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum377 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers377 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_377(a: u32) -> NumEnum377 {
    if a == 1 {
        NumEnum377::A(a)
    } else if a == 2 {
        NumEnum377::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum377::C(a as u8 * 3)
    } else {
        NumEnum377::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_377(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers377 {
    Numbers377 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum378 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers378 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_378(a: u32) -> NumEnum378 {
    if a == 1 {
        NumEnum378::A(a)
    } else if a == 2 {
        NumEnum378::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum378::C(a as u8 * 3)
    } else {
        NumEnum378::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_378(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers378 {
    Numbers378 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum379 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers379 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_379(a: u32) -> NumEnum379 {
    if a == 1 {
        NumEnum379::A(a)
    } else if a == 2 {
        NumEnum379::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum379::C(a as u8 * 3)
    } else {
        NumEnum379::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_379(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers379 {
    Numbers379 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum380 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers380 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_380(a: u32) -> NumEnum380 {
    if a == 1 {
        NumEnum380::A(a)
    } else if a == 2 {
        NumEnum380::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum380::C(a as u8 * 3)
    } else {
        NumEnum380::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_380(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers380 {
    Numbers380 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum381 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers381 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_381(a: u32) -> NumEnum381 {
    if a == 1 {
        NumEnum381::A(a)
    } else if a == 2 {
        NumEnum381::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum381::C(a as u8 * 3)
    } else {
        NumEnum381::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_381(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers381 {
    Numbers381 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum382 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers382 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_382(a: u32) -> NumEnum382 {
    if a == 1 {
        NumEnum382::A(a)
    } else if a == 2 {
        NumEnum382::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum382::C(a as u8 * 3)
    } else {
        NumEnum382::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_382(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers382 {
    Numbers382 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum383 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers383 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_383(a: u32) -> NumEnum383 {
    if a == 1 {
        NumEnum383::A(a)
    } else if a == 2 {
        NumEnum383::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum383::C(a as u8 * 3)
    } else {
        NumEnum383::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_383(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers383 {
    Numbers383 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum384 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers384 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_384(a: u32) -> NumEnum384 {
    if a == 1 {
        NumEnum384::A(a)
    } else if a == 2 {
        NumEnum384::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum384::C(a as u8 * 3)
    } else {
        NumEnum384::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_384(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers384 {
    Numbers384 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum385 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers385 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_385(a: u32) -> NumEnum385 {
    if a == 1 {
        NumEnum385::A(a)
    } else if a == 2 {
        NumEnum385::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum385::C(a as u8 * 3)
    } else {
        NumEnum385::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_385(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers385 {
    Numbers385 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum386 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers386 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_386(a: u32) -> NumEnum386 {
    if a == 1 {
        NumEnum386::A(a)
    } else if a == 2 {
        NumEnum386::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum386::C(a as u8 * 3)
    } else {
        NumEnum386::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_386(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers386 {
    Numbers386 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum387 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers387 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_387(a: u32) -> NumEnum387 {
    if a == 1 {
        NumEnum387::A(a)
    } else if a == 2 {
        NumEnum387::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum387::C(a as u8 * 3)
    } else {
        NumEnum387::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_387(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers387 {
    Numbers387 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum388 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers388 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_388(a: u32) -> NumEnum388 {
    if a == 1 {
        NumEnum388::A(a)
    } else if a == 2 {
        NumEnum388::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum388::C(a as u8 * 3)
    } else {
        NumEnum388::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_388(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers388 {
    Numbers388 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum389 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers389 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_389(a: u32) -> NumEnum389 {
    if a == 1 {
        NumEnum389::A(a)
    } else if a == 2 {
        NumEnum389::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum389::C(a as u8 * 3)
    } else {
        NumEnum389::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_389(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers389 {
    Numbers389 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum390 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers390 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_390(a: u32) -> NumEnum390 {
    if a == 1 {
        NumEnum390::A(a)
    } else if a == 2 {
        NumEnum390::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum390::C(a as u8 * 3)
    } else {
        NumEnum390::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_390(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers390 {
    Numbers390 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum391 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers391 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_391(a: u32) -> NumEnum391 {
    if a == 1 {
        NumEnum391::A(a)
    } else if a == 2 {
        NumEnum391::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum391::C(a as u8 * 3)
    } else {
        NumEnum391::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_391(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers391 {
    Numbers391 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum392 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers392 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_392(a: u32) -> NumEnum392 {
    if a == 1 {
        NumEnum392::A(a)
    } else if a == 2 {
        NumEnum392::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum392::C(a as u8 * 3)
    } else {
        NumEnum392::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_392(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers392 {
    Numbers392 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum393 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers393 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_393(a: u32) -> NumEnum393 {
    if a == 1 {
        NumEnum393::A(a)
    } else if a == 2 {
        NumEnum393::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum393::C(a as u8 * 3)
    } else {
        NumEnum393::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_393(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers393 {
    Numbers393 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum394 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers394 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_394(a: u32) -> NumEnum394 {
    if a == 1 {
        NumEnum394::A(a)
    } else if a == 2 {
        NumEnum394::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum394::C(a as u8 * 3)
    } else {
        NumEnum394::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_394(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers394 {
    Numbers394 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum395 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers395 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_395(a: u32) -> NumEnum395 {
    if a == 1 {
        NumEnum395::A(a)
    } else if a == 2 {
        NumEnum395::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum395::C(a as u8 * 3)
    } else {
        NumEnum395::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_395(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers395 {
    Numbers395 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum396 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers396 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_396(a: u32) -> NumEnum396 {
    if a == 1 {
        NumEnum396::A(a)
    } else if a == 2 {
        NumEnum396::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum396::C(a as u8 * 3)
    } else {
        NumEnum396::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_396(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers396 {
    Numbers396 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum397 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers397 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_397(a: u32) -> NumEnum397 {
    if a == 1 {
        NumEnum397::A(a)
    } else if a == 2 {
        NumEnum397::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum397::C(a as u8 * 3)
    } else {
        NumEnum397::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_397(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers397 {
    Numbers397 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum398 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers398 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_398(a: u32) -> NumEnum398 {
    if a == 1 {
        NumEnum398::A(a)
    } else if a == 2 {
        NumEnum398::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum398::C(a as u8 * 3)
    } else {
        NumEnum398::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_398(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers398 {
    Numbers398 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum399 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers399 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_399(a: u32) -> NumEnum399 {
    if a == 1 {
        NumEnum399::A(a)
    } else if a == 2 {
        NumEnum399::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum399::C(a as u8 * 3)
    } else {
        NumEnum399::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_399(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers399 {
    Numbers399 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum400 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers400 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_400(a: u32) -> NumEnum400 {
    if a == 1 {
        NumEnum400::A(a)
    } else if a == 2 {
        NumEnum400::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum400::C(a as u8 * 3)
    } else {
        NumEnum400::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_400(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers400 {
    Numbers400 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum401 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers401 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_401(a: u32) -> NumEnum401 {
    if a == 1 {
        NumEnum401::A(a)
    } else if a == 2 {
        NumEnum401::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum401::C(a as u8 * 3)
    } else {
        NumEnum401::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_401(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers401 {
    Numbers401 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum402 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers402 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_402(a: u32) -> NumEnum402 {
    if a == 1 {
        NumEnum402::A(a)
    } else if a == 2 {
        NumEnum402::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum402::C(a as u8 * 3)
    } else {
        NumEnum402::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_402(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers402 {
    Numbers402 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum403 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers403 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_403(a: u32) -> NumEnum403 {
    if a == 1 {
        NumEnum403::A(a)
    } else if a == 2 {
        NumEnum403::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum403::C(a as u8 * 3)
    } else {
        NumEnum403::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_403(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers403 {
    Numbers403 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum404 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers404 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_404(a: u32) -> NumEnum404 {
    if a == 1 {
        NumEnum404::A(a)
    } else if a == 2 {
        NumEnum404::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum404::C(a as u8 * 3)
    } else {
        NumEnum404::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_404(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers404 {
    Numbers404 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum405 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers405 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_405(a: u32) -> NumEnum405 {
    if a == 1 {
        NumEnum405::A(a)
    } else if a == 2 {
        NumEnum405::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum405::C(a as u8 * 3)
    } else {
        NumEnum405::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_405(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers405 {
    Numbers405 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum406 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers406 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_406(a: u32) -> NumEnum406 {
    if a == 1 {
        NumEnum406::A(a)
    } else if a == 2 {
        NumEnum406::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum406::C(a as u8 * 3)
    } else {
        NumEnum406::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_406(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers406 {
    Numbers406 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum407 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers407 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_407(a: u32) -> NumEnum407 {
    if a == 1 {
        NumEnum407::A(a)
    } else if a == 2 {
        NumEnum407::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum407::C(a as u8 * 3)
    } else {
        NumEnum407::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_407(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers407 {
    Numbers407 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum408 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers408 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_408(a: u32) -> NumEnum408 {
    if a == 1 {
        NumEnum408::A(a)
    } else if a == 2 {
        NumEnum408::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum408::C(a as u8 * 3)
    } else {
        NumEnum408::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_408(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers408 {
    Numbers408 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum409 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers409 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_409(a: u32) -> NumEnum409 {
    if a == 1 {
        NumEnum409::A(a)
    } else if a == 2 {
        NumEnum409::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum409::C(a as u8 * 3)
    } else {
        NumEnum409::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_409(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers409 {
    Numbers409 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum410 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers410 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_410(a: u32) -> NumEnum410 {
    if a == 1 {
        NumEnum410::A(a)
    } else if a == 2 {
        NumEnum410::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum410::C(a as u8 * 3)
    } else {
        NumEnum410::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_410(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers410 {
    Numbers410 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum411 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers411 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_411(a: u32) -> NumEnum411 {
    if a == 1 {
        NumEnum411::A(a)
    } else if a == 2 {
        NumEnum411::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum411::C(a as u8 * 3)
    } else {
        NumEnum411::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_411(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers411 {
    Numbers411 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum412 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers412 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_412(a: u32) -> NumEnum412 {
    if a == 1 {
        NumEnum412::A(a)
    } else if a == 2 {
        NumEnum412::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum412::C(a as u8 * 3)
    } else {
        NumEnum412::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_412(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers412 {
    Numbers412 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum413 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers413 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_413(a: u32) -> NumEnum413 {
    if a == 1 {
        NumEnum413::A(a)
    } else if a == 2 {
        NumEnum413::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum413::C(a as u8 * 3)
    } else {
        NumEnum413::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_413(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers413 {
    Numbers413 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum414 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers414 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_414(a: u32) -> NumEnum414 {
    if a == 1 {
        NumEnum414::A(a)
    } else if a == 2 {
        NumEnum414::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum414::C(a as u8 * 3)
    } else {
        NumEnum414::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_414(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers414 {
    Numbers414 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum415 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers415 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_415(a: u32) -> NumEnum415 {
    if a == 1 {
        NumEnum415::A(a)
    } else if a == 2 {
        NumEnum415::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum415::C(a as u8 * 3)
    } else {
        NumEnum415::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_415(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers415 {
    Numbers415 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum416 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers416 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_416(a: u32) -> NumEnum416 {
    if a == 1 {
        NumEnum416::A(a)
    } else if a == 2 {
        NumEnum416::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum416::C(a as u8 * 3)
    } else {
        NumEnum416::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_416(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers416 {
    Numbers416 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum417 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers417 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_417(a: u32) -> NumEnum417 {
    if a == 1 {
        NumEnum417::A(a)
    } else if a == 2 {
        NumEnum417::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum417::C(a as u8 * 3)
    } else {
        NumEnum417::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_417(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers417 {
    Numbers417 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum418 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers418 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_418(a: u32) -> NumEnum418 {
    if a == 1 {
        NumEnum418::A(a)
    } else if a == 2 {
        NumEnum418::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum418::C(a as u8 * 3)
    } else {
        NumEnum418::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_418(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers418 {
    Numbers418 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum419 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers419 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_419(a: u32) -> NumEnum419 {
    if a == 1 {
        NumEnum419::A(a)
    } else if a == 2 {
        NumEnum419::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum419::C(a as u8 * 3)
    } else {
        NumEnum419::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_419(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers419 {
    Numbers419 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum420 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers420 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_420(a: u32) -> NumEnum420 {
    if a == 1 {
        NumEnum420::A(a)
    } else if a == 2 {
        NumEnum420::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum420::C(a as u8 * 3)
    } else {
        NumEnum420::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_420(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers420 {
    Numbers420 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum421 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers421 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_421(a: u32) -> NumEnum421 {
    if a == 1 {
        NumEnum421::A(a)
    } else if a == 2 {
        NumEnum421::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum421::C(a as u8 * 3)
    } else {
        NumEnum421::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_421(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers421 {
    Numbers421 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum422 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers422 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_422(a: u32) -> NumEnum422 {
    if a == 1 {
        NumEnum422::A(a)
    } else if a == 2 {
        NumEnum422::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum422::C(a as u8 * 3)
    } else {
        NumEnum422::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_422(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers422 {
    Numbers422 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum423 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers423 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_423(a: u32) -> NumEnum423 {
    if a == 1 {
        NumEnum423::A(a)
    } else if a == 2 {
        NumEnum423::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum423::C(a as u8 * 3)
    } else {
        NumEnum423::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_423(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers423 {
    Numbers423 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum424 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers424 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_424(a: u32) -> NumEnum424 {
    if a == 1 {
        NumEnum424::A(a)
    } else if a == 2 {
        NumEnum424::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum424::C(a as u8 * 3)
    } else {
        NumEnum424::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_424(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers424 {
    Numbers424 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum425 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers425 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_425(a: u32) -> NumEnum425 {
    if a == 1 {
        NumEnum425::A(a)
    } else if a == 2 {
        NumEnum425::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum425::C(a as u8 * 3)
    } else {
        NumEnum425::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_425(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers425 {
    Numbers425 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum426 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers426 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_426(a: u32) -> NumEnum426 {
    if a == 1 {
        NumEnum426::A(a)
    } else if a == 2 {
        NumEnum426::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum426::C(a as u8 * 3)
    } else {
        NumEnum426::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_426(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers426 {
    Numbers426 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum427 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers427 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_427(a: u32) -> NumEnum427 {
    if a == 1 {
        NumEnum427::A(a)
    } else if a == 2 {
        NumEnum427::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum427::C(a as u8 * 3)
    } else {
        NumEnum427::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_427(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers427 {
    Numbers427 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum428 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers428 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_428(a: u32) -> NumEnum428 {
    if a == 1 {
        NumEnum428::A(a)
    } else if a == 2 {
        NumEnum428::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum428::C(a as u8 * 3)
    } else {
        NumEnum428::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_428(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers428 {
    Numbers428 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum429 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers429 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_429(a: u32) -> NumEnum429 {
    if a == 1 {
        NumEnum429::A(a)
    } else if a == 2 {
        NumEnum429::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum429::C(a as u8 * 3)
    } else {
        NumEnum429::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_429(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers429 {
    Numbers429 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum430 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers430 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_430(a: u32) -> NumEnum430 {
    if a == 1 {
        NumEnum430::A(a)
    } else if a == 2 {
        NumEnum430::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum430::C(a as u8 * 3)
    } else {
        NumEnum430::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_430(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers430 {
    Numbers430 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum431 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers431 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_431(a: u32) -> NumEnum431 {
    if a == 1 {
        NumEnum431::A(a)
    } else if a == 2 {
        NumEnum431::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum431::C(a as u8 * 3)
    } else {
        NumEnum431::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_431(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers431 {
    Numbers431 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum432 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers432 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_432(a: u32) -> NumEnum432 {
    if a == 1 {
        NumEnum432::A(a)
    } else if a == 2 {
        NumEnum432::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum432::C(a as u8 * 3)
    } else {
        NumEnum432::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_432(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers432 {
    Numbers432 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum433 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers433 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_433(a: u32) -> NumEnum433 {
    if a == 1 {
        NumEnum433::A(a)
    } else if a == 2 {
        NumEnum433::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum433::C(a as u8 * 3)
    } else {
        NumEnum433::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_433(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers433 {
    Numbers433 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum434 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers434 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_434(a: u32) -> NumEnum434 {
    if a == 1 {
        NumEnum434::A(a)
    } else if a == 2 {
        NumEnum434::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum434::C(a as u8 * 3)
    } else {
        NumEnum434::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_434(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers434 {
    Numbers434 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum435 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers435 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_435(a: u32) -> NumEnum435 {
    if a == 1 {
        NumEnum435::A(a)
    } else if a == 2 {
        NumEnum435::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum435::C(a as u8 * 3)
    } else {
        NumEnum435::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_435(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers435 {
    Numbers435 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum436 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers436 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_436(a: u32) -> NumEnum436 {
    if a == 1 {
        NumEnum436::A(a)
    } else if a == 2 {
        NumEnum436::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum436::C(a as u8 * 3)
    } else {
        NumEnum436::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_436(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers436 {
    Numbers436 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum437 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers437 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_437(a: u32) -> NumEnum437 {
    if a == 1 {
        NumEnum437::A(a)
    } else if a == 2 {
        NumEnum437::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum437::C(a as u8 * 3)
    } else {
        NumEnum437::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_437(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers437 {
    Numbers437 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum438 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers438 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_438(a: u32) -> NumEnum438 {
    if a == 1 {
        NumEnum438::A(a)
    } else if a == 2 {
        NumEnum438::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum438::C(a as u8 * 3)
    } else {
        NumEnum438::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_438(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers438 {
    Numbers438 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum439 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers439 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_439(a: u32) -> NumEnum439 {
    if a == 1 {
        NumEnum439::A(a)
    } else if a == 2 {
        NumEnum439::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum439::C(a as u8 * 3)
    } else {
        NumEnum439::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_439(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers439 {
    Numbers439 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum440 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers440 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_440(a: u32) -> NumEnum440 {
    if a == 1 {
        NumEnum440::A(a)
    } else if a == 2 {
        NumEnum440::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum440::C(a as u8 * 3)
    } else {
        NumEnum440::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_440(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers440 {
    Numbers440 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum441 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers441 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_441(a: u32) -> NumEnum441 {
    if a == 1 {
        NumEnum441::A(a)
    } else if a == 2 {
        NumEnum441::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum441::C(a as u8 * 3)
    } else {
        NumEnum441::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_441(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers441 {
    Numbers441 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum442 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers442 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_442(a: u32) -> NumEnum442 {
    if a == 1 {
        NumEnum442::A(a)
    } else if a == 2 {
        NumEnum442::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum442::C(a as u8 * 3)
    } else {
        NumEnum442::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_442(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers442 {
    Numbers442 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum443 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers443 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_443(a: u32) -> NumEnum443 {
    if a == 1 {
        NumEnum443::A(a)
    } else if a == 2 {
        NumEnum443::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum443::C(a as u8 * 3)
    } else {
        NumEnum443::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_443(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers443 {
    Numbers443 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum444 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers444 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_444(a: u32) -> NumEnum444 {
    if a == 1 {
        NumEnum444::A(a)
    } else if a == 2 {
        NumEnum444::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum444::C(a as u8 * 3)
    } else {
        NumEnum444::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_444(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers444 {
    Numbers444 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum445 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers445 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_445(a: u32) -> NumEnum445 {
    if a == 1 {
        NumEnum445::A(a)
    } else if a == 2 {
        NumEnum445::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum445::C(a as u8 * 3)
    } else {
        NumEnum445::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_445(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers445 {
    Numbers445 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum446 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers446 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_446(a: u32) -> NumEnum446 {
    if a == 1 {
        NumEnum446::A(a)
    } else if a == 2 {
        NumEnum446::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum446::C(a as u8 * 3)
    } else {
        NumEnum446::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_446(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers446 {
    Numbers446 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum447 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers447 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_447(a: u32) -> NumEnum447 {
    if a == 1 {
        NumEnum447::A(a)
    } else if a == 2 {
        NumEnum447::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum447::C(a as u8 * 3)
    } else {
        NumEnum447::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_447(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers447 {
    Numbers447 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum448 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers448 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_448(a: u32) -> NumEnum448 {
    if a == 1 {
        NumEnum448::A(a)
    } else if a == 2 {
        NumEnum448::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum448::C(a as u8 * 3)
    } else {
        NumEnum448::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_448(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers448 {
    Numbers448 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum449 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers449 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_449(a: u32) -> NumEnum449 {
    if a == 1 {
        NumEnum449::A(a)
    } else if a == 2 {
        NumEnum449::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum449::C(a as u8 * 3)
    } else {
        NumEnum449::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_449(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers449 {
    Numbers449 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum450 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers450 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_450(a: u32) -> NumEnum450 {
    if a == 1 {
        NumEnum450::A(a)
    } else if a == 2 {
        NumEnum450::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum450::C(a as u8 * 3)
    } else {
        NumEnum450::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_450(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers450 {
    Numbers450 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum451 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers451 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_451(a: u32) -> NumEnum451 {
    if a == 1 {
        NumEnum451::A(a)
    } else if a == 2 {
        NumEnum451::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum451::C(a as u8 * 3)
    } else {
        NumEnum451::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_451(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers451 {
    Numbers451 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum452 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers452 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_452(a: u32) -> NumEnum452 {
    if a == 1 {
        NumEnum452::A(a)
    } else if a == 2 {
        NumEnum452::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum452::C(a as u8 * 3)
    } else {
        NumEnum452::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_452(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers452 {
    Numbers452 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum453 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers453 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_453(a: u32) -> NumEnum453 {
    if a == 1 {
        NumEnum453::A(a)
    } else if a == 2 {
        NumEnum453::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum453::C(a as u8 * 3)
    } else {
        NumEnum453::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_453(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers453 {
    Numbers453 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum454 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers454 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_454(a: u32) -> NumEnum454 {
    if a == 1 {
        NumEnum454::A(a)
    } else if a == 2 {
        NumEnum454::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum454::C(a as u8 * 3)
    } else {
        NumEnum454::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_454(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers454 {
    Numbers454 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum455 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers455 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_455(a: u32) -> NumEnum455 {
    if a == 1 {
        NumEnum455::A(a)
    } else if a == 2 {
        NumEnum455::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum455::C(a as u8 * 3)
    } else {
        NumEnum455::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_455(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers455 {
    Numbers455 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum456 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers456 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_456(a: u32) -> NumEnum456 {
    if a == 1 {
        NumEnum456::A(a)
    } else if a == 2 {
        NumEnum456::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum456::C(a as u8 * 3)
    } else {
        NumEnum456::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_456(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers456 {
    Numbers456 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum457 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers457 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_457(a: u32) -> NumEnum457 {
    if a == 1 {
        NumEnum457::A(a)
    } else if a == 2 {
        NumEnum457::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum457::C(a as u8 * 3)
    } else {
        NumEnum457::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_457(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers457 {
    Numbers457 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum458 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers458 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_458(a: u32) -> NumEnum458 {
    if a == 1 {
        NumEnum458::A(a)
    } else if a == 2 {
        NumEnum458::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum458::C(a as u8 * 3)
    } else {
        NumEnum458::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_458(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers458 {
    Numbers458 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum459 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers459 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_459(a: u32) -> NumEnum459 {
    if a == 1 {
        NumEnum459::A(a)
    } else if a == 2 {
        NumEnum459::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum459::C(a as u8 * 3)
    } else {
        NumEnum459::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_459(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers459 {
    Numbers459 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum460 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers460 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_460(a: u32) -> NumEnum460 {
    if a == 1 {
        NumEnum460::A(a)
    } else if a == 2 {
        NumEnum460::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum460::C(a as u8 * 3)
    } else {
        NumEnum460::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_460(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers460 {
    Numbers460 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum461 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers461 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_461(a: u32) -> NumEnum461 {
    if a == 1 {
        NumEnum461::A(a)
    } else if a == 2 {
        NumEnum461::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum461::C(a as u8 * 3)
    } else {
        NumEnum461::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_461(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers461 {
    Numbers461 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum462 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers462 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_462(a: u32) -> NumEnum462 {
    if a == 1 {
        NumEnum462::A(a)
    } else if a == 2 {
        NumEnum462::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum462::C(a as u8 * 3)
    } else {
        NumEnum462::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_462(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers462 {
    Numbers462 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum463 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers463 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_463(a: u32) -> NumEnum463 {
    if a == 1 {
        NumEnum463::A(a)
    } else if a == 2 {
        NumEnum463::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum463::C(a as u8 * 3)
    } else {
        NumEnum463::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_463(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers463 {
    Numbers463 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum464 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers464 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_464(a: u32) -> NumEnum464 {
    if a == 1 {
        NumEnum464::A(a)
    } else if a == 2 {
        NumEnum464::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum464::C(a as u8 * 3)
    } else {
        NumEnum464::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_464(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers464 {
    Numbers464 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum465 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers465 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_465(a: u32) -> NumEnum465 {
    if a == 1 {
        NumEnum465::A(a)
    } else if a == 2 {
        NumEnum465::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum465::C(a as u8 * 3)
    } else {
        NumEnum465::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_465(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers465 {
    Numbers465 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum466 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers466 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_466(a: u32) -> NumEnum466 {
    if a == 1 {
        NumEnum466::A(a)
    } else if a == 2 {
        NumEnum466::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum466::C(a as u8 * 3)
    } else {
        NumEnum466::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_466(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers466 {
    Numbers466 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum467 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers467 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_467(a: u32) -> NumEnum467 {
    if a == 1 {
        NumEnum467::A(a)
    } else if a == 2 {
        NumEnum467::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum467::C(a as u8 * 3)
    } else {
        NumEnum467::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_467(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers467 {
    Numbers467 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum468 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers468 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_468(a: u32) -> NumEnum468 {
    if a == 1 {
        NumEnum468::A(a)
    } else if a == 2 {
        NumEnum468::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum468::C(a as u8 * 3)
    } else {
        NumEnum468::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_468(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers468 {
    Numbers468 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum469 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers469 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_469(a: u32) -> NumEnum469 {
    if a == 1 {
        NumEnum469::A(a)
    } else if a == 2 {
        NumEnum469::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum469::C(a as u8 * 3)
    } else {
        NumEnum469::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_469(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers469 {
    Numbers469 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum470 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers470 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_470(a: u32) -> NumEnum470 {
    if a == 1 {
        NumEnum470::A(a)
    } else if a == 2 {
        NumEnum470::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum470::C(a as u8 * 3)
    } else {
        NumEnum470::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_470(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers470 {
    Numbers470 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum471 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers471 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_471(a: u32) -> NumEnum471 {
    if a == 1 {
        NumEnum471::A(a)
    } else if a == 2 {
        NumEnum471::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum471::C(a as u8 * 3)
    } else {
        NumEnum471::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_471(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers471 {
    Numbers471 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum472 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers472 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_472(a: u32) -> NumEnum472 {
    if a == 1 {
        NumEnum472::A(a)
    } else if a == 2 {
        NumEnum472::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum472::C(a as u8 * 3)
    } else {
        NumEnum472::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_472(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers472 {
    Numbers472 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum473 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers473 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_473(a: u32) -> NumEnum473 {
    if a == 1 {
        NumEnum473::A(a)
    } else if a == 2 {
        NumEnum473::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum473::C(a as u8 * 3)
    } else {
        NumEnum473::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_473(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers473 {
    Numbers473 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum474 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers474 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_474(a: u32) -> NumEnum474 {
    if a == 1 {
        NumEnum474::A(a)
    } else if a == 2 {
        NumEnum474::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum474::C(a as u8 * 3)
    } else {
        NumEnum474::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_474(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers474 {
    Numbers474 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum475 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers475 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_475(a: u32) -> NumEnum475 {
    if a == 1 {
        NumEnum475::A(a)
    } else if a == 2 {
        NumEnum475::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum475::C(a as u8 * 3)
    } else {
        NumEnum475::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_475(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers475 {
    Numbers475 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum476 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers476 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_476(a: u32) -> NumEnum476 {
    if a == 1 {
        NumEnum476::A(a)
    } else if a == 2 {
        NumEnum476::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum476::C(a as u8 * 3)
    } else {
        NumEnum476::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_476(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers476 {
    Numbers476 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum477 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers477 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_477(a: u32) -> NumEnum477 {
    if a == 1 {
        NumEnum477::A(a)
    } else if a == 2 {
        NumEnum477::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum477::C(a as u8 * 3)
    } else {
        NumEnum477::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_477(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers477 {
    Numbers477 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum478 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers478 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_478(a: u32) -> NumEnum478 {
    if a == 1 {
        NumEnum478::A(a)
    } else if a == 2 {
        NumEnum478::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum478::C(a as u8 * 3)
    } else {
        NumEnum478::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_478(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers478 {
    Numbers478 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum479 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers479 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_479(a: u32) -> NumEnum479 {
    if a == 1 {
        NumEnum479::A(a)
    } else if a == 2 {
        NumEnum479::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum479::C(a as u8 * 3)
    } else {
        NumEnum479::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_479(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers479 {
    Numbers479 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum480 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers480 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_480(a: u32) -> NumEnum480 {
    if a == 1 {
        NumEnum480::A(a)
    } else if a == 2 {
        NumEnum480::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum480::C(a as u8 * 3)
    } else {
        NumEnum480::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_480(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers480 {
    Numbers480 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum481 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers481 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_481(a: u32) -> NumEnum481 {
    if a == 1 {
        NumEnum481::A(a)
    } else if a == 2 {
        NumEnum481::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum481::C(a as u8 * 3)
    } else {
        NumEnum481::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_481(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers481 {
    Numbers481 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum482 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers482 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_482(a: u32) -> NumEnum482 {
    if a == 1 {
        NumEnum482::A(a)
    } else if a == 2 {
        NumEnum482::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum482::C(a as u8 * 3)
    } else {
        NumEnum482::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_482(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers482 {
    Numbers482 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum483 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers483 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_483(a: u32) -> NumEnum483 {
    if a == 1 {
        NumEnum483::A(a)
    } else if a == 2 {
        NumEnum483::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum483::C(a as u8 * 3)
    } else {
        NumEnum483::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_483(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers483 {
    Numbers483 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum484 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers484 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_484(a: u32) -> NumEnum484 {
    if a == 1 {
        NumEnum484::A(a)
    } else if a == 2 {
        NumEnum484::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum484::C(a as u8 * 3)
    } else {
        NumEnum484::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_484(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers484 {
    Numbers484 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum485 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers485 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_485(a: u32) -> NumEnum485 {
    if a == 1 {
        NumEnum485::A(a)
    } else if a == 2 {
        NumEnum485::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum485::C(a as u8 * 3)
    } else {
        NumEnum485::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_485(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers485 {
    Numbers485 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum486 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers486 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_486(a: u32) -> NumEnum486 {
    if a == 1 {
        NumEnum486::A(a)
    } else if a == 2 {
        NumEnum486::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum486::C(a as u8 * 3)
    } else {
        NumEnum486::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_486(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers486 {
    Numbers486 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum487 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers487 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_487(a: u32) -> NumEnum487 {
    if a == 1 {
        NumEnum487::A(a)
    } else if a == 2 {
        NumEnum487::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum487::C(a as u8 * 3)
    } else {
        NumEnum487::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_487(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers487 {
    Numbers487 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum488 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers488 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_488(a: u32) -> NumEnum488 {
    if a == 1 {
        NumEnum488::A(a)
    } else if a == 2 {
        NumEnum488::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum488::C(a as u8 * 3)
    } else {
        NumEnum488::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_488(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers488 {
    Numbers488 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum489 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers489 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_489(a: u32) -> NumEnum489 {
    if a == 1 {
        NumEnum489::A(a)
    } else if a == 2 {
        NumEnum489::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum489::C(a as u8 * 3)
    } else {
        NumEnum489::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_489(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers489 {
    Numbers489 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum490 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers490 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_490(a: u32) -> NumEnum490 {
    if a == 1 {
        NumEnum490::A(a)
    } else if a == 2 {
        NumEnum490::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum490::C(a as u8 * 3)
    } else {
        NumEnum490::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_490(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers490 {
    Numbers490 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum491 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers491 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_491(a: u32) -> NumEnum491 {
    if a == 1 {
        NumEnum491::A(a)
    } else if a == 2 {
        NumEnum491::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum491::C(a as u8 * 3)
    } else {
        NumEnum491::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_491(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers491 {
    Numbers491 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum492 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers492 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_492(a: u32) -> NumEnum492 {
    if a == 1 {
        NumEnum492::A(a)
    } else if a == 2 {
        NumEnum492::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum492::C(a as u8 * 3)
    } else {
        NumEnum492::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_492(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers492 {
    Numbers492 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum493 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers493 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_493(a: u32) -> NumEnum493 {
    if a == 1 {
        NumEnum493::A(a)
    } else if a == 2 {
        NumEnum493::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum493::C(a as u8 * 3)
    } else {
        NumEnum493::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_493(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers493 {
    Numbers493 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum494 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers494 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_494(a: u32) -> NumEnum494 {
    if a == 1 {
        NumEnum494::A(a)
    } else if a == 2 {
        NumEnum494::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum494::C(a as u8 * 3)
    } else {
        NumEnum494::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_494(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers494 {
    Numbers494 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum495 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers495 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_495(a: u32) -> NumEnum495 {
    if a == 1 {
        NumEnum495::A(a)
    } else if a == 2 {
        NumEnum495::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum495::C(a as u8 * 3)
    } else {
        NumEnum495::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_495(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers495 {
    Numbers495 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum496 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers496 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_496(a: u32) -> NumEnum496 {
    if a == 1 {
        NumEnum496::A(a)
    } else if a == 2 {
        NumEnum496::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum496::C(a as u8 * 3)
    } else {
        NumEnum496::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_496(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers496 {
    Numbers496 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum497 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers497 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_497(a: u32) -> NumEnum497 {
    if a == 1 {
        NumEnum497::A(a)
    } else if a == 2 {
        NumEnum497::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum497::C(a as u8 * 3)
    } else {
        NumEnum497::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_497(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers497 {
    Numbers497 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum498 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers498 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_498(a: u32) -> NumEnum498 {
    if a == 1 {
        NumEnum498::A(a)
    } else if a == 2 {
        NumEnum498::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum498::C(a as u8 * 3)
    } else {
        NumEnum498::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_498(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers498 {
    Numbers498 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
#[derive(Debug)]
pub enum NumEnum499 {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers499 {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum_499(a: u32) -> NumEnum499 {
    if a == 1 {
        NumEnum499::A(a)
    } else if a == 2 {
        NumEnum499::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum499::C(a as u8 * 3)
    } else {
        NumEnum499::D(a as i32, a as i32 * 2)
    }
}

#[no_mangle]
pub extern fn numbers_499(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers499 {
    Numbers499 {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}
