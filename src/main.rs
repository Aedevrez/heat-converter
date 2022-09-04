use std::io;

fn main() {
    loop {
        //part where we decide the operation
        println!("Which converter would you like to use?\n1: Fahrenheit to Celsius\n2: Celsius to Fahrenheit\nPlease type 1\\2");
        let mut operator:String = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line!");
        let operator_but_int: i32 = match operator.trim().parse() {
            Ok(operator_but_int) => operator_but_int,
            Err(_) => continue
        };
        //part where we read input
        println!("Please enter the value:");
        let mut entered_value:String = String::new();
            io::stdin()
                .read_line(&mut entered_value)
                .expect("Failed to read your line!");
        let entered_value_but_trimmed = entered_value.trim();
        let converted_value: f32 = match entered_value.trim().parse() {
            Ok(converted_value) => converted_value,
            Err(_) => continue,
        };
        //it's calculatin time
        if operator_but_int == 1 {
            let converted_value: f32 = 5.0/9.0 * (converted_value - 32.0);
            println!("{} Fahrenheit equals to {} Celsius!", entered_value_but_trimmed, converted_value);
            break;
        } else if operator_but_int == 2 {
            let converted_value: f32 = converted_value * 9.0 / 5.0 + 32.0;
            println!("{} Celsius equals to {} Fahrenheit!", entered_value_but_trimmed, converted_value);
            break;
        } else {
            println!("Please enter a valid operation!");
            continue;
        }
    }
}

//i am learning how to use git properly