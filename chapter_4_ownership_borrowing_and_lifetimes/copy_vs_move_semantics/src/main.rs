fn main() {
    // Rust differentiates between copy types and move types

    // Copy types
    // Small, fixed-size values (like integers, floats and booleans)
    // implement the Copy trait. Assigning them creates a bitwise copy.
    let x = 10;
    let mut y = x; // Copies the value
    y += 1;
    println!("x = {}, y = {}", x, y);

    // Move types
    // Heap-allocated types like String, Vec, or Box are moved, not copied
    let s1 = String::from("hello");
    let s2 = s1; // move
                 // println!("{}", s1); // error
    println!("{}", s2); // ok

    // If you want to duplicate heap data, use .clone()
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("{} and {}", s1, s2);

    // Rust's move semantics make ownership explicit and
    // gurantee that each heap resource has a single, clear owner.
}
