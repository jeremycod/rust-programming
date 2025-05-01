fn main() {
    let number = 3;

    if number < 5 {
        println!("number less than 5")
    } else {
        println!("number greater than 5")
    }

    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
