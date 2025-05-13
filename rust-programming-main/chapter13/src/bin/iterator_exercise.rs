fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let doubled_and_filtered = numbers.iter()
        .map(|&x| x * 2) // Doubles each number
        .filter(|&x| x > 4); // Keeps only numbers greater than 4

    for number in doubled_and_filtered {
        println!("Number: {}", number); // Prints 6, 8, 10
    }
}