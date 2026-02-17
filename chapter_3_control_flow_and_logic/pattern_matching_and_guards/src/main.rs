fn main() {
    
    // Composite match
    let pair = (0,5);

    match pair {
        (0, y) => println!("x is zero, y = {}", y),
        (x, 0) => println!("x is {}, y = 0", x),
        (x, y) => println!("x is {}, y = {}", x, y), 
    }

    // Struct matching
    struct Point { x: i32, y: i32 }

    let p = Point { x: 0, y: 7 };

    match p {
        Point {x, y: 0} => println!("on the x axis at {}", x),
        Point {x: 0, y} => println!("on the y axis at {}", y),
        Point {x, y} => println!("at ({}, {})", x, y),
    }

    // If guards
    let n = 4;

    match n {
        x if x < 0 => println!("negative!"),
        x if x % 2 == 0 => println!("even!"),
        _ => println!("odd!")
    }

    // @ bindings
    enum Message {Id(u32)}

    let msg = Message::Id(42);

    match msg {
        Message::Id(id @ 1..100) => println!("Small id: {}", id),
        Message::Id(id) => println!("id: {}", id)
    }

    // ref and ref mut
    // getting a reference to value inside match
    let s = String::from("hello");

    match s {
        ref r => println!("borrowed: {}", r) // r: &String
    }

    // if let and while let
    let opt: Option<i32> = Some(3);

    if let Some(x) = opt  {
        println!("got {}", x);
    } else {
        println!("None");
    }

    let mut stack = vec![1,2,3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

}
