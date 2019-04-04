#[derive(Debug)]
pub enum NumEnum {
    A(u32),
    B(u16),
    C(u8),
    D(i32, i32),
}

#[derive(Debug)]
pub struct Numbers {
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64,
}

pub fn num_enum(a: u32) -> NumEnum {
    if a == 1 {
        NumEnum::A(a)
    } else if a == 2 {
        NumEnum::B(a as u16 * 2)
    } else if a == 3 {
        NumEnum::C(a as u8 * 3)
    } else {
        NumEnum::D(a as i32, a as i32 * 2)
    }
}

pub fn numbers(
    x_u8: u8,
    x_u16: u16,
    x_u32: u32,
    x_u64: u64,
    x_i8: i8,
    x_i16: i16,
    x_i32: i32,
    x_i64: i64) -> Numbers {
    Numbers {
        x_u8,
        x_u16,
        x_u32,
        x_u64,
        x_i8,
        x_i16,
        x_i32,
        x_i64,
    }
}

fn main() {
    let mut i = 0;
    while i < 100 {
        println!("num_enum {:?}", num_enum(i));
        i = i + 1;
        let x_u8 = 2 * i as u8;
        let x_u16 = 3 * i as u16;
        let x_u32 = 4 * i as u32;
        let x_u64 = 5 * i as u64;
        let x_i8 = i as i8;
        let x_i16 = 6 * i as i16;
        let x_i32 = 7 * i as i32;
        let x_i64 = 8 * i as i64;
        println!("Numbers: {:?}",
                 numbers(x_u8, x_u16, x_u32, x_u64, x_i8, x_i16, x_i32, x_i64));
    }
}
