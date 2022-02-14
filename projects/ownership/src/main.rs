// Ownership axioms:
//
//    Each value in Rust has a variable that’s called its owner.
//    There can only be one owner at a time.
//    When the owner goes out of scope, the value will be dropped.

// Reference axioms
//
//    At any given time, you can have either one mutable reference or any number of immutable references.
//    References must always be valid.

fn main() {
    println!("4.1 What is ownership?");
    what_is_ownership();
    println!();

    println!("4.2 References and borrowing");
    references_and_borrowing();
    println!();

    println!("4.3 The slice type");
    the_slice_type();
}

fn what_is_ownership() {
    let mut s = String::from("hello"); // string builder, mutable
    s.push_str(", world!"); // append
    println!("{}", s);
    // s is automatically freed when it goes out of scope

    // This copies x and let-binds it to the _y variable
    let x = 5;
    let y = x;

    // Strings are different
    let s1 = String::from("hello");
    let s2 = s1;

    // A string is a stack-allocated value (ptr, len, capacity) and a heap buffer with the contents
    // the second assigment above copies s1 stack-struct to s2. The heap buffer is the same.
    // So, it is a shallow copy.
    //
    // the s2 = s1 assignment transfers ownership, so s2 is now the owner
    // s1 is no longer accessible (it 'moved' to s2)

    // This gives an error: "value borrowed here after move"
    // println!("s1: {}", s1);

    println!("s2: {}", s2);

    // If we create a deep copy, we get a new object with new ownership
    let deep_s2 = s2.clone();
    println!("s2: {}", s2);
    println!("deep s2: {}", deep_s2);

    // stack values are value types and always deep copy
    println!("x = {}, y = {}", x, y);

    // you may use the Copy trait on bespoke types to give them the same value semantics
    // more on that later when we get to traits

    let s = String::from("hello");
    takes_ownership(s);
    // s is no longer accessible

    let x = 5;
    makes_copy(x);
    // x is still accessible

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);

    println!("s1: {}", s1);
    // This would fail with "value borrowed after move"
    // println!("s2: {}", s2);
    println!("s3: {}", s3);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("String {} has length {}", s2, len);
}

fn takes_ownership(s: String) {
    println!("takes_ownership: {}", s);
}

fn makes_copy(x: i32) {
    println!("makes_copy: {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s // returning s moves it out to the caller
}

fn takes_and_gives_back(s: String) -> String {
    // this function is now owner of s
    s // returning s moves it out to the caller
}

// no type inference, you have to type out the signature
fn calculate_length(s: String) -> (String, usize) {
    let l = s.len();
    (s, l)
}

// ----------------------------------------------------------------

// https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html

fn references_and_borrowing() {
    let s1 = String::from("hello");

    // there is no function overloading in Rust,
    // so we have to invent a new name
    // & is the reference operator like in many other languages
    // (dereferencing is *)
    // references are guaranteed to point to non-null instances
    // they do not move the ownership
    let len = calculate_length_by_reference(&s1);

    // we can still use s1 since passing a reference does not move
    // the ownership
    println!("The length of '{}' is {}.", s1, len);

    // creating a reference is called "borrowing"
    let mut s = String::from("hello");
    // Now, we pass a reference including the ownership, by making it mutable
    // Note: there can only be one mutable reference to a datum at a time:
    change(&mut s);

    let r1 = &mut s;
    // This would fail: "second mutable borrow happens here"
    // let r2 = &mut s;

    // Dangling references are not possible
    //let reference_to_nothing = dangle();

    let _reference_to_something = dangle_not();
}

// the & indicates that s is a reference (i.e. no ownership)
fn calculate_length_by_reference(s: &String) -> usize {
    // since we don't have ownership we cannot modify it, so this would fail:
    // s.push_str("foo");

    s.len()
}

fn change(s: &mut String) {
    // we can change a mutable reference
    s.push_str("foo!");
}

// this fails
// " expected named lifetime parameter"
// &s is borrowed, but when s goes out of scope s is no more
//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn dangle_not() -> String {
    let s = String::from("hello");
    s
}

// ----------------------------------------------------------------

// https://doc.rust-lang.org/stable/book/ch04-03-slices.html

fn the_slice_type() {
    let s = String::from("hello world");

    // reference subsets of the string, "slices"
    // slices are nicely half-open, excluding the "end" value
    let hello = &s[0..5]; // {0,1,2,3,4}
    let world = &s[6..11];
    println!("hello/world {} {}", hello, world);

    // syntax sugar for 0-indexed slices
    let hello_sugar = &[..5];
    // syntax sugar for slice to end of string
    let world_sugar = &[6..];
    // syntax sugar for whole string
    let hello_world = [..];

    // let's find the first word
    let fw = first_word_of_String(&s); // this is an immutable borrow
    println!("First word: {}", fw);

    // now we have a slice of s
    // thus, we cannot make a "mutable borrow" of s
    // since it would possibly invalidate the slice

    // E.g. this mutable borrow is not possible:
    // s.clear();
    println!("First word of String: {}", fw);

    // string literals (str) are slices (immutable references)
    let foobar = "foo bar";
    // foobar has type &str, an immutable reference

    let fw = first_word_of_str_slice(foobar);
    println!("First word of literal &str: {}", fw);

    let fw = first_word_of_str_slice(&s);
    println!("First word of literal &String: {}", fw);

    // We can slice other types, too
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    // this slice has type &[i32]
    // it should match the array [2,3]
    assert_eq!(slice, &[2, 3]);
}

fn first_word_of_String(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // no spaces
    &s[..]
}

// by changing from &String to &str we take a string slice as input
// this way the function accepts both &Strings and string literals (&str).
fn first_word_of_str_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    // no spaces
    &s[..]
}
