use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: temp-conv <value> <unit (C/F)>");
        eprintln!("Example: temp-conv 100 C");
        process::exit(1);
    }

    let value: f64 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Error: Invalid number '{}'", args[1]);
            process::exit(1);
        }
    };

    let unit = args[2].to_uppercase();

    match unit.as_str() {
        "C" => {
            let fahrenheit = (value * 9.0 / 5.0) + 32.0;
            println!("{:.2}°C is {:.2}°F", value, fahrenheit);
        }
        "F" => {
            let celsius = (value - 32.0) * 5.0 / 9.0;
            println!("{:.2}°F is {:.2}°C", value, celsius);
        }
        _ => {
            eprintln!(
                "Error: Invalid unit '{}'. Use 'C' for Celsius or 'F' for Fahrenheit.",
                unit
            );
            process::exit(1);
        }
    }
}
