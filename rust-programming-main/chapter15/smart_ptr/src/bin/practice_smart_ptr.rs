use std::rc::Rc;
struct Data {
    value: i32,
}

impl Data {
    fn print_value(&self) {
        println!("The value is: {}", self.value);
    }
}
impl Drop for Data {
    fn drop(&mut self) {
        println!("Dropping data with value: {}", self.value);
    }
}

fn main() {
    let data = Box::new(Data { value: 10 });
    // Your code here
    println!("data: {}", data.value);
    let shared = Rc::new(Data { value: 42 });
    let shared_clone1 = Rc::clone(&shared);
    let shared_clone2 = Rc::clone(&shared);
    println!("Shared: {}", shared.value);
    println!("Shared 1: {}", shared_clone1.value);
    println!("Shared 2: {}", shared_clone2.value);
    println!("Strong count: {}",Rc::strong_count(&shared));
    
}