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

fn get_number_from_input() -> u64 {
    loop {
        let mut input_time = String::new();

        if io::stdin().read_line(&mut input_time).is_ok() {
            // If the number can be successfully parsed into an u64 data type,
            // then return it.
            if let Ok(num) = input_time.trim().parse::<u64>() {
                return num;
            }
        }
        // If the number cannot succesfully be parsed, then ask the user to
        // try again, and run the loop again.
        println!("Invalid input! Please input a positive integer.")
    }
}

fn user_input(timer: &mut Timer) {
    // time input for timer time
    println!("How long should the Pomodoro timer last?");
    println!("Please input in minutes: ");

    let input_work: u64 = get_number_from_input();

    // time input for break time
    println!("How long should the breaks be?");
    println!("Please input in minutes: ");

    let input_break: u64 = get_number_from_input();

    timer.work_minutes = input_work;
    timer.break_minutes = input_break;

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
