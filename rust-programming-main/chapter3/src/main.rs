fn main() {
    const MAX_POINTS: u32 = 100_000;

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("{}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_Four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);
    println!("The vaue of five() is: {}", five());
}

fn five() -> i32 {
    5
}
