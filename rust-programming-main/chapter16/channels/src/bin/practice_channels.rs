use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread::JoinHandle;
use std::time::Duration;

fn create_worker(i: i32, receiver: Arc<Mutex<Receiver<i32>>>) -> JoinHandle<()> {
    println!("create worker {}", i);
    let worker = thread::spawn(move || {
        while let Ok(received) = receiver.lock().unwrap().recv() {
            println!("Worker {} got: {:?}", i, received);
        }
    });
    worker
}
fn main(){
    let (sender, receiver) = mpsc::channel();
    let rec = Arc::new(Mutex::new(receiver));
    let producer = thread::spawn(move ||{
        for num in 1..=10 {
            sender.send(num).unwrap();
        }
    });
    let mut workers = vec![];
    for i in 1..=3 {
        let receiver_clone = Arc::clone(&rec);
        workers.push(create_worker(i, receiver_clone));
    }

    for worker in workers {
        worker.join().unwrap();
    }
    thread::sleep(Duration::from_millis(1000));
    producer.join().unwrap();
}