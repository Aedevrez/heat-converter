use std::io;

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
