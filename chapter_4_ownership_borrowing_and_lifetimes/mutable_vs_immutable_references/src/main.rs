fn main() {
    // Immutable references
    // Allow multiple concurrent readers
    // Gurarantee that the referenced data won't change while they exist
    let s = String::from("read-only");
    let r1 = &s;
    let r2 = &s;
    println!("{} {}", r1, r2); // ok, both are immutable references

    // Mutable references
    // Allow exclusive access: only one mutable reference at a time
    // Useful for safe in-place mutation
    let mut num = 5;
    let r = &mut num;
    *r += 1; // modify through mutable reference
    println!("{}", num);

    // Mutable and immutable references cannot coexist:
    let mut num = 10;
    let r1 = &num;
    let r2 = &mut num;
    println!("{} {}", r1, r2); // Compile error
}
