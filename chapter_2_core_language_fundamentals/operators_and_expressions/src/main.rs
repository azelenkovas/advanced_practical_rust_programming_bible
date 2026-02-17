fn main() {
    // Operators
    // +, -, *, /, % for arithmetic operations
    let a: i32 = 10;
    let b: i32 = 3;
    println!("a + b = {}", a + b); // 13
    println!("a - b = {}", a - b); // 7
    println!("a * b = {}", a * b); // 30
    println!("a / b = {}", a / b); // 3 (integer division)
    println!("a % b = {}", a % b); // 1 (remainder)

    // Comparison operators: ==, !=, <, >, <=, >=
    println!("a == b: {}", a == b); // false
    println!("a != b: {}", a != b); // true
    println!("a < b: {}", a < b); // false
    println!("a > b: {}", a > b); // true
    println!("a <= b: {}", a <= b); // false
    println!("a >= b: {}", a >= b); // true

    // Logical operators: &&, ||, !
    let x: bool = true;
    let y: bool = false;
    println!("x && y: {}", x && y); // false
    println!("x || y: {}", x || y); // true
    println!("!x: {}", !x); // false
    println!("!y: {}", !y); // true

    // Assignment operators: =, +=, -=, *=, /=, %=
    let mut c: i32 = 5;
    c += 2; // c = c + 2
    println!("c after += 2: {}", c); // 7
    c *= 3; // c = c * 3
    println!("c after *= 3: {}", c); // 21
    c /= 7; // c = c / 7
    println!("c after /= 7: {}", c); // 3 (integer division)
    c %= 2; // c = c % 2
    println!("c after %= 2: {}", c); // 1

    // Operator precedence: parentheses > unary > multiplicative > additive > comparison > logical
    let result = (a + b) * 2; // parentheses first, then multiplication
    println!("(a + b) * 2 = {}", result); // 26

    // Ranges
    // .. for exclusive range, ..= for inclusive range
    for i in 0..5 {
        // 0, 1, 2,
        println!("Exclusive range: {}", i);
    }
    for i in 0..=5 {
        // 0, 1, 2, 3, 4, 5
        println!("Inclusive range: {}", i);
    }

    // Expressions and statements
    let x = 5; // statement
    let y = {
        let z = 2;
        z + 1
    }; // expression block, evaluates to 3
    println!("y: {}", y); // 3
    let z = if x > 3 { "greater" } else { "less or equal" }; // if is an expression, evaluates to a string slice
    println!("z: {}", z); // "greater"
}
