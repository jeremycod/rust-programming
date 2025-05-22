use std::f64::consts::PI;

enum Shape {
    Circle {radius: f64},
    Rectangle {width: f64, height: f64},
    Triangle {base: f64, height: f64}
}
fn calculate_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle {radius } => PI * radius * radius,
        Shape::Rectangle {width, height}=> width * height,
        Shape::Triangle {base, height}=> 0.5 * base * height
    }
}
fn main () {
    let triangle = Shape::Triangle {
        base: 34.68,
        height: 14.21
    };
    println!("Triangle area: {}", calculate_area(&triangle));

}