fn largest_non_generic_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_non_generic_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Any type T with a partial ordering trait 
// and the Copy trait so we can move the value out of the list
fn largest_generic<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// with generic methods
impl<T> Point<T> {
    // the generic type itself
    fn x(&self) -> &T {
        &self.x
    }
}

// only for Point<f32>, not all
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(self, other: MixedPoint<X2, Y2>) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let max = largest_non_generic_i32(&number_list);
    println!("The largest number is {}", max);

    let number_list = vec![134, 150, 125, 1100, 165];
    let max = largest_non_generic_i32(&number_list);
    println!("The largest number is {}", max);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let max = largest_non_generic_char(&char_list);
    println!("The largest char is {}", max);

    // We will come back to this
    // but it takes a bit of voodoo to make this generic

    // Generics in structs
    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // Generics with multiple types
    let _mixed = MixedPoint { x: 1, y: 2.0 };

    // Generics in enums
    // we have seen the standard option type
    // enum Option<T> { Some(T), None, }
    let _some = Some(42);
    let _none: Option<i32> = None;

    // generic methods
    let p = Point { x: 5, y: 10 };
    println!("Point has x-coördinate: {}", p.x());
    let p = Point { x: 1.0, y: 0.0 };
    println!("Dist from origo: {}", p.distance_from_origin());

    // mixing
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // there is no overhead for generics,
    // the compiler performs monomorphisation, binding the concrete types at compile-time

    traits();
}

// Let's define some traits
// See https://doc.rust-lang.org/stable/book/ch10-02-traits.html

pub trait Summary {
    fn summarize(&self) -> String;
}

// we can also define a trait with a default implementation
pub trait SummaryWithDefault {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// implements Summary trait for this
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// implements the trait for this, too
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// we may use the trait as parameter (items that implement Summary)
fn notify_trait(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// or we may state it using generics (types T with trait summary)
// this is called the "trait bound" syntax
// See https://doc.rust-lang.org/stable/book/ch10-02-traits.html
fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

use std::fmt;

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "@{}: {}, reply:{}, RT:{}", self.username, self.content, self.reply, self.retweet)
    }
}


// use plus to require multiple traits
pub fn notify_multiple_traits(item: &(impl Summary + fmt::Display)) {
    println!("Multiple traits: {}", item.summarize());
}

// use where for a cleaner syntax to require multiple traits
pub fn notify_multiple_traits_generic<T>(item: &T)
where
    T: Summary + fmt::Display,
{
    println!("Multiple traits with where: {} / {}", item.summarize(), item);
}

fn coerce_to_summary<T: Summary>(item:&T) -> &impl Summary 
{
    item
}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

/*
impl<T> std::fmt::Display for Pair<T> where T: std::fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.x, self.y)
}*/

// mixing is possible,
// implement for typs that are both Display and PartialOrd
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn traits() {
    let na = NewsArticle {
        headline: String::from("Clickbait"),
        location: String::from("Giedi Prime"),
        author: String::from("Harkonnen"),
        content: String::from("Some people met here"),
    };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // use the trait
    println!("1 new article: {}", na.summarize());
    println!("1 new tweet:   {}", tweet.summarize());

    // we can use the trait as parameter type in fns
    notify_trait(&na);
    notify_generic(&tweet);

    notify_multiple_traits(&tweet);
    notify_multiple_traits_generic(&tweet);

    let summarizable = coerce_to_summary(&tweet);
    println!("Summarizable: {}", summarizable.summarize());


    // trait fits in a vector with a little twis
    // vectors are of things that have a size, a trait does not
    // but we can box it, and the boxes have a size so we can put them in a list
    let stuff: Vec<Box<dyn Summary>> = vec![Box::new(na), Box::new(tweet)];
    for i in stuff {
        println!("Summary: {}", i.summarize());
    }

    let char_list = vec!['y', 'm', 'a', 'q'];
    let max = largest_generic(&char_list);
    println!("The largest char is {}", max);

    let p:Pair<i32> = Pair::new(1, 2);
    p.cmp_display(); // prints: The largest member is y = 2
}
