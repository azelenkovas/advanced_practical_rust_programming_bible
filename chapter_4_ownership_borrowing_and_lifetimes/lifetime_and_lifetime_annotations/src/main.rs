fn main() {
    // A lifetime tells the compiler how long a reference is valid.
    // Every reference has a lifetime, but usually, Rust infers it automatically.
    // When it can't, you must specify lifetimes explicitly.

    // Lifetimes prevent dangling references.

    // Example of an invalid borrow:
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // error; x does not live long enough
    // }
    // println!("{}", r);

    let s1 = String::from("hi");
    let s2 = String::from("hello");
    println!("{}", longest(&s1, &s2));
}

// Lifetime annotations in functions
// When a function returns a reference, you must sometimes
// specify how input lifetimes relate to output lifetimes
// 'a is the lifetime parameter
// Here it means: the returned reference lives as long as both x and y
// This tells the compiler the returned reference is valid only if both inputs are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime elision rules:
// 1. Each parameter gets its own lifetime
// 2. If there is exactly one lifetime, that becomes the output lifetime
// 3. If there are multiple lifetimes but one of them is &self or &mut self, that one is used for the output
