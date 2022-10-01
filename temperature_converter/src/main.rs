use std::io;

// Ask the temperature value
// Validate temperature value
// Ask temperature option to conervte
// Validate temperature option
// Convert temperature Celsius to Fahrenheit and print
// Convert temperature Fahrenheit to Celsius and print

fn celsius_to_fahrenheit(c_value: i32) {
    let f_value = c_value * 9 / 5 + 32;
    println!("{} °C = {} °F", c_value, f_value);
}

fn fahrenheit_to_celsius(f_value: i32) {
    let c_value = (f_value - 32) * 5 / 9;
    println!("{} °F = {} °C", f_value, c_value);
}

fn main() {
    loop {
        let mut option = String::new();
        let mut temperature = String::new();

        println!("Type an value to convert (Example: \"-5\", \"21\", \"70\")");
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: i32 = match temperature.trim().parse::<i32>() {
            Ok(value) => value,
            Err(_) => continue,
        };

        println!(
            "Type an option to convert \"{}\" or (3) to exit:\n (1) Celsius to Fahrenheit\n (2) Fahrenheit to Celsius\n (3) Exit",
            temperature
        );
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        match option.trim() {
            "1" => celsius_to_fahrenheit(temperature),
            "2" => fahrenheit_to_celsius(temperature),
            "3" => break,
            _ => {
                println!("\"{}\" is not a valid option!", option.trim());
                continue;
            }
        }
    }
}
