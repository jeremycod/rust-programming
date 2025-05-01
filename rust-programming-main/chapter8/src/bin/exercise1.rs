fn main() {
    let mut v = vec![1, 3, 7, 9, 5, 11];
    v.sort();
    println!("{}", v[v.len() / 2]);
    println!("{}", v.iter().sum::<i32>() / v.len() as i32)
}
