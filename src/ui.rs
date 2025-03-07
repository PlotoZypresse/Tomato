use crate::migration::{is_correct_version, migrate_settings};
use crate::settings::SETTINGS_VERSION;
use crate::{
    json_serializable::JsonSerializable,
    menu,
    session::SessionList,
    settings::Settings,
    storage::Storage,
    timers::{self, Timer},
};
use crossterm::{cursor, execute, terminal};
use std::io;

pub fn ui_loop(sessions: &mut SessionList, settings: &mut Settings) {
    let settings_json = Storage::new(Some(".tomato".to_string()), "settings.json".to_string());
    let settings_json = settings_json.read().unwrap();

    let checked_settings: &mut Settings =
        if !is_correct_version(settings_json.as_str(), SETTINGS_VERSION) {
            &mut migrate_settings(settings_json.as_str())
        } else {
            settings
        };

    loop {
        if ui(sessions, checked_settings) == 9 {
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

fn user_text_input() -> String {
    loop {
        let mut input_text = String::new();

        if io::stdin().read_line(&mut input_text).is_ok() {
            return input_text.trim().to_string();
        } else {
            println!("Failed to read input. Please try again.");
        }
    }
}

pub fn user_input(timer: &mut Timer, settings: &mut Settings) {
    // time input for timer time
    println!("How long should the Pomodoro timer last?");
    println!("Please input in minutes: ");

    let input_work: u64 = get_number_from_input();

    // time input for break time
    println!("How long should the breaks be?");
    println!("Please input in minutes: ");

    let input_break: u64 = get_number_from_input();

    timer.set_work_minutes(input_work);
    timer.set_break_minutes(input_break);

    let file_name = String::from("settings.json");
    let settings_storage = Storage::new(None, file_name);

    settings.work_time = input_work;
    settings.break_time = input_break;

    settings_storage
        .write(settings.to_json())
        .expect("Something went wrong while trying to write to settings.json");
}

fn ui(session_list: &mut SessionList, settings: &mut Settings) -> u64 {
    let total_minutes = session_list.total_work_minutes();

    let mut timer = Timer::new(settings.work_time, settings.break_time, total_minutes);

    loop {
        menu::print_menu(settings);

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
                user_input(&mut timer, settings);
                get_input_before_going_back_to_menu();
            }
            2 => {
                start_cycle(&mut timer, session_list, settings);
                get_input_before_going_back_to_menu();
            }
            3 => {
                stats(&mut timer);
                get_input_before_going_back_to_menu();
            }
            4 => {
                println!("Please input your desired notification for getting work done");
                let work_msg = user_text_input();
                println!("Please input your desired notification for getting back to work.");
                let break_msg = user_text_input();

                let file_name = String::from("settings.json");
                let settings_storage = Storage::new(None, file_name);

                settings.notification.work_msg = work_msg;
                settings.notification.break_msg = break_msg;

                settings_storage
                    .write(settings.to_json())
                    .expect("Something went wrong while trying to write to settings.json");

                println!("Individaul notification messages set!")
            }
            5 => {
                if settings.notification.enable {
                    println!("To turn off notifications type 0 and press enter.");
                    let toggle = get_number_from_input();
                    if toggle == 0 {
                        settings.notification.enable = false;
                    } else {
                        println!("Notification settings not changed try again.")
                    }
                } else {
                    println!("To turn on notifications type 1 and press enter.");
                    let toggle = get_number_from_input();
                    if toggle == 1 {
                        settings.notification.enable = true;
                    } else {
                        println!("Notification settings not changed try again.")
                    }
                }
            }
            9 => {
                println!("Exiting...");
                return 9;
            }
            _ => println!("Invalid option. Please try again.\n"),
        }
    }
}

pub fn start_cycle(timer: &mut Timer, sessions: &mut SessionList, settings: &mut Settings) {
    execute!(
        std::io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    println!("\nStarting Pomodoro timer...");
    timers::pomodoro_work_timer(timer, settings);
    println!("...Press Enter to start the break...");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
    timers::pomodoro_break_timer(timer, sessions, settings);
}

pub fn stats(timer: &mut Timer) {
    let minutes = timer.total_worked_minutes;

    println!(
        "You've worked for {} days, {} hours and {} minutes.",
        (minutes / (60 * 24)), // Automatically rounds down
        (minutes % (60 * 24)) / 60,
        minutes % 60
    );

    if minutes == 0 {
        println!("It's almost better than nothing!");
    } else if minutes <= 25 {
        println!("It's better than nothing!");
    } else {
        println!("Good job!");
    }
}

// TODO: Get a new name for this function.
fn get_input_before_going_back_to_menu() {
    println!("\nPress Enter to return to the menu.");
    let mut dummy = String::new();
    io::stdin().read_line(&mut dummy).unwrap();
}
