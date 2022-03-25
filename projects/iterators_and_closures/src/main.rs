use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // this is a closure, not the Ruby-like syntax (or inspired by Smalltalk [:num | num+1] )
    // unlike functions, the compile will infer the types so they may be omitted
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // types may be specified like this:
    let _typed_closure = |num: u32| -> u32 { num + 1 };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn _equivalent_fns_and_closures() {
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| x + 1;
    let add_one_v4 = |x| x + 1;

    // these calls give the information to infer the types of v3 and v4
    add_one_v3(42);
    add_one_v4(2);
}

fn _conflict_in_type_inference() {
    // The inference is just one
    let example_closure = |x| x;
    // now we infers x to be a String -> String
    let _s = example_closure(String::from("hello"));
    // the types are now fixed for the closure
    // calling with non-String will not compile:
    // let n = example_closure(5);
}

// we can use a struct to cache the results of a closure, making it a lazy eval-once calculation
// All closures implement at least one of the traits: Fn, FnMut, or FnOnce.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // we just instantiate it with the calculation and value None for not calculated
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // the laziness is implemented in fn value:
    // - returns the value if the closure has already been evaluated, or
    // - mutates the struct to add the evaluation result the first time it is needed
    //
    // Note: this definition is a bit funny since it only uses arg the first time
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// this fails, arg is fixed on first call
#[ignore]
#[test]
fn cacher_call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

fn generate_workout_with_cacher(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Let's try again to make it work for all args
struct Memoizer<T, T1, T2>
where
    T: Fn(&T1) -> T2,
{
    calculation: T,
    memoized: HashMap<T1, T2>,
}

impl<T, T1, T2> Memoizer<T, T1, T2>
where
    T: Fn(&T1) -> T2,
    T1: Eq,
    T1: Hash,
    T2: Clone,
{
    // we just instantiate it with the calculation and value None for not calculated
    fn new(calculation: T) -> Memoizer<T, T1, T2> {
        Memoizer {
            calculation,
            memoized: HashMap::new(),
        }
    }

    // memoize on lookup
    fn value(&mut self, arg: T1) -> T2 {
        match self.memoized.get(&arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(&arg);
                self.memoized.insert(arg, v.clone());
                v
            }
        }
    }
}

// this fails, arg is fixed on first call
#[test]
fn memoizer_call_with_different_values() {
    let mut c = Memoizer::new(|a| a * 10);

    let _v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 20);
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout_with_cacher(simulated_user_specified_value, simulated_random_number);

    play_with_iterators();
}

#[test]
fn closure_captures_local_env() {
    let x = 4;
    // this closure captures x 
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}


fn play_with_iterators() {

    // vanilla stuff
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // iterators come from the Iterator trait,
    // you just need to implement the next function:
    // fn next(&mut self) -> Option<Self::Item>;

    // see the lib.rs file for more examples

    
}

