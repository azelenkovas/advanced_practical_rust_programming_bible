fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" world!");
}

fn main() {
    // Borrowing allows you to let other code temporarily
    // access your data without transferring ownership.
    // This is done through references:
    //   &T for immutable
    //   &mut T for mutable

    // Immutable borrowing
    // An immutable reference lets other code read the dat but not modify it.
    let s = String::from("hello");

    let len = calculate_length(&s);
    println!("The length of {} is {}", s, len);

    // The &s indicates a borrow, not an ownership transfer.
    // After calling calculate_length, s is still valid.

    // Mutable borrowing:
    // Mutable references alow modifying the borrowed value but follow a strict rule:
    // At any given time, there can be either one mutable reference or any number of
    // immutable references but not both

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    // If you try to have both a mutable and an immutable reference:
    let mut s = String::from("hi");
    let r1 = &s;
    let r2 = &mut s; // Error: cannot borrow s as mutable because it is also borrowed as immutable
    println!("{} {}", r1, r2);

    // This rule prevents data races - situations where multiple threads (or code paths)
    // simultaneously read and write the same memory
}
