fn main() {
    println!("{}", fibo(5));
    println!("{}", to_fahrenheit(10.0));
    println!("{}", to_fahrenheit(0.0));
}

fn fibo(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn to_fahrenheit(celsius: f32) -> f32 {
    celsius * 1.8 + 32.0
}
