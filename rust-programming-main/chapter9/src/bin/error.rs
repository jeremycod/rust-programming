use std::io;
use std::fs;
use std::fs::File;
use std::io::ErrorKind;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => {
                panic!("There was a problem opening the file: {:?}", other_error)
            }
        }
    };

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    read_username_from_file().expect("Failed to read username from file");

    let f = File::open("kuckjwi.txt")?;

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    Ok(fs::read_to_string("hello.txt")?)
}
