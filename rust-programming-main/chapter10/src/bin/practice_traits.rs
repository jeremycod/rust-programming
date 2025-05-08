use std::f64::consts::PI;
use std::fmt::Debug;

fn display_shape_info<T>(shape: &T)
where T: Shape + Debug {
    println!("Debug: {:?}, {}", shape, shape.area());
}
fn get_perimeter_plus_one(shape: &impl Shape) -> f64 {
    shape.perimeter() + 1.0
}
fn get_area<T: Shape>(shape: &T) -> f64 {
    shape.area()
}
fn print_shape_info(shape: &dyn Shape){
    println!("Shape area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}

trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64
}
#[derive(Debug)]
struct Circle {
    radius: f64
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        PI * self.radius
    }
}
fn main () {
    let rec = Rectangle{
        width: 3.56,
        height: 5.56
    };
    let cir = Circle {
        radius: 3.15
    };
    print_shape_info(&rec);
    print_shape_info(&cir);
    let shapes: Vec<&dyn Shape> = vec![&rec, &cir];
    for s in shapes {
        print_shape_info(s);
    }
    println!("shape area: {}", get_area(&rec));
    println!("perimeter plus one: {}", get_perimeter_plus_one(&rec));
    display_shape_info(&rec);
}