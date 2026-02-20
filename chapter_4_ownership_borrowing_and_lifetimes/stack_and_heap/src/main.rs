fn main() {
    // Stack
    // Stores fixed-size, known-at-compile-time values.
    // Very fast allocation and deallocation (LIFO)
    // Example: integers, floats, references, small tuples
    let s = 42; // stored on the stack

    // Heap
    // Stores data with variable or unknown size at compile time.
    // Slower allocation, managed through ownership
    // Example: String, Vec, Box
    let s = String::from("Hello"); // heap allocation

    // When s goes out of scope, Rust automatically calls Drop implementation,
    // freeing the memory

    // Moving between stack and heap
    let b = Box::new(10); // allocates integer 10 on the heap
    println!("{}", b); // access through pointer

    // Ownership ensures the heap memory is properly cleaned up
    // when b goes out of scope - no leaks or dangling pointers
}
