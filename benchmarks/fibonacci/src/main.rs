fn fibonacci(x: u64) -> u64 {
    if x < 2 {
        return x
    } else {
        fibonacci(x - 1) + fibonacci(x - 2)
    }
}

fn main() {
    let n = 30;
    println!("fibonacci({:?}) = {:?}", n, fibonacci(n));
}
