use std::fs::File;
use std::io::{self, Read};

// Return
// The last expression is returned implicitly, no need for the return statement
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;
    }
    Some(a / b)
}

// ? operator
// Canonical way of handling errors from functions that return Result or Option
// Unwraps the Ok/Some case and returns early on Err/None
// It converts between error types when From is implemented
// Use map_err to transform error types when necessary
fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("{:?}", divide(64.0, 8.0));
    println!("{:?}", read_file("some_file.txt"));

    // break can return a value
    let mut i = 0;
    let found = loop {
        i += 1;
        if i == 3 {
            break i * 2;
        }
    };
    println!("{}", found);

    // Continue
    println!("Even numbers:");
    for i in 0..10 {
        if i % 2 == 1 {
            continue;
        }
        println!("{}", i);
    }
}
