use std::io;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub enum ConvMode {
    CtoF,
    FtoC,
    None,
}

#[derive(Debug)]
pub struct Degree {
    pub value: i64,
    pub mode: ConvMode,
}

impl Degree {
    pub fn build(value: i64) -> Degree {
        let mode = match env::var("CF") {
            Ok(_) => match env::var("FC") {
                Ok(_) => ConvMode::None,
                Err(_) => ConvMode::CtoF,
            }
            Err(_) => match env::var("FC") {
                Ok(_) => ConvMode::FtoC,
                Err(_) => ConvMode::None,
            }
        };

        Degree {value, mode}
    }
}

pub fn input_checker(mut args: impl Iterator<Item = String>) -> Result<String, &'static str> {
    args.next();

    let result = match args.next() {
        Some(degree) => Ok(degree),
        None => input_reminder(),
    };

    match args.next() {
        Some(_) => Err("Too many arguments!"),
        None => result,
    }
}

pub fn input_reminder() -> Result <String, &'static str> {
    println!("You didn't enter a value as a command line argument.
What is the number you want to convert?");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Could not read from Command Line.");
    Ok(value.trim().to_string())
}

pub fn degree_parser(value: &String) -> Result<i64, Box<dyn Error>> {
    Ok((*value).clone().parse::<i64>()?)
}

pub fn run(num: i64) {
    let degree = Degree::build(num);
    match degree.mode {
        ConvMode::None => {
            print_fahrenheit(&degree);
            print_celsius(&degree);
        },
        ConvMode::CtoF => print_fahrenheit(&degree),
        ConvMode::FtoC => print_celsius(&degree),
    }
} //Temporary

pub fn c_to_f_converter(num: i64) -> f64 {
    ((num as f64) * 9.0 / 5.0) + 32.0
}

pub fn f_to_c_converter(num: i64) -> f64 {
    ((num as f64) - 32.0) * 5.0 / 9.0
}

pub fn print_fahrenheit(degree: &Degree) {
    let fahrenheit = c_to_f_converter(degree.value);
    println!("{} Celsius = {:.2} Fahrenheit", format!("{}", degree.value), fahrenheit);
}

pub fn print_celsius(degree: &Degree) {
    let celsius = f_to_c_converter(degree.value);
    println!("{} Fahrenheit = {:.2} Celsius", format!("{}", degree.value), celsius);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let string = String::from("123");
        assert_eq!(123, degree_parser(&string));
    }

    #[test]
    fn c_to_f_tester() {
        let val = 0;
        assert_eq!(32.0, c_to_f_converter(val));
    }

    #[test]
    fn f_to_c_tester() {
        let val = 32;
        assert_eq!(0.0, f_to_c_converter(val));
    }
}
