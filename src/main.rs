use std::thread;
use std::time::Duration;
use std::io;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};
use crossterm::{execute, terminal, cursor};
//use std::io::{stdout, Write};

struct Timer {
    work_time: u64,
    break_time: u64,
    time_worked: u64,
}

fn main() {
    loop {
        if ui() == 9 {
            break;
        }
    }
}

fn pomodor_work_timer(timer: &mut Timer) {
    // convert the input time to seconds
    let time_to_sec = &timer.work_time * 60;

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
    
    // Get an output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // load the sound file
    let file = BufReader::new(File::open("sounds/pomodoroFinish.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();

    println!("✅ Pomodoro Timer completed\n");

    //increment the time worked
    timer.time_worked = timer.time_worked + timer.work_time;

    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}

fn pomodoro_break_timer(timer: &Timer) {
    let break_time_sec = timer.work_time * 60;

    let bar = ProgressBar::new(break_time_sec);
    bar.set_style(
        ProgressStyle::with_template("{spinner:.cyan} 🍅 [Break Remainng {bar:.40.cyan/gray}] {pos}/{len}s")
        .unwrap()
        .progress_chars("█▓▒░")
    );

    for _ in 0..break_time_sec {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }
    // Get an output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // load the sound file
    let file = BufReader::new(File::open("sounds/breakDone.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();
    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    println!("✅ Break is completed\n");

    std::thread::sleep(std::time::Duration::from_secs(2));
}

fn user_input(timer: &mut Timer) {
    // time input for timer time
    println!("How long should the Pomodoro timer last?");
    println!("Please input in minutes: ");

    let mut input_time = String::new();

    // read user input
    io::stdin()
        .read_line(&mut input_time)
        .expect("Failed to read input");

    //parsing the input to an integer
    let number_time: u64 = match input_time.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! please input a positiv integer");
            return;
        }
    };

    // time input for break time
    println!("How long should the breaks be?");
    println!("Please input in minutes: ");

    let mut input_break = String::new();

    // read user input
    io::stdin()
        .read_line(&mut input_break)
        .expect("Failed to read input");

    //parsing the input to an integer
    let number_break: u64 = match input_break.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! please input a positiv integer");
            return;
        }
    };

    timer.work_time = number_time;
    timer.break_time = number_break;
}

fn ui() -> u64{
    let mut timer = Timer {
        work_time: 25,
        break_time: 5,
        time_worked: 0,
    };

    loop {
        execute!(
            std::io::stdout(),
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0,0)
        )
        .unwrap();

        //formated like this to print right
        println!(
            "===================================================
Tomato a terminal pomodoro timer written in rust
===================================================
Please choose an option:
1. Set time for work and break time
2. Start timer (Default 25/5)
3. Stats
9. Exit
Enter your choice: "
        );

        // read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        //parsing the input to an integer
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! please input a positiv integer");
                continue;
            }
        };

        match input {
            1 => {
                //time_tup =
                user_input(&mut timer);
                println!("Work and break timers set.\n");
            }
            2 => {
                execute!(
                    std::io::stdout(),
                    terminal::Clear(terminal::ClearType::All),
                    cursor::MoveTo(0,0)
                )
                .unwrap();

                println!("\nStarting Pomodoro timer...");
                pomodor_work_timer(&mut timer);
                println!("...Press Enter to start the break...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
                pomodoro_break_timer(&timer);
            }
            3 => {
                println!("You have worked for {} minutes this session good job!!!", timer.time_worked);
                println!("...Press Enter to return to the menu...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
            9 =>  {
                println!("Exiting...");
                return 9
            }
            _ => println!("Invalid option. Please try again.\n"),
        }
    }
}
