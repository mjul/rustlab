// use on the module level brings in the HashMap namespace
use std::collections::HashMap;

use std::collections::HashSet as HSet; // this changes the name from HashSet:: to HSet::

// Using external crates - add them to Cargo.toml 
//
//   [dependencies]
//   rand = "0.8.5"

use rand::Rng;

// we may also nest paths to simplify use statements
use std::{cmp::Ordering, io};
// the above uses std::comp::Ordering and std::io

// we may also glob to bring in all public items
use std::collections::*;

fn main() {
    println!("Hello, world!");

    // full path to HashMap
    let mut map_with_full_path = std::collections::HashMap::new();
    map_with_full_path.insert(42, 1);

    // shortened path by the use statement above
    // this is the idiomatic way
    let mut map_with_use_path = HashMap::new(); 
    map_with_use_path.insert(1, 42);

    let mut hset = HSet::new();
    hset.insert(42);

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secret: {}", secret_number);
}
