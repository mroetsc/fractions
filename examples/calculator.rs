use fractions::Fraction;
use std::io::{self, Write};

fn main() {
    println!("Fraction Calculator");
    println!("==================");

    let fraction1 = match get_fraction("Enter first fraction (e.g., 3/4 or 5): ") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let fraction2 = match get_fraction("Enter second fraction (e.g., 1/2 or 3): ") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    print!("Enter operation (+, -, *, /): ");
    io::stdout().flush().unwrap();

    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read operation");

    let result = match operation.trim() {
        "+" => Ok(fraction1 + fraction2),
        "-" => Ok(fraction1 - fraction2),
        "*" => Ok(fraction1 * fraction2),
        "/" => fraction1 / fraction2,
        _ => {
            eprintln!("Unknown operation! Use +, -, *, or /");
            return;
        }
    };

    match result {
        Ok(fraction) => {
            println!("\nResult: {} = {:.4}", fraction, fraction.to_f64());
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn get_fraction(prompt: &str) -> Result<Fraction, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input.contains('/') {
        let parts: Vec<&str> = input.split('/').collect();
        if parts.len() != 2 {
            return Err("Invalid format".into());
        }

        let num: i64 = parts[0].parse()?;
        let den: i64 = parts[1].parse()?;
        Ok(Fraction::new(num, den)?)
    } else {
        let num: i64 = input.parse()?;
        Ok(Fraction::from(num))
    }
}
