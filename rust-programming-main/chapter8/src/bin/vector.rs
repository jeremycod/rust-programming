fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
    
    println!("{:?}", vec![1, 2, 3]);
    println!("{:?}", v);
    println!("{:?}", &v[0]);

    match v.get(1) {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    let row = vec![
        SperadsheetCell::Int(3),
        SperadsheetCell::Text(String::from("blue")),
        SperadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}

#[derive(Debug)]
enum SperadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
