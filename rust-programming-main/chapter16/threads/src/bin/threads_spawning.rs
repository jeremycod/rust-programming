use std::thread;
use std::time::Duration;
use rand::{random, Rng};

fn main(){
    let threadA = thread::spawn(|| {

        let r_time:u64 = rand::thread_rng().gen_range(500..=1500);
        println!("threadA id: {:?}, sleep {}", thread::current().id(), r_time);
        thread::sleep(Duration::from_millis(r_time ));
    });
    let threadB = thread::spawn(|| {
        let r_time:u64 = rand::thread_rng().gen_range(500..=1500);
        println!("threadB id: {:?}, sleep {}", thread::current().id(), r_time);
        thread::sleep(Duration::from_millis(r_time ));
    });


    let title = String::from("sample");
    let threadC = thread::spawn(move || {
       println!("Title from spawned thread: {}", title); 
    });
    threadA.join().unwrap();
    threadB.join().unwrap();
    threadC.join().unwrap();
}