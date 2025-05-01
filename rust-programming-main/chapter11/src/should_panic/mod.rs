pub mod should_panic {
    pub struct Guess {
        value: u32,
    }
    
    
    impl Guess {
        pub fn new(value: u32) -> Guess {
            if value < 1 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            } else if value > 100 {
                panic!("Less than 100. got {}.", value);
            }
    
            Guess {
                value
            }
        }
    
        pub fn value(&self) -> u32 {
            self.value
        }
    }

    #[test]
    #[should_panic(expected = "Less than 100.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
