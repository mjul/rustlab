// constants are always immutable and cannot be marked mutable with mut
// constants must always have a specified type (no type inference).
// constants must be built from constant expressions (at compile time)
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5; // mut makes the variable mutable so the assignment below will compile
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    {
        // shadowing the x variable in the inner scope
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    x = x + 1;
    println!("The value of x is: {}", x);
    // use the constant above to make the "unused" warning go away:
    x = THREE_HOURS_IN_SECONDS / 3;
    // print it to avoid the warning that it is never read
    println!("The value of x is: {}", x); 

    // also shadowing works in the same scope (last one wins)
    let spaces = "    ";
    let spaces = spaces.len();

    println!("Spaces: {}", spaces);
}
