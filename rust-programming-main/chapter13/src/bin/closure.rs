use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn main() {
    generate_workout(10, 7);
    generate_workout(5, 12);
}

fn generate_workout(intensity: u32, random_nubmer: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_nubmer == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: HashMap<u32, u32>,
}

impl <T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    
    fn value(&mut self, arg: u32) -> u32 {
       if let Some(&v) = self.value.get(&arg){
           v
       } else {
           let v = (self.calculation)(arg);
           self.value.insert(arg,v);
           v
       }
    }
}
