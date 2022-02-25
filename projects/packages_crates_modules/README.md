# Managing Growing Projects with Packages, Crates, and Modules

See the Rust book: https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

# The Rust Module System

- Packages: A Cargo feature that lets you build, test, and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and use: Let you control the organization, scope, and privacy of paths
- Paths: A way of naming an item, such as a struct, function, or module

# Packages and Crates
See https://doc.rust-lang.org/stable/book/ch07-01-packages-and-crates.html

`src/main.rs` is the crate root by default, not mentioned in [Cargo.toml](Cargo.toml).

- a Package is one or more crates, [Cargo.toml](Cargo.toml) is the build spec for the crate.
- a Crate is a tree of modules for building a library or executable


# Modules 
See https://doc.rust-lang.org/stable/book/ch07-02-defining-modules-to-control-scope-and-privacy.html


# Split modules across files
Finally, we may split a module into files, see:
https://doc.rust-lang.org/stable/book/ch07-05-separating-modules-into-different-files.html