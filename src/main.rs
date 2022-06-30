use std::io;

fn main() {
    //part where we decide the operation
    println!("Which converter would you like to use?\n1 : Fahrenheit to Celsius\n2:Celsius to Fahrenheit\nPlease type 1\\2");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read line!");
    //write operations tomorrow.
}
