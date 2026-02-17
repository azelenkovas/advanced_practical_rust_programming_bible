/// Supported temperature units
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

/// Parses a character into a `Unit` enum variant.
///
/// Supports 'c'/'C' for Celsius, 'f'/'F' for Fahrenheit, and 'k'/'K' for Kelvin.
///
/// # Arguments
///
/// * `unit_char` - A character representing the temperature unit.
///
/// # Returns
///
/// * `Some(Unit)` - If the character is one of 'C', 'F', or 'K' (case-insensitive).
/// * `None` - If the character is not a recognized unit.
pub fn parse_unit(unit_char: char) -> Option<Unit> {
    match unit_char.to_ascii_lowercase() {
        'c' => Some(Unit::Celsius),
        'f' => Some(Unit::Fahrenheit),
        'k' => Some(Unit::Kelvin),
        _ => None,
    }
}

/// Convert temperature between units
/// This function takes a temperature value, the source unit, and the target unit, and returns the converted
/// temperature value. It supports conversions between Celsius, Fahrenheit, and Kelvin. If the source and target
/// units are the same, it returns the original value. If the conversion is not supported, it returns None.
/// # Arguments
/// - `value`: The temperature value to convert
/// - `from_unit`: The unit of the input temperature value
/// - `to_unit`: The unit to convert the temperature value to
/// # Returns
/// - `Some(f64)`: The converted temperature value if the conversion is successful
/// - `None`: If the conversion is not supported or if the input is invalid
/// # Examples
/// ```
/// use simple_temperature_converter::convert_temperature;
/// use simple_temperature_converter::Unit;
/// let temp_c = convert_temperature(25.0, Unit::Celsius, Unit::Fahrenheit);
/// assert_eq!(temp_c, Some(77.0));
/// let temp_k = convert_temperature(373.15, Unit::Kelvin, Unit::Celsius);
/// assert_eq!(temp_k, Some(100.0));
/// ```
/// # Panics
/// This function does not panic. It returns None for unsupported conversions or invalid inputs.
/// # Note
/// This function assumes that the input value is a valid temperature and does not perform any range checks (e.g., negative Kelvin values). It is the caller's responsibility to ensure that the input value is valid for the given unit.
/// # See Also
/// - `parse_unit`: Function to parse unit characters into `Unit` enum variants
/// - `parse_input`: Function to parse temperature input strings into value and unit components
pub fn convert_temperature(value: f64, from_unit: Unit, to_unit: Unit) -> Option<f64> {
    if from_unit == to_unit {
        return Some(value); // No conversion needed
    }
    match (from_unit, to_unit) {
        (Unit::Celsius, Unit::Fahrenheit) => Some(value * 9.0 / 5.0 + 32.0),
        (Unit::Celsius, Unit::Kelvin) => Some(value + 273.15),
        (Unit::Fahrenheit, Unit::Celsius) => Some((value - 32.0) * 5.0 / 9.0),
        (Unit::Fahrenheit, Unit::Kelvin) => Some((value - 32.0) * 5.0 / 9.0 + 273.15),
        (Unit::Kelvin, Unit::Celsius) => Some(value - 273.15),
        (Unit::Kelvin, Unit::Fahrenheit) => Some((value - 273.15) * 9.0 / 5.0 + 32.0),
        _ => None, // Invalid unit combination
    }
}

/// Parses a temperature input string into a value and a unit.
///
/// The input string should consist of a numeric value followed by a single character
/// representing the unit (C, F, or K).
///
/// # Arguments
///
/// * `input` - A string slice containing the temperature value and unit (e.g., "25C", "77F").
///
/// # Returns
///
/// * `Some((f64, Unit))` - A tuple containing the parsed numeric value and the `Unit` enum variant if successful.
/// * `None` - If the input string is too short, the value is not a valid number, or the unit character is unrecognized.
///
/// # Examples
///
/// ```
/// use simple_temperature_converter::{parse_input, Unit};
///
/// assert_eq!(parse_input("25C"), Some((25.0, Unit::Celsius)));
/// assert_eq!(parse_input("77f"), Some((77.0, Unit::Fahrenheit)));
/// assert_eq!(parse_input("300K"), Some((300.0, Unit::Kelvin)));
/// assert_eq!(parse_input("invalid"), None);
/// ```
pub fn parse_input(input: &str) -> Option<(f64, Unit)> {
    if input.len() < 2 {
        return None; // Input too short to be valid
    }
    let (value_str, unit_char) = input.split_at(input.len() - 1);
    let value: f64 = value_str.trim().parse().ok()?;
    let unit = parse_unit(unit_char.chars().next()?)?;
    Some((value, unit))
}

/// Prints the usage instructions for the temperature converter command-line tool.
///
/// This function outputs the expected command-line argument format, provides an example,
/// and lists the supported temperature units to standard error (stderr).
pub fn print_usage() {
    eprintln!("Usage: temperature_converter <value><unit> <target_unit>");
    eprintln!("Example: temperature_converter 25C F");
    eprintln!("Supported units: C (Celsius), F (Fahrenheit), K (Kelvin)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unit() {
        assert_eq!(parse_unit('C'), Some(Unit::Celsius));
        assert_eq!(parse_unit('F'), Some(Unit::Fahrenheit));
        assert_eq!(parse_unit('K'), Some(Unit::Kelvin));
        assert_eq!(parse_unit('X'), None);
    }

    #[test]
    fn test_convert_temperature() {
        assert_eq!(
            convert_temperature(25.0, Unit::Celsius, Unit::Fahrenheit),
            Some(77.0)
        );
        assert_eq!(
            convert_temperature(25.0, Unit::Celsius, Unit::Kelvin),
            Some(298.15)
        );
        assert_eq!(
            convert_temperature(77.0, Unit::Fahrenheit, Unit::Celsius),
            Some(25.0)
        );
        assert_eq!(
            convert_temperature(77.0, Unit::Fahrenheit, Unit::Kelvin),
            Some(298.15)
        );
        assert_eq!(
            convert_temperature(273.15, Unit::Kelvin, Unit::Celsius),
            Some(0.0)
        );
        assert_eq!(
            convert_temperature(373.15, Unit::Kelvin, Unit::Fahrenheit),
            Some(212.0)
        );
        assert_eq!(
            convert_temperature(25.0, Unit::Celsius, Unit::Celsius),
            Some(25.0)
        ); // No conversion
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(parse_input("25C"), Some((25.0, Unit::Celsius)));
        assert_eq!(parse_input("77F"), Some((77.0, Unit::Fahrenheit)));
        assert_eq!(parse_input("300K"), Some((300.0, Unit::Kelvin)));
        assert_eq!(parse_input("25X"), None);
        assert_eq!(parse_input("C"), None);
    }
}
