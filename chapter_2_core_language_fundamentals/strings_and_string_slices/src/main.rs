fn main() {
    // String vs &str
    // String is a heap-allocated, growable string type, while &str is a string slice that references a string data.
    let s_literal: &str = "Hello, world!"; // string literal, type &str, stored in binary
    let mut s: String = String::from("Hello, world!"); // String type, owned, heap-allocated, mutable
    s.push_str(" Welcome to Rust!"); // mutate

    // String -> &str: &s or s.as_str()
    let s_slice: &str = &s; // implicit coercion from String to &str
    let s_slice2: &str = s.as_str(); // explicit method to get &str from String

    println!("String literal: {}", s_literal);
    println!("String: {}", s);

    // &str -> String: String::from(s_slice) or s_slice.to_string()
    let s_from_slice: String = String::from(s_slice); // create a new String
    let s_from_slice2: String = s_slice.to_string(); // another way to create a new String
    println!("String from &str: {}", s_from_slice);

    // Indexing and slicing
    // You cannot index String by integer becuase UTF-8 characters can be multiple bytes, but you can slice it using byte indices if you are sure they are valid UTF-8 boundaries.
    // Use chars() or bytes() iterators, or slice by byte index only if you know the boundaries.
    let s = String::from("Hello, world!");
    for c in s.chars() {
        println!("Character: {}", c);
    }

    // Common String methods
    // Concatenation: + operator or format! macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s4 = format!("{}{}", s1, s2); // format! does not take ownership of s1 and s2
    println!("Concatenated String with format!: {}", s4);
    let s3 = s1 + &s2; // s1 is moved here, s2 is borrowed, s3 is the result
    println!("Concatenated String: {}", s3);

    // Searching: contains, find, starts_with, ends_with
    let s = String::from("Hello, world!");
    println!("Contains 'world': {}", s.contains("world"));
    println!("Find 'world': {:?}", s.find("world"));
    println!("Starts with 'Hello': {}", s.starts_with("Hello"));
    println!("Ends with '!': {}", s.ends_with("!"));

    // Splitting: split, split_whitespace
    let s = String::from("Hello, world! Welcome to Rust.");
    for word in s.split_whitespace() {
        println!("Word: {}", word);
    }

    // Trimming: trim, trim_start, trim_end
    let s = String::from("   Hello, world!   ");
    println!("Trimmed: '{}'", s.trim());
    println!("Trimmed start: '{}'", s.trim_start());
    println!("Trimmed end: '{}'", s.trim_end());

    // &str is a fat pointer: pointer + length. Slices are cheap to coy and pass around, but they do not own the data they reference. String owns its data and can be modified, while &str is immutable and borrowed.
}
