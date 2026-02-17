fn main() {
    let a = 10; // i32 by default
    let b: i64 = 10; // explicitly i64
    let c = 3.14 // f64 by default
    
    let v8: i8 = 127; // i8 can hold values from -128 to 127
    let v16: i16 = 32767; // i16 can hold values from -32768 to 32767
    let v32: i32 = 214748364
    let v64: i64 = 9223372036854775807; // i64 can hold values from -9223372036854775808 to 9223372036854775807
    let v128: i128 = 170141183460469231731687303715884105727; // i128 can hold values from -170141183460469231731687303715884105728 to 170141183460469231731687303715884105727
    let vsize: isize = 9223372036854775807; // isize can hold values from -9223372036854775808 to 9223372036854775807 on a 64-bit system
    
    let vu8: u8 = 255; // u8 can hold values from 0 to 255
    let vu16: u16 = 65535; // u16 can hold values from
    let vu32: u32 = 4294967295; // u32 can hold values from 0 to 4294967295
    let vu64: u64 = 18446744073709551615; // u64 can hold values from 0 to 18446744073709551615
    let vu128: u128 = 340282366920938463463374607431768211455; // u128 can hold values from 0 to 340282366920938463463374607431768211455
    let vusize: usize = 18446744073709551615; // usize can hold values from 0 to 18446744073709551615 on a

    let f32: f32 = 3.14; // f32 can hold values with 6-9 decimal digits of precision
    let f64: f64 = 3.14; // f64 can hold values with 15-17 decimal digits of precision

    let bool: bool = true; // boolean type can hold true or false

    let char: char = 'A'; // char can hold a single Unicode scalar value

    let tuple: (i32, f64, char) = (10, 3.14, 'A'); // a tuple can hold multiple values of different types

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // fixed size: an array can hold multiple values of the same type

    let slice: &[i32] = &array; // a slice is a reference to a contiguous sequence of elements in an array
    
    println!("a: {}, b: {}, c: {}", a, b, c);

    // When to annotate types explicitly:
    // 1. When the type cannot be inferred from the context
    // 2. When you want to specify a different type than the default
    // 3. When you want to improve code readability by making the type clear
    // 4. Public function signatures (API clarity)
    // 5. Numeric literals that must by a specific type (e.g. indexing with usize)


}
