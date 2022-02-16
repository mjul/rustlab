// https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// let's see if we can change from String to str
struct UserWithStr {
    // we cannot use the type &str since it will give the error
    // "expected named lifetime parameter"
    // username: &str,
    // email: &str,
    sign_in_count: u64,
    active: bool,
}
// we will come back to this in Chapter 10

fn main() {
    let immutable_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("Field access: {}", immutable_user.username);

    // weirdly the example is with String not str
    // (it is due to ownership and lifetime rules, as we will see below)
    // however, we cannot mutate the String fields anyway
    //
    // this will fail with an error:
    // error[E0596]: cannot borrow `immutable_user.email` as mutable, as `immutable_user` is not declared as mutable
    //
    // immutable_user.email.clear();
    // Structs may be mutable
    let mut mutable_user = User {
        email: String::from("foo@example.com"),
        username: String::from("foo"),
        active: true,
        sign_in_count: 1,
    };
    mutable_user.email = String::from("another.email@example.com");
    // now we may borrow the email field and mutate it
    mutable_user.email.clear();
    mutable_user.email.push_str("different.email@example.com");
    println!(
        "User/email: {} {}",
        mutable_user.username, mutable_user.email
    );

    let template_user = build_user(String::from("foobar@example.com"), String::from("foobar"));
    println!("New user: {}", template_user.username);

    // create and User by copying another (the .. part)
    // and assigning new values to the specified fields
    let updated_user = User {
        email: String::from("another@example.com"),
        ..template_user
    };
    println!("Updated: {} {}", updated_user.username, updated_user.email);

    // not the move semantics for the fields
    // email was not copied to template_user, so it is still accessible
    println!("Template: email {}", template_user.email);
    // username was copied (moved) so it is not longer accessible
    // not possible:
    // println!("Template: username {}", template_user.username);

    // Tuple structs
    // Structs may be named tuples, not just records like we saw above
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-like structs without fields
    // maybe these can be used for phantom types?
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // Structs in practice
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };
    println!("Area of the rectangle: {}", area(&rect1));

    // ToString:
    // We cannot print the rectangle unless it has the std::fmt::Display trait
    // Error: the trait `std::fmt::Display` is not implemented for `Rectangle`
    //println!("Rect: {}", rect1);

    // Print using the Debug format specifier
    // when we add this with the #[derive(Debug)] macro on the struct
    // declaration it will make the struct printable
    let pr = PrintableRectangle {
        width: 10,
        height: 20,
    };
    println!("Rect: {:?}", pr);

    // we may also use the dbg! macro to print file/linenumber info and the value:
    dbg!(&pr);

    // below we have added an "impl" area function to the
    // Rectangle struct namespace
    // we can now call it like a member method

    // this is called a "method syntax" function call
    println!("rectangle.area(): {}", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // lets create a square rectangle with the "static" factory method
    // this has a different syntax
    let r = Rectangle::square(42);
    println!("Square area: {}", r.area());
}

// a builder function, convention: match parameter and field names
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}

// This macro adds the Debug trait to the struct, making it printable with the :? formatter
//     println!("Rect: {:?}", pr);
// And with the debug printer:
//     dbg!(&pr);
#[derive(Debug)]
struct PrintableRectangle {
    width: u32,
    height: u32,
}

// we may add functions to the namespace of the struct like this
// impl declares an implementation block for Rectangle
//
// the functions in that block are called "associated functions" of the Rectangle
// they are not required to take an instance: you just omit the first parameter self reference
impl Rectangle {
    // this is kind of pythonic with a self parameter
    // self is a syntactic sugar short-hand for &self:Self
    fn area(&self) -> u32 {
        // this will borrow self immutably (since we use the borrow operator &, &self)
        // Taking ownership (self) and borrowing mutably (&mut self) is also possible
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// impl blocks for a type will be combined by the compiler, so we may break 
// them into multiple blocks if we like:
impl Rectangle {
    // a factory method does not take an instance, so it has no self reference
    // BTW, this is a patological example since a square rectangle has
    // rectangle semantics but in many cases the better model would give it Square semantics.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}