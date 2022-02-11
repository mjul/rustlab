use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // let bindings are immutable by default, `mut` makes it a mutable variable
    // String is is a UTF-8 growable string (think StringBuffer or StringBuilder)
    // There is also a string primitive, str.
    let mut guess = String::new();

    // intermezzo: try out the string builder stuff
    guess.push_str("foo");
    guess.push_str("bar");
    println!("Pushed foo and bar: {}", guess);
    guess.clear();

    // back to the game:

    let secret_number: i64 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let parsed: i64 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        
        println!("Parsed guess: {}", parsed);

        match parsed.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!("You win!");
                break;
            }
        }
    }
}
