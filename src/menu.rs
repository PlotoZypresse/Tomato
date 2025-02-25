use crossterm::style::Stylize;
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

use crate::settings::Settings;

pub fn print_menu(settings: &mut Settings) {
    execute!(
        io::stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )
    .unwrap();

    // Title and Borders
    println!(
        "{}",
        "==================================================".green()
    );
    println!("{}", "    Tomato - A Terminal Pomodoro Timer".bold().red());
    println!(
        "{}",
        "==================================================".green()
    );

    // Menu Instructions
    println!();
    println!("{}", "Please choose an option:".cyan());
    println!();

    // Menu options
    println!("1. Set time for work and break time");
    println!(
        "2. Start timer ({}/{})",
        settings.work_time, settings.break_time
    );
    println!("3. Stats");
    println!("4. Notifications");
    println!("{}", "9. Exit".red());

    // User choice prompt
    println!();
    print!("{}", "Enter your choice: ".cyan());
    io::stdout().flush().unwrap(); // Make sure the prompt appears
}
