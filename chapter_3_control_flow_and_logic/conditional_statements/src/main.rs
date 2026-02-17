fn main() {
    // Conditional statements

    // If/else
    let x = 10;
    if x % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }

    // if as an expression
    let parity = if x % 2 == 0 {
        "even"
    } else if x % 2 == 1 {
        "odd"
    } else {
        "I don't know"
    };
    println!("parity: {}", parity);

    // Match
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let d = Direction::North;

    match d {
        Direction::North => println!("North!"),
        Direction::East => println!("East!"),
        Direction::South => println!("South!"),
        Direction::West => println!("West!"),
    }
}
