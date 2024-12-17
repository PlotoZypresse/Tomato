use std::thread;
use std::time::Duration;
use std::io;

fn main() {
    // get user input
    let input = user_input();
    
    //start timer with input time
    timer(input);
}

fn timer(time: u64) {
    // convert the input time to seconds
    let time_to_sec = time * 60;

    //set the timer
    thread::sleep(Duration::from_secs(time_to_sec));

    println!("{time} minutes have passed");
} 

fn user_input() -> u64 {
    println!("How long should the Pomodoro timer last?");
    println!("Please input in minutes: ");

    let mut input = String::new();
    
    // read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    //parsing the input to an integer
    let number: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! please input a positiv integer");
            return 0;
        }
    };

    return number;
}
