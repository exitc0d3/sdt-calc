/* Calculate speed distance and time with the other two variables in the equation */
/* Written by Finn Fallowfield: Digital Native                                    */

use std::io;
mod my_input;

fn calc_speed() {
    //calculate speed
    let distance = my_input::prompt_for_value("distance");
    let time     = my_input::prompt_for_value("time");
    println!("Speed = distance / time"); // Working
    println!("Speed = {} / {}", distance, time);
    println!("Speed = {}", distance / time);
}

fn calc_time() {
    //calculate time
    let speed    = my_input::prompt_for_value("speed");
    let distance = my_input::prompt_for_value("distance");
    println!("Time = distance / speed"); // Working
    println!("Time = {} / {}", distance, speed);
    println!("Time = {}", distance / speed);
}

fn calc_distance() {
    //calculate distance
    let speed = my_input::prompt_for_value("speed");
    let time  = my_input::prompt_for_value("time");
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