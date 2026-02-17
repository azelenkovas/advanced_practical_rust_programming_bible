fn main() {
    let num = 10; // i32 by default
    let num_f64 = 3.14; // f64 by default
    let num2 = 1_000_000; // underscores are ignored in numeric literals for readability
    let num3 = 42u8; // explicitly u8
    let num_f32 = 3.14f32; // explicitly f32

    let x_as = 10132 as f64; // casting an integer to a floating-point number
    let x_from = f64::from(10132); // using the From trait to convert an integer to a floating-point number
    let x_try_from = f64::try_from(10132); // using the TryFrom trait to attempt to convert an integer to a floating-point number, returning a Result

    println!(
        "num: {}, num_f64: {}, num2: {}, num3: {}, num_f32: {}, x_as: {}, x_from: {}, x_try_from: {:?}",
        num, num_f64, num2, num3, num_f32, x_as, x_from, x_try_from
    );

    // Integer overflow:
    // In debug builds, Rust will panic on integer overflow, while in release builds it will wrap around using two's complement arithmetic.
    // For example:
    let max_u8 = u8::MAX; // 255
                          // let overflow = max_u8 + 1; // In debug mode, this will panic
                          // println!("max_u8: {}, overflow: {}", max_u8, overflow);

    // Boolean type
    let is_rust_fun = true; // boolean type can hold true or false
                            // Conditions require a boolean, integers do not coerce into booleans like in some other languages like C
    if is_rust_fun {
        println!("Rust is fun!");
    } else {
        println!("Rust is not fun.");
    }

    // Character type
    // Char is a 4-byte Unicode scalar value, which means it can represent a wide range of characters from different languages and symbol sets, including emojis.
    let letter_a = 'A'; // char can hold a single Unicode scalar value
    let emoji = '😀'; // char can also hold emoji
    let chinese_char = '你'; // char can hold characters from different languages
    let math_symbol = '∑'; // char can hold mathematical symbols
    println!(
        "letter_a: {}, emoji: {}, chinese_char: {}, math_symbol: {}",
        letter_a, emoji, chinese_char, math_symbol
    );

    // Strings are UTF-8 encoded sequences of bytes
    // char is a single Unicode scalar value
    // String literals are string slices (&str) that are stored in the binary and have a static lifetime
}
