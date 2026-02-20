fn main() {
    // Ownership rules:
    // Each value in Rust has a single owner
    // When the owner goes out of scope, the value is dropped (freed)
    // At any time, you can have ONE mutable reference or any number of immutable references, but NOT both!
    //
    // Those rules eliminate bugs:
    //   Dangling pointers
    //   Double frees
    //   Data races in concurrent programs

    // Ownership in action:

    let s1 = String::from("Rust");
    let s2 = s1; // ownership moves from s1 to s2

    println!("{}", s1); // error: s1 no longer owns the data
    println!("{}", s2); // valid

    // Here String is stored on the heap. When we assign s1 to s2,
    // ownership is moved, not copied. s1 becomes invalid; attempting
    // to use it causes a compile-time error.

    // This prevents dangling references - when the owner (s1) goes out
    // of scope, the data is freed, and no invalid references exist.
}
