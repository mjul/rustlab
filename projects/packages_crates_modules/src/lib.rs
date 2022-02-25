// a module definition, see https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
mod front_of_house {
    
    // modules form a tree (nesting)

    // modules are private by default, like everything else
    // use `pub` to expose the module itself (this does not expose its functions)
    pub mod hosting {
        // expose this function
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // note this is private, so you can only initialize it from inside the module
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // all variants are public, so they can be instantiated from outside
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}


// pub modifier to make it public
pub fn eat_at_restaurant_abs_rel_paths() {
    // Absolute path from module root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path from here
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_at_restaurant_appetizer_with_public_instantiation() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}


// use brings a package into scope (not it has its name as a qualifier)
use crate::front_of_house::hosting;

// alternatively, we may also use relative names for use
// use self::front_of_house::hosting;

// in both cases the idiomatic way is to 'use' the module 
// (even if it is also possible to extend the path to specific functions)

pub fn eat_at_restaurant_with_use_notation() {
    // now, just qualify with the package name (no path)
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// pub use is for re-exporting (use brings in symbols as private)
pub use crate::front_of_house::hosting as hoostiiiing;

