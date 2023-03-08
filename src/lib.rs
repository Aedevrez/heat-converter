use std::io;

pub enum ConvMode {
    CtoF,
    FtoC,
    None,
}

pub struct Degree {
    pub value: i64,
    pub mode: ConvMode,
}

pub fn input_checker(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return input_reminder(); //Temporary
    } else if args.len() > 2 {
        return Err("Too many arguments!");
    } else {
        let degree = args[1].clone();
        Ok(degree)
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

pub fn degree_parser(value: &String) -> i64 {
    let degree = (*value).clone().parse::<i64>().expect("Failed to parse input!");
    degree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let string = String::from("123");
        assert_eq!(123, degree_parser(&string));
    }
}
