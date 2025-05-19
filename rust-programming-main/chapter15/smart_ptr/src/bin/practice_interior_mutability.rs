use std::cell::Cell;
use std::cell::RefCell;

fn use_cell(){
    let c = Cell::new(5);
    let shared_c = &c; // Immutable reference
    let shared_d = &c;
    shared_c.set(10); // Modifying the value through the immutable reference


    shared_d.set(11);
    println!("Value in c: {}", shared_c.get()); // Output: 11
    println!("Value in d: {}", shared_d.get()); // Output: 11 
}
fn use_ref_cell(){
    let shared_number = RefCell::new(5);

    // Immutable borrows
    let borrow1 = shared_number.borrow();
    let borrow2 = shared_number.borrow();
    println!("Immutable borrows: {} and {}", *borrow1, *borrow2); // Output: 5 and 5
    drop(borrow1);
    drop(borrow2);

    // Mutable borrow
    let mut mutable_borrow = shared_number.borrow_mut();
    *mutable_borrow += 5;
    drop(mutable_borrow);


    let final_borrow = shared_number.borrow();
    println!("Final value: {}", *final_borrow); // Output: 10

    // The following would panic at runtime:
     // let _borrow3 = shared_number.borrow();
    //  let mut _mutable_borrow2 = shared_number.borrow_mut(); // Panic!
}
fn main(){
    use_cell();
    use_ref_cell();
}