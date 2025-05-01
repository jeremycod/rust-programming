fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    println!("{}", format!("{}-{}-{}", s1, s2, s3));

    let hello = String::from("안녕하세요");
    println!("{}", &hello[0..3]);
    println!("{}", hello.len());

    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}
