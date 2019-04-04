fn foo(mut x: u32) -> u32 {
    let mut sum = 0;
    while x > 11 {
        match x % 5 {
            0 => {
                sum += x * 10;
                x -= 2;
            }
            1 => {
                sum += x;
            }
            2 => {
                sum += x % 9;
            }
            3 => {
                sum += 9;
                x -= 10;
            }
            _ => {
                sum += 2;
            }
        }
        sum = sum % 37;
        x = x - 1;
    }
    sum
}

fn main() {
    let mut i = 0;
    let mut sum = 0;
    while i < 10000 {
        i = i + 1;
        sum = sum + foo(i);
    }
    println!("sum {:?}", sum);
}
