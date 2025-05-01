struct Point<T, U> {
    x: T,
    y: U,
}

impl <T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl <T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}


impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p2 = Point { x: 5.0, y: 10.0 };
    let p3 = Point { x: 5, y: 10.4 };
    let p4 = Point { x: "Hello", y: 10.0 };
    println!("p.x = {}", p.x());
    println!("{}", p2.distance_from_origin());
    let p5 = p3.mixup(p4);
    println!("x = {}, y = {}", p5.x, p5.y);
}
