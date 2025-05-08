use std::ops::{Add, Sub, Div};
use std::fmt::Display;

#[derive(Debug)]
enum MyResult<T, E> {
    MyOk(T),
    MyErr(E)
}
struct Point<T> {
    x: T,
    y: T
}
impl <T: Display +  Copy + Add<Output = T> + Sub<Output = T> + Default> Point<T> {
    fn distance_from_origin(&self) -> Point<T> {
        Point {
            x: T::default() - self.x,
            y: T:: default() - self.y
        }
    }
}

fn divide<T: PartialEq + Add<Output = T> + Default + Div<Output = T>>(a: T, b: T) -> MyResult<T, String> {
    if b == T::default() {
        MyResult::MyErr("Division by zero".to_string())
    } else {
        MyResult::MyOk(a/b)
    }
}

fn find_max<T: Clone + Copy + PartialOrd>(slice: &mut [T]) -> Option<T> {
    let mut max_value = slice[0];
    for &item in slice.iter(){
        if max_value < item {
            max_value = item
        }
    }
    Some(max_value)
}
fn main(){
    println!("start");
    let mut input = vec![2,4,6,1,0,8];
    let result1 = find_max(input.as_mut_slice());
    match result1 {
        Some(value) => println!("{}", value),
        None => println!("No value found"),
    }
    let point1 = Point {x: 2, y: 4};
    let point2 = Point{x: 1.2, y: 5.6};
    let result1 = point1.distance_from_origin();
    println!("{}, {}", result1.x, result1.y);


    let result2 = point2.distance_from_origin();
    println!("{}, {}", result2.x, result2.y);


    let mut v1: Vec<i32> = vec![1,2,4,6];
    let mut v2: Vec<f64> = vec![2.1, 3.1, 5.2, 1.4];
    let result2 = find_max(v1.as_mut_slice());
    println!("find_max 2: {}", result2.unwrap());

    let result3 = find_max(v2.as_mut_slice());
    println!("find_max 3: {}", result3.unwrap());

    let result4 = divide(2, 4);
    println!("divide 4: {:?}", result4);

    let result5 = divide(4.3, 6.7);
    println!("divide 5: {:?}", result5);


}