

#[cfg(test)]
mod tests_intro {
    // A passing test
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // A failing test
    #[test]
    // we will ignore this to have cleaner results
    #[ignore]
    fn failing_test () {
        panic!("Make this test fail.");
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

#[cfg(test)]
mod tests_rectangle {
    use super::*;
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // expected value is false, so negate
        assert!(!smaller.can_hold(&larger));
    }
}


pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}


pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests_error_messages {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // we can add a formatted error message for failures
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }
        Guess { value }
    }
}

#[cfg(test)]
mod tests_should_panic {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_with_error_message() {
        Guess::new(200);
    }
}




#[cfg(test)]
mod tests_using_result {

    // tests may use assert and panic! as above
    // or return a Result (Ok/Err) like this
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}