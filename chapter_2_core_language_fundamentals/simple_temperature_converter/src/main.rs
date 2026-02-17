use std::env;
use std::process;

use simple_temperature_converter::{
    convert_temperature, parse_input, parse_unit, print_usage, Unit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        print_usage();
        process::exit(1);
    }

    let input = &args[1];
    let target_unit_char = args[2].chars().next().unwrap_or_default();

    let (value, from_unit) = match parse_input(input) {
        Some(result) => result,
        None => {
            eprintln!("Error: Invalid input format. Expected format: <value><unit> (e.g., 25C, 77F, 300K)");
            print_usage();
            process::exit(1);
        }
    };

    let to_unit = match parse_unit(target_unit_char) {
        Some(unit) => unit,
        None => {
            eprintln!(
                "Error: Invalid target unit '{}'. Supported units are C, F, K.",
                target_unit_char
            );
            print_usage();
            process::exit(1);
        }
    };

    match convert_temperature(value, from_unit, to_unit) {
        Some(converted_value) => {
            println!(
                "{}{} = {}{}",
                value,
                match from_unit {
                    Unit::Celsius => "°C",
                    Unit::Fahrenheit => "°F",
                    Unit::Kelvin => "K",
                },
                converted_value,
                match to_unit {
                    Unit::Celsius => "°C",
                    Unit::Fahrenheit => "°F",
                    Unit::Kelvin => "K",
                }
            );
        }
        None => {
            eprintln!(
                "Error: Conversion from {:?} to {:?} is not supported.",
                from_unit, to_unit
            );
            process::exit(1);
        }
    }
}
