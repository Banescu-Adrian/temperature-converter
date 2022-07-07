use std::io;

fn main() {
    println!("Fahrenheit to Celsius converter");
    println!("Enter temperature in Fahrenheit!");

    let mut temperature_in_fahrenheit = String::new();

    io::stdin()
        .read_line(&mut temperature_in_fahrenheit)
        .expect("Failed to read line");

    let temperature_in_fahrenheit: f64 = temperature_in_fahrenheit
        .trim()
        .parse()
        .expect("Must be a number");

    let temperature_in_celsius: f64 = convert_fahrenheit_in_celsius(temperature_in_fahrenheit);

    println!("Temperature in celsius is: {temperature_in_celsius}");
}

fn convert_fahrenheit_in_celsius(temperate_in_fahrenheit: f64) -> f64 {
    (temperate_in_fahrenheit - 32.0) * 0.5556
}
