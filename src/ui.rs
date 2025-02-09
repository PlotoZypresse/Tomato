use crate::{
    json_serializable::JsonSerializable,
    menu,
    settings::Settings,
    storage::{SessionList, Storage},
    timers::{self, Timer},
};
use crossterm::{cursor, execute, terminal};
use std::io;

fn load_sessions() -> SessionList {
    let folder = String::from(".tomato");
    let file_name = String::from("sessions.json");

    let storage = Storage::new(Some(folder), file_name.clone());

    let contents = storage.read().unwrap_or_else(|_| "ERR".to_string());

    if contents.is_empty() || contents == "ERR" {
        return SessionList::new(None);
    }

    // Load from storage.
    let sessions =
        SessionList::from_json(&contents).expect("Could not parse the contents of file.");

    println!("{:?}", sessions);

    sessions
}

fn load_settings() -> Settings {
    let folder = String::from(".tomato");
    let file_name = String::from("settings.json");

    let storage = Storage::new(Some(folder), file_name.clone());

    let contents = storage.read().unwrap_or_else(|_| {
        let settings = Settings::new(25, 5);
        match storage.write(None, settings.to_json()) {
            Ok(_) => (),
            Err(v) => panic!(
                "An error occured while writing the settings to the settings file: {}",
                v
            ),
        }

        format!(
            "Could not read the contents of {}, creating a new file.",
            file_name
        )
    });

    if contents.is_empty() {
        return Settings::new(25, 5);
    }

    Settings::from_json(&contents).expect("Could not parse the contents of file.")
}

pub fn ui_loop() {
    let mut sessions = load_sessions();
    let mut settings = load_settings();

    loop {
        if ui(&mut sessions, &mut settings) == 9 {
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

fn user_input(timer: &mut Timer, settings: &mut Settings) {
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
        .write(None, settings.to_json())
        .expect("Something went wrong while trying to write to settings.json");
}

fn ui(session_list: &mut SessionList, settings: &mut Settings) -> u64 {
    let total_minutes = session_list.total_work_minutes();

    let mut timer = Timer::new(settings.work_time, settings.break_time, total_minutes);

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
                user_input(&mut timer, settings);
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
                timers::pomodoro_break_timer(&timer, session_list);
                println!("\nPress Enter to return to the menu.");
                let mut dummy = String::new();
                io::stdin().read_line(&mut dummy).unwrap();
            }
            3 => {
                let minutes = timer.total_worked_minutes;

                if minutes == 0 {
                    println!(
                        "You've worked for 0 minutes in total! It's almost better than nothing!"
                    );
                } else if minutes <= 25 {
                    println!(
                        "You've worked for {minutes} minutes in total! It's better than nothing!"
                    );
                } else {
                    println!("You've worked for {minutes} minutes! Good job!");
                }
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
