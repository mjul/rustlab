fn main() {
    println!("Hello, world!");

    vectors();
    strings();
    hashmaps();
}

fn vectors() {
    // vectors are generic, so put a type annotation to specify the type
    let _v_explicit_type: Vec<i32> = Vec::new();

    // there is a vec! macro to instantiate vectors
    let _v_macro = vec![1, 2, 3];

    // strangely, vectors are mutable 
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    // the type of v is inferred to Vec<i32> since we call push with i32

    // everything is freed when the vectors go out of scope

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // vectors are 0-indexed, [] is the indexing operator
    println!("The third element is {}", third);

    // there is also a get method
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // let out_of_bounds = &v[42];
    // if we do this everything blows up:
    // thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 42', src\main.rs:34:26

    // v.get(42) returns an option and avoids this

    let first = &v[0]; // immutable borrow of v
    println!("The first element is: {}", first);
    
    // not possible: 
    // v.push(6); // mutable borrow of v

    println!("The first element is: {}", first);

    // iterating over the vector values

    // immutable borrows
    for i in &v {
        println!("{}", i);
    }
    
    // mutable borrows
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    // vecs for heterogeneous types using enums (vecs are homogenous)
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}


fn strings() {
    let mut s = String::new(); // type std::string::String
    let data = "initial contents"; // type &str
    let s = "initial contents".to_string(); // to_string produces a String

    let s = String::from("initial contents"); // String from literal (string slice, &str)

    // strings are unicode
    let smiley = String::from("😀");

    // String is mutable
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("String is now: {}", s);

    // the + operator concatenates strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // note that this takes ownership of s1 and immutably borrows s2
    let s3 = s1 + &s2;
    println!("s3 is now: {}", s3);


    // string formatting with format!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    println!("s: {}", s);

    // but only at char boundaries
    println!("sometimes we can take slices of strings: {}", &s[..3]);

    // these are the Unicode scalar values (not grapheme clusters or what we would call letters)
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}


fn hashmaps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // zip the teams and scores to form (team,score) pairs
    // then turn them into a k-v hash-map
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    for kv in scores {
        let (k, v) = kv;
        println!("{} -> {}", k, v);
    }

    let mut scores = HashMap::new();
    // inserting k-v pairs
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // looking up by key
    let score = scores.get(&team_name);  

    // score is an Option<&{integer}>

    match score {
        Some(s) => println!("score: {}", s),
        None => println!("no score"),
    }

    let mut scores = HashMap::new();

    // overwriting, last one wins
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


    // conditional insert if key does not exist

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // update based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word counts: {:?}", map);

}