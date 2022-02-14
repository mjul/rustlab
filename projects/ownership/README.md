# Ownership

Now we are getting somewhere.

## Part One
See https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html

## Part Two
https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html

## Part Three
https://doc.rust-lang.org/stable/book/ch04-03-slices.html



## Tooling Using `cargo watch`

To improve the developer experience we can run `cargo` automatically when something changes.
See https://crates.io/crates/cargo-watch.


Install it:
    
    cargo install cargo-watch

Run it (this runs `cargo check` in a loop):

    cargo watch

Run cargo `run` on every change:

    cargo watch -x run



