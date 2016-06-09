use std::io;

pub fn prompt_for_value(value_name: &str) -> f64 {
    println!("Enter {}:", value_name);

    loop { // Loop until user enters a float and loop is broken
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        if let Ok(num) = input.trim().parse() {
            return num;
        }
        else {println!("Enter a floating point number!");}
    }
}