use std::io;

#[derive(Debug)]
enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

fn main() {
    intro();

    loop {
        let temp_unit_to = get_temperature_unit_to();
        let temp_unit_from = get_temperature_unit_from(&temp_unit_to);

        println!(
            "Great, we're converting from {:?} to {:?}",
            &temp_unit_from, &temp_unit_to
        );

        let amount = get_amount();

        let converted_amount = get_converted_amount(&amount, &temp_unit_to);

        let converted_amount_string = format!("{:.4}", converted_amount);

        println!(
            "{} {:?} is {} in {:?}",
            &amount, &temp_unit_from, converted_amount_string, &temp_unit_to
        );
    }
}

fn get_converted_amount(amount: &i32, temp_unit_to: &TemperatureUnit) -> f32 {
    match temp_unit_to {
        TemperatureUnit::Celsius => fahrenheit_to_celsius(*amount as f32),
        TemperatureUnit::Fahrenheit => celsius_to_fahrenheit(*amount as f32),
    }
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn get_amount() -> i32 {
    loop {
        println!("Please enter the temperature amount:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().parse::<i32>();

        match input {
            Ok(number) => {
                println!("Nice, you entered {}", number);
                break number;
            }
            Err(_) => {
                println!("The input entered was not a valid amount");
                continue;
            }
        }
    }
}

fn get_temperature_unit_from(temp_unit_to: &TemperatureUnit) -> TemperatureUnit {
    match temp_unit_to {
        TemperatureUnit::Fahrenheit => TemperatureUnit::Celsius,
        TemperatureUnit::Celsius => TemperatureUnit::Fahrenheit,
    }
}

fn get_temperature_unit_to() -> TemperatureUnit {
    loop {
        println!(
            "What temperature unit would you like to convert to? Celsius (C) or Fahrenheit (F)?"
        );

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "F" => break TemperatureUnit::Fahrenheit,
            "C" => break TemperatureUnit::Celsius,
            _ => {
                println!("Invalid input");
                continue;
            }
        }
    }
}

fn intro() {
    println!("=========================");
    println!("Welcome to temp_converter");
    println!("=========================");
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_celsius() {
        assert_eq!(fahrenheit_to_celsius(50.0), 10.0);
    }

    #[test]
    fn test_convert_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(30.0), 86.0);
    }
}
