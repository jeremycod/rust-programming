
fn add_numbers() -> impl Fn(i32, i32) -> i32 {
    |x, y| x + y
}
fn calculate_product() {{}
    let multiplier = 3;
    let multiplicator = |num| multiplier * num;
    println!("product: {}", multiplicator(4));
}

fn closure_traits(){
    let mut message = String::new();
    let closureA = |s: &str| message.push_str(s);
}
fn main() {
    println!("Practice closures");
    // Basic Closure
    let result = add_numbers()(5, 10);
    println!("Result: {}", result);
    // Closure Capturing Environment
    calculate_product();
}