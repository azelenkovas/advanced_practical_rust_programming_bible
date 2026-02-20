use simple_calculator::{evaluate, parse_expression};
use std::io::{self, Write};

// Simple calculator example

// Design goals
//   Support binary operations
//   Accept both words and symbols: add 2 3, + 2 3, 2 + 3
//   Provide a REPL (read, eval print loop)
//   Gracefully handle errors (bad parse, division by zero)
//   Expose testable functions

fn main() -> Result<(), Box<dyn ::std::error::Error>> {
    println!("Simple calculator REPL");
    println!("Enter expressions like + 2 3 or add 2 3");
    println!("Type 'quit' or 'exit' to leave");

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        let bytes = stdin.read_line(&mut line)?;
        if bytes == 0 {
            println!();
            break;
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.eq_ignore_ascii_case("quit") || trimmed.eq_ignore_ascii_case("exit") {
            break;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        match parse_expression(&tokens) {
            Ok((op, a, b)) => match evaluate(op, a, b) {
                Ok(result) => println!(" = {}", result),
                Err(e) => eprintln!("Error evaluating expression: {}", e),
            },
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }

    Ok(())
}
