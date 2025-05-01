use std::collections::HashMap;
use std::io;

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let inputs: Vec<&str> = input.trim().split(' ').collect();
        
        departments
            .entry(inputs[3].to_string())
            .and_modify(|e| e.push(inputs[1].to_string()))
            .or_insert(vec![inputs[1].to_string()]);

        let mut v = vec![];
        
        for (_, value) in &departments {
            v.extend(value);
        }

        v.sort();

        println!("{:?}", v);
    }
}
