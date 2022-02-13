fn main() {
    println!("Hello, world!");

    another_function(42);
    print_labeled_measurement(5, 'h');

    // statements
    // note: underscore name marks the var as unused, avoiding a compiler warning
    let _x = 6;

    // statements return no values
    // function definitions (fn) are statements
    // let is a statement, hence it does not return a value

    // so you cannot do this:
    // let x = (let y = 6);

    // expressions evaluate to a value

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    // functions may return a value, unsurprisingly
    let two_squared = square(2);
    println!("two squared is: {}", two_squared);
    let three_squared = square_no_return(3);
    println!("three squared is: {}", three_squared);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

//  comma-separated parameters as expected
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// there is no inference on the signature
// unfortunately you have to write out the types
fn square(x: i64) -> i64 {
    // return is a statement or used for early returns
    return x * x;
}

fn square_no_return(x: i64) -> i64 {
    // functions also return the last expression
    // note, no semicolon in this case:
    x * x
}
