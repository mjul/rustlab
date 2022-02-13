fn main() {
    let number = 3;

    // if has no parens, and takes a Boolean expression
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // there is no implicit conversion to bool, so this does not work:
    // if number == 3 { ... }

    // cond may be expressed with if else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        // default case
        println!("number is not divisible by 4, 3, or 2");
    }

    // if may be used in let statements
    // there is a lot of syntactic "blood" to help the compiler
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // the arms of an if must have the same type
    // this is not possible
    // let number = if condition { 5 } else { "six" };

    // loop is the general loop construct
    // infinite loop:
    // loop { println!("again"); }

    // we can break out of a loop
    let mut count = 0;
    // loops can be named with loop labels
    'counting_up: loop {
        println!("count={}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // break out of inner-most loop
                break;
            }
            if count == 2 {
                // break out of the named outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Loop may be used to retry operations so they may return a value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of loop is: {}", result);

    // while is conditional looping
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!");

    // for is used for looping over a collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    // while is a bit cumbersome
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // for is simpler
    for element in a {
        println!("the value is: {}", element);
    }

    // rev reverses a range
    // a range is denoted (1..n), n is not included
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF");
}
