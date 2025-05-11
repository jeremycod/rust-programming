#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod area_tests { // First group of tests for the 'area' method
        use super::*;
        #[test]
        fn test_basic_area() {
            let rect = Rectangle::new(10, 5);
            assert_eq!(rect.area(), 50);
        }

        #[test]
        fn test_square_area() {
            let square = Rectangle::new(7, 7);
            assert_eq!(square.area(), 49);
        }
    }

    mod can_hold_tests { // Second group of tests for the 'can_hold' method
        use super::*;
        #[test]
        fn test_larger_can_hold_smaller() {
            let larger = Rectangle::new(8, 7);
            let smaller = Rectangle::new(5, 1);
            assert!(larger.can_hold(&smaller));
        }

        #[test]
        fn test_smaller_cannot_hold_larger() {
            let larger = Rectangle::new(8, 7);
            let smaller = Rectangle::new(5, 1);
            assert!(!smaller.can_hold(&larger));
        }

        #[test]
        fn test_cannot_hold_equal() {
            let rect1 = Rectangle::new(5, 5);
            let rect2 = Rectangle::new(5, 5);
            assert!(!rect1.can_hold(&rect2));
        }
    }
}