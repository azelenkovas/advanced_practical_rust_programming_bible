fn main() {
    let x = 5; // immutable variable
    println!("The value of x is: {}", x);
    let mut y = 5; // mutable variable
    println!("The value of y is: {}", y);
    // x = 6; // this will cause a compile-time error because x is immutable
    y = 6; // this is allowed because y is mutable
    println!("The value of y is: {}", y);

    // Mutability
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    let s_ref = &s; // immutable reference
                    // s_ref.push_str("!"); // this will cause a compile-time error because s_ref is immutable
    println!("{}", s_ref);

    let mut s_mut_ref = &mut s; // mutable reference
    s_mut_ref.push_str("!!!"); // this is allowed because s_mut_ref is mutable
    println!("{}", s_mut_ref);
}
