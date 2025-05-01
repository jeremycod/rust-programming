use std::collections::HashMap;

fn main() {
    let vowel = HashMap::from([
        ("a", true),
        ("e", true),
        ("i", true),
        ("o", true),
        ("u", true),
    ]);
    
    let s = String::from("apple");
    let pig_latin = if vowel.contains_key(&s[..1]) {
        format!("{}-hay", s)
    } else {
        format!("{}-{}ay", &s[1..], &s[..1])
    };
    println!("{}", pig_latin);
}
