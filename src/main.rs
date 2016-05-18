use std::io;


// simple input function for functions

fn get_param(s: &str) -> f64 {
    println!("{}", s);
    
    loop {
        println!("Enter a floating point number!");
        let mut input = String::new();
    
        io::stdin().read_line(&mut input)
                .expect("failed to read line");
    
        let input: f64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        return input;
    }
}

fn calc_speed() -> f64 {
    let distance: f64   = get_param("Enter distance:");
    let time: f64       = get_param("Enter time:");
    println!("Speed = distance / time");
    println!("Speed = {} / {}", distance, time);
        distance / time
}

fn calc_time() -> f64 {
    let speed: f64      = get_param("Enter speed:");
    let distance: f64   = get_param("Enter distance:");
    println!("Time = distance / speed");
    println!("Time = {} / {}", distance, speed);
        distance / speed
}

fn calc_distance() -> f64 {
    let speed: f64      = get_param("Enter speed:");
    let time: f64       = get_param("Enter time:");
    println!("Distance = speed * time");
    println!("Distance = {} * {}", speed, time);
        speed * time
}


fn main() {   
    let mut calc_input = String::new();
    
    println!("Hello, would you like to calculate speed, distance or time?");
    
    io::stdin().read_line(&mut calc_input)
            .expect("failed to read line");
    
    let answer: f64;
    
    match calc_input.trim() {
        "speed" | "Speed"       => {
            answer = calc_speed();
            println!("Speed = {}", answer);
        }
        "distance" | "Distance" => {
            answer = calc_distance();
            println!("Distance = {}", answer);
        }
        "time" | "Time"         => {
            answer = calc_time();
            println!("Time = {}", answer);
        }
        _                       => {
            println!("Enter speed, distance or time!")
        }
    }
    
}