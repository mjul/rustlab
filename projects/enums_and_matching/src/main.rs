// An enum with two kinds of IP address
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrWithKind {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrWithTuple {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This is a data-carrying enum, a sort of algebraic data type "sum type"
enum IpAddrWithAddress {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called");
    }
}

fn main() {
    // we get instances like this (:: "static")
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // unsurprisingly we may call functions for type &IpAddKind -> ()
    // with any of the enum branches
    route(&four);
    route(&six);

    // we may use enums in other structs
    let _home = IpAddrWithKind {
        kind: four,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddrWithKind {
        kind: six,
        address: String::from("::1"),
    };

    // now, we may also add the address datum directly
    // if we define the enum differently

    let _home = IpAddrWithAddress::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrWithAddress::V6(String::from("::1"));

    // enums may contain complex data like tuples, not just single values
    let _home = IpAddrWithTuple::V4(127, 0, 0, 1);
    let _loopback = IpAddrWithTuple::V6(String::from("::1"));

    // we may add methods (associated functions) to enums, like for structs
    let m = Message::Write(String::from("hello"));
    m.call();

    // Standard library enums
    // Option
    //
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    let _some_number = Some(5); // type Option<i32>
    let _some_string = Some("a string"); // type Option<&str>
    let _absent_number: Option<i32> = None; // explicit type since it cannot be inferred

    let _x = Some(1);
    let _y = Some(2);
    // we cannot do this, since we must be able to handle both Some and None:
    //    let sum = _x + _y;

    // so we need pattern matching
    // move to chapter 6.2: https://doc.rust-lang.org/stable/book/ch06-02-match.html

    let cents = value_in_cents(Coin::Nickel);
    println!("A nickel is {} cents", cents);

    // We may "match" on enums, see the inc function below
    let five = Some(5);
    let six = inc(five);
    println!("Five {:?}", five); // Option has the debug trait
    println!("Six {:?}", six);

    let none = inc(None);
    println!("None++ {:?}", none);

    // we may use catch all matches, see this function
    let s = catch_all_match(Coin::Quarter);
    println!("Coin catch-all: {}", s);

    // _ is an anonymous catch-all
    let s = placeholder_match(Coin::Dime);
    println!("Coin placeholder: {}", s);

    // next up is chapter 6.3: https://doc.rust-lang.org/stable/book/ch06-03-if-let.html
    
    let config_max = Some(3u8);
    // just match
    match config_max {
        Some(max) => println!("Max config: {}", max),
        _ => (),
    }

    // syntax sugar
    
    // match destructuring with if let if there is only a single 
    // pattern to match and everything else is ignored
    // note that this is not exhaustive, so it will not warn if
    // more values are added to the enum being matched against
    if let Some(max) = config_max {
        println!("Some max config: {}", max);
    }

    // if let also supports else
    let coin = Coin::Penny;

    let mut count = 0;
    if let Coin::Quarter = coin {
        println!("Quarter");
    } else {
        count += 1;
    }
    println!("Count: {}", count);

}

fn route(ip_kind: &IpAddrKind) {}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            // if an expression is not sufficient, blocks are also possible
            25
        }
    }
}

fn inc(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_match(coin: Coin) -> String {
    match coin {
        Coin::Penny => String::from("penny"),
        other => String::from("other"),
    }
}

fn placeholder_match(coin: Coin) -> String {
    match coin {
        Coin::Penny => String::from("penny"),
        Coin::Nickel => String::from("nickel"),
        _ => String::from("anything else"),
    }
}
