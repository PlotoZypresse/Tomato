use std::thread;
use std::time::Duration;
use std::io;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


fn main() {
    // get user input
    let input = user_input();
    
    //start timer with input time
    timer(input);
}

fn timer(time: u64) {
    // convert the input time to seconds
    let time_to_sec = time * 60;

    let bar = ProgressBar::new(time_to_sec);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.cyan} 🍅 [Time Remainng {bar:.40.cyan/gray}] {pos}/{len}s")
        .unwrap()
        .progress_chars("█▓▒░")
    );

    for _ in 0..time_to_sec {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }

    //set the timer
    //thread::sleep(Duration::from_secs(time_to_sec));
    
    // Get an output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // load the sound file
    let file = BufReader::new(File::open("sounds/pomodoroFinish.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();
    //Play the sound
    
   
    println!("✅ Pomodoro Timer completed");

    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(5));

    //bar.finish_with_message() :wq
    //
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
