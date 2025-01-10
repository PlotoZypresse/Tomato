use crate::{
    menu,
    timers::{self, Timer},
};
use crossterm::{cursor, execute, terminal};
use std::io;

pub fn ui_loop() {
    loop {
        if ui() == 9 {
            break;
        }
    }
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

    // parsing the input to an integer
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

    // parsing the input to an integer
    let number_break: u64 = match input_break.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input! please input a positiv integer");
            return;
        }
    };

    timer.work_minutes = number_time;
    timer.break_minutes = number_break;
}

fn ui() -> u64 {
    let mut timer = Timer {
        work_time: 25,
        break_time: 5,
        time_worked: 0,
    };

    loop {
        menu::print_menu();

        // read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        //parsing the input to an integer
        let input: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input! please input a positive integer");
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
                    cursor::MoveTo(0, 0)
                )
                .unwrap();

                println!("\nStarting Pomodoro timer...");
                timers::pomodoro_work_timer(&mut timer);
                println!("...Press Enter to start the break...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
                timers::pomodoro_break_timer(&timer);
                println!("\nPress Enter to return to the menu.");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
            3 => {
                println!(
                    "You have worked for {} minutes this session good job!!!",
                    timer.total_worked_minutes
                );
                println!("...Press Enter to return to the menu...");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
            9 => {
                println!("Exiting...");
                return 9;
            }
            _ => println!("Invalid option. Please try again.\n"),
        }
    }
}
