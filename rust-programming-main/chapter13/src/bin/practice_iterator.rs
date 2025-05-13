fn creating_iterators() {
    let string_vec = vec!["dog", "cat", "rabbit", "snake"];
    let int_array = [1, 2, 3, 4, 5, 6];
    let range_vec = [1..=10];

    for s in string_vec.iter() {
        println!("{}", s);
    }
    let int_sum: i32 = int_array.iter().sum();
    println!("SUM:{}", int_sum);
    for r in range_vec.iter() {
        for num in r.clone() {
            if num % 2 == 0 {
                println!("Even number: {}", num);
            }
        }
    }
}
fn iterator_adaptors() {
    let floating_numbers = vec![1.1, -2.2, 3.3, -4.4, 5.5];
    let abs_numbers: Vec<f64> = floating_numbers.iter().map(|n: &f64| n.abs()).collect();
    println!("agg: {:?}", abs_numbers);
    let names: Vec<&str> = vec!["abc", "efg", "klm", "ast", "mnt", "tsn", "act"];
    let a_names: Vec<&&str> = names.iter().filter(|&n| n.starts_with("a")).collect();
    println!("a names: {:?}", a_names);
    let int_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let first_three_numbers: Vec<i32> = int_numbers.iter().take(3).copied().collect();
    println!("first three: {:?}", first_three_numbers);
}
fn iterator_consumers() {
    let int_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let sum_of_integers: i32 = int_numbers.iter().sum();
    println!("sum of integers: {}", sum_of_integers);

    let vec_of_bools: Vec<bool> = vec![false, false, false, false];
    let any_true = vec_of_bools.iter().any(|&value| value);
    println!("any true: {}", any_true);

    let vec_of_string: Vec<&str> = vec![
        "apple",
        "banana",
        "error found",
        "cherry",
        "date",
        "error message",
        "fig",
    ];
    let first_error: &str = vec_of_string
        .iter()
        .find(|&w| w.contains("error"))
        .map_or("", |v| v);
    println!("first error: {}", first_error);
}
fn combining_adaptors_and_consumers() {
    let int_numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    let result1: i32 = int_numbers.iter().map(|&v| v * v).filter(|&v| v < 10).sum();
    println!("result 1: {:?}", result1);

    let vec_of_string: Vec<&str> = vec!["cat", "Dog", "Canada", "flower", "Japan", "tree", "United States"];
    let result2: Vec<String> = vec_of_string.iter().map(|&w| w.to_lowercase()).filter(|w| w.len() < 5).collect();
    println!("result 2: {:?}", result2);
}
fn ownership_and_borrowing(){
    let vec_of_string: Vec<&str> = vec!["cat", "Dog", "Canada", "flower", "Japan", "tree", "United States"];
    for word in vec_of_string.iter() {
        println!("word:{:?}", word);
    }
    let new_vector: Vec<&&str> = vec_of_string.iter().take(3).collect();
    println!("new vector: {:?}", new_vector);

    let new_vec_of_strings_iter = vec_of_string.into_iter();
    for word in new_vec_of_strings_iter {
        println!("word 2:{:?}", word);
    }
}
fn main() {
    creating_iterators();
    iterator_adaptors();
    iterator_consumers();
    combining_adaptors_and_consumers();
    ownership_and_borrowing();
}
