use std::io;

/* This function will simplify input later on and reduce repetition */
fn prompt_for_value(value_name: &str) -> f64 {
    println!("Enter {}:", value_name);

    loop { // Loop until user enters a float and loop is broken
        println!("Enter a floating point number!");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        if let Ok(num) = input.trim().parse() {
            return num;
        }
    }
}

fn calc_speed() {
    let distance = prompt_for_value("distance");
    let time     = prompt_for_value("time");
    println!("Speed = distance / time"); // Working
    println!("Speed = {} / {}", distance, time);
    println!("Speed = {}", distance / time);
}

fn calc_time() {
    let speed    = prompt_for_value("speed");
    let distance = prompt_for_value("distance");
    println!("Time = distance / speed"); // Working
    println!("Time = {} / {}", distance, speed);
    println!("Time = {}", distance / speed);
}

fn calc_distance() {
    let speed = prompt_for_value("speed");
    let time  = prompt_for_value("time");
    println!("Distance = speed * time"); // Working
    println!("Distance = {} * {}", speed, time);
    println!("Distance = {}", speed * time);
}

fn main() {
    println!("Hello, would you like to calculate speed, distance or time?");

    let mut calc_input = String::new();
    io::stdin().read_line(&mut calc_input).expect("failed to read line"); // Get input

    match calc_input.to_lowercase().trim() { // Convert string to lowercase to catch all inputs
        "speed"    => calc_speed(),
        "distance" => calc_distance(),
        "time"     => calc_time(),
        _          => println!("Enter speed, distance or time!"), // Catch wrong input
    }
}
