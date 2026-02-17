// Rust separates between recoverable and unrecoverable errors
// Use panic() for unrecoverable errors
// Use Result<T, E> for recoverable errors
// Use Option<T> for optional values (presence/absence)

// Option<T>
// Represents a value that might be present
// Useful helpers:
//   unwrap, unwrap_or, unwrap_or_else
//   map, and_then(chain), filter, ok_or
//   if let Some(x) = opt { ... } for handling the present case
// Prefer explicit error handling to unwrap in production; unwrap
// is good for quick prototypes or when you guarantee correctness
fn find_even(nums: &[i32]) -> Option<i32> {
    for &n in nums {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

// Resut<T, E>
// Represents a success (Ok) or failure (Err). E is a type that
// implements std::error::Error (for user-friendly formatting)
// but can be any type
//
// Common patterns:
//   Return Result from functions that might fail
//   Use ? to propagate errors
//   Use thiserror or anyhow crates in real projects for ergonomic
//   custom errors, but for learning, implement std::fmt::Display
//   and std::error::Error
//
// Result helpers:
//   map, map_err
//   and_then (flatMap)
//   unwrap_or_else
//   unwrap_or_default
fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    let n: i32 = s.parse::<i32>()?;
    Ok(n * 2)
}

// Use Option when absence is expected and not an error (e.g. Vec::pop())
// Use Result when an operation can fail due to external factors or invalid input
// Keep error types meaningful: Err(String) is quick, but custom error enums provide
// structure and conversation abilities

fn main() {
    println!("{:?}", find_even(&[1, 2, 5, 7]));
    println!("{:?}", find_even(&[1, 3, 5, 7]));
    println!("{:?}", parse_and_double("128"));
    println!("{:?}", parse_and_double("128A"));
}
