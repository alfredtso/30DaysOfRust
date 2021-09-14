#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_holder_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 5,
            height: 4,
        };

        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[ignore]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was {}",
            result
            );
    }

    #[test]
    #[should_panic(expected = "Guess value must be smaller than 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_should_works() -> Result<(), String> {
        if 1 + 1 == 2 {
            println!("I am here!");
            Ok(())
        } else {
            Err(String::from("one plus one does not equal to two"))
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn greeting(name: &str) -> String {
    String::from("Hello")
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than 1, got {} instead", value);
        } else if value > 100 {
            panic!("Guess value must be smaller than 100, got {} instead", value);
        }

        Guess { value }
    }
}
