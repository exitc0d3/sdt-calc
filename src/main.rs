use std::io;

fn prompt_for_value(value_name: &str) -> f64 {
    println!("Enter {}:", value_name);

    loop {
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
    println!("Speed = distance / time");
    println!("Speed = {} / {}", distance, time);
    println!("Speed = {}", distance / time);
}

fn calc_time() {
    let speed    = prompt_for_value("speed");
    let distance = prompt_for_value("distance");
    println!("Time = distance / speed");
    println!("Time = {} / {}", distance, speed);
    println!("Time = {}", distance / speed);
}

fn calc_distance() {
    let speed = prompt_for_value("speed");
    let time  = prompt_for_value("time");
    println!("Distance = speed * time");
    println!("Distance = {} * {}", speed, time);
    println!("Distance = {}", speed * time);
}

fn main() {
    println!("Hello, would you like to calculate speed, distance or time?");

    let mut calc_input = String::new();
    io::stdin().read_line(&mut calc_input).expect("failed to read line");

    match calc_input.to_lowercase().trim() {
        "speed"    => calc_speed(),
        "distance" => calc_distance(),
        "time"     => calc_time(),
        _          => println!("Enter speed, distance or time!"),
    }
}
