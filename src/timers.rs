use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

use crate::json_serializable::JsonSerializable;
use crate::notify;
use crate::settings::Settings;
use crate::sound::*;
use crate::storage::Session;
use crate::storage::SessionList;
use crate::storage::Storage;

/// Represents the values of a timer, as well as the time worked in minutes.
///
/// # Examples
///
/// ```
/// let timer = Timer { work_minutes: 25, break_minutes: 10, total_worked_minutes: 0 };
/// println!("Current work/break distribution: {}/{}", timer.work_minutes, timer.break_minutes);
/// println!("Time worked in total: {}", timer.total_worked_minutes);
/// assert_eq!(timer.work_minutes, 25);
/// assert_eq!(timer.break_minutes, 10);
/// assert_eq!(timer.total_worked_minutes, 0);
/// ```
#[derive(Debug)]
pub struct Timer {
    pub work_minutes: u64,
    pub break_minutes: u64,
    pub total_worked_minutes: u64,
}

impl Timer {
    /// Creates a new `Timer` instance with the specified work and break durations.
    pub fn new(work_minutes: u64, break_minutes: u64, total_worked_minutes: u64) -> Timer {
        Timer {
            work_minutes,
            break_minutes,
            total_worked_minutes,
        }
    }

    /// Adds a number of minutes to the total number of minutes worked.
    pub fn add_worked_minutes(&mut self, minutes: u64) {
        self.total_worked_minutes += minutes;
    }

    /// Sets the work minutes to the amount specified in the argument.
    pub fn set_work_minutes(&mut self, minutes: u64) {
        self.work_minutes = minutes;
    }

    /// Sets the break minutes to the amount specified in the argument.
    pub fn set_break_minutes(&mut self, minutes: u64) {
        self.break_minutes = minutes;
    }
}

pub fn pomodoro_work_timer(timer: &mut Timer, settings: &Settings) {
    // convert the input time to seconds
    let time_to_sec = &timer.work_minutes * 60;

    let bar = ProgressBar::new(time_to_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Time Remaining {bar:.40.cyan/gray}] {msg}",
        )
        .unwrap()
        .progress_chars("█▓▒░"),
    );

    for elapsed in 0..time_to_sec {
        let remaining = time_to_sec - elapsed;

        let min = remaining / 60;
        let sec = remaining % 60;

        let min_formatted = format!("{:02}", min);
        let sec_formatted = format!("{:02}", sec);

        bar.set_message(format!("{min_formatted}:{sec_formatted}"));
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }

    println!("✅ Pomodoro Timer completed\n");

    if settings.notification.enable {
        notify::send_notification_work();
    }

    play_sound(POMODORO_FINISH.to_vec(), 2);

    //increment the time worked
    timer.add_worked_minutes(timer.work_minutes);
}

pub fn pomodoro_break_timer(timer: &Timer, session_list: &mut SessionList, settings: &Settings) {
    let break_time_sec = timer.break_minutes * 60;

    let bar = ProgressBar::new(break_time_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Time Remaining {bar:.40.cyan/gray}] {msg}",
        )
        .unwrap()
        .progress_chars("█▓▒░"),
    );

    for elapsed in 0..break_time_sec {
        let remaining = break_time_sec - elapsed;

        let min = remaining / 60;
        let sec = remaining % 60;

        let min_formatted = format!("{:02}", min);
        let sec_formatted = format!("{:02}", sec);

        bar.set_message(format!("{min_formatted}:{sec_formatted}"));
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }

    let session = Session::new(
        Some(Utc::now()),
        timer.work_minutes as u32,
        timer.break_minutes as u32,
    );

    let storage = Storage::new(None, "sessions.json".to_string());

    session_list.append(session);

    match storage.write(session_list.to_json()) {
        Ok(_) => (),
        Err(v) => panic!("There was an error while writing to file. {}", v),
    }

    println!("✅ Break is completed\n");

    if settings.notification.enable {
        notify::send_notification_break();
    }

    play_sound(BREAK_FINISH.to_vec(), 2);
}
