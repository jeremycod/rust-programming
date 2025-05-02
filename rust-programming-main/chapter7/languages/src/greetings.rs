mod english {
    pub mod formal {
        pub fn greet() {
            println!("Good day!")
        }
    }
    pub fn hello() {
        println!("Hello!");
    }
}
mod spanish {
    pub mod informal {
        pub fn saludo() {
            println!("¿Qué pasa?")
        }
    }
    pub fn hola() {
        println!("Hola!");
    }
}

pub use english::hello;
pub use spanish::hola;
pub use spanish::informal;
pub use english::formal;