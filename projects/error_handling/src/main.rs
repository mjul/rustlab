use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};


fn main() {

    recoverable_errors();

    // game over
    panic!("crash and burn");
}


fn recoverable_errors() {
    open_unknown_file();
    match_errors();
    match_and_unwrap_expect();

    // Errors may propagate by returning Result<T,E> 
    let r = read_username_from_file();
    match r {
        Ok(_) => println!("read_username_from_file: Ok"),
        Err(_) => println!("read_username_from_file: Err"),
    };

    // the ? operator is shorthand for handling Result values
    let r = read_username_from_file_with_question_mark_operator();
    match r {
        Ok(_) => println!("read_username_from_file_with_question_mark_operator: Ok"),
        Err(_) => println!("read_username_from_file_with_question_mark_operator: Err"),
    };

    let _ = read_username_from_file_shortened();
    let _ = read_username_from_file_shortest();

    let c = last_char_of_first_line("line 1\nline 2");
    println!("c = {}", c.unwrap());
}

fn open_unknown_file() {
    let f = File::open("does_not_exist.txt");
    match f {
        Ok(file) => println!("Opened file: {:?}", file),
        Err(error) => println!("Problem opening file {:?}", error),
    };
}

fn match_errors() {
    let fname = "hello.txt";
    let f = File::open(fname);

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(fname) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // apparently this is not the idiomatic style, 
    // closures are frequently used instead of match:
    //  .unwrap_or_else(|error| { ... })  
}

fn match_and_unwrap_expect() {
    let fname = "hello.txt";
    // unwrap panics if the result is not Ok (the Err result).
    let f = File::open(fname).unwrap();

    let g = File::open(fname).expect("My handy error message");
    println!("match_and_unwrap_expect done");
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


// note that errors have a From trait to convert errors into the return type
fn read_username_from_file_with_question_mark_operator() -> Result<String, io::Error> {
    // ? operator: if the value is Err the Err will be returned as the result
    let mut f = File::open("hello.txt")?; 
    // at this point we know that f is of the Ok(_) type, no match required
    let mut s = String::new(); 
    // return the error or continue
    f.read_to_string(&mut s)?;
    // no errors, return a good result
    Ok(s)
}


// note that errors have a From trait to convert errors into the return type
fn read_username_from_file_shortened() -> Result<String, io::Error> {
    let mut s = String::new(); 
    File::open("hello.txt")?.read_to_string(&mut s)?;
    // no errors, return a good result
    Ok(s)
}


fn read_username_from_file_shortest() -> Result<String, io::Error> {
    // there is a standard library function to do all of the above in one operation
    fs::read_to_string("hello.txt")
}


// the `?` operator can only be used in a function that returns `Result` or `Option` 
// (or another type that implements `FromResidual`)


fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}