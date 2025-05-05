use std::collections::HashMap;

fn main() {
    let mut ages = HashMap::new();
    ages.insert("Alice", 30);
    ages.insert("Bob", 25);
    ages.insert("Charlie", 35);

    println!("Bob is {} years old", ages.get("Bob").unwrap());
    match ages.get("David"){
        Some(age) => println!("David is {} years old", age),
        None => println!("David's age is not found")
    }

    println!("David is {} years old", ages.get("David").unwrap_or_else(|| &0));
    
    if ages.contains_key("Alice"){
        println!("Alice exists in HashMap");
    }
    
    ages.insert("Bob", 26);
    println!("Bob is now {} years old", ages.get("Bob").unwrap());
    
    if ages.contains_key("Charlie"){
        ages.remove("Charlie");
    }

}