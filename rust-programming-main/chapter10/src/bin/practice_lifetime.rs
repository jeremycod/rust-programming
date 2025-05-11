struct MyStruct<'a> {
    value: &'a i32,
}


fn get_max<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main(){
    let x = 10;
    let my_struct = MyStruct { value: &x };
    println!("The value is: {}", my_struct.value);

}