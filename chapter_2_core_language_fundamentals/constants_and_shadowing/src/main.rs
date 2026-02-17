fn main() {
    // Constants
    const MAX_POINTS: u32 = 100_000; // Constants must have a type annotation and are immutable by default
    static mut COUNTER: u32 = 0; // Static variables can be mutable but require unsafe code to modify
    static HELLO_WORLD: &str = "Hello, world!"; // Static variables are global and can also hold string literals

    // Shadowning
    let spaces = "  "; // spaces is a string slice
    let spaces = spaces.len(); // spaces is now an integer, shadowing the previous variable

    // Shadowing is useful for
    // Transforming data while keeping the same variable name
    // Narrowing types (e.g. parse a string to a number and keep the same name)

    println!(
        "MAX_POINTS: {}, COUNTER: {}, HELLO_WORLD: {}, spaces: {}",
        MAX_POINTS,
        unsafe { COUNTER },
        HELLO_WORLD,
        spaces
    );
}
