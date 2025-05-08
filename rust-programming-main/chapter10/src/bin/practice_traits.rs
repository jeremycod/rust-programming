use std::f64::consts::PI;
/*

 */

fn print_shape_info(shape: &dyn Shape){
    println!("Shape area: {}", shape.area());
    println!("Perimeter: {}", shape.perimeter());
}
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}
struct Rectangle {
    width: f64,
    height: f64
}
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
}