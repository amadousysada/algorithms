fn fib(n: i32) -> i32 {
    match n {
        0..=1 => 1,
        _ => fib(n - 2) + fib(n - 1),
    }
}

fn main() {
    println!("{}", fib(10))
}