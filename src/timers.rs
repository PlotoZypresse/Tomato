use chrono::Utc;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

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

    /// Resets the total number of worked minutes for the current timer.
    pub fn reset(&mut self) {
        self.total_worked_minutes = 0;
    }

    /// Adds a number of minutes to the total number of minutes worked.
    pub fn add_worked_minutes(&mut self, minutes: u64) {
        self.total_worked_minutes += minutes;
    }

    /// Returns the current distribution of work and break minutes, along with
    /// the total number of minutes worked, formatted in a `String`.
    pub fn display(&self) -> String {
        format!(
            "Work: {}m, Break: {}m, Total Worked: {}m",
            self.work_minutes, self.break_minutes, self.total_worked_minutes
        )
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

pub fn pomodoro_work_timer(timer: &mut Timer, session_list: &mut SessionList) {
    // convert the input time to seconds
    let time_to_sec = &timer.work_minutes * 60;

    let bar = ProgressBar::new(time_to_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Time Remaining {bar:.40.cyan/gray}] {pos}/{len}s",
        )
        .unwrap()
        .progress_chars("█▓▒░"),
    );

    for _ in 0..time_to_sec {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }

    // Get an output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // load the sound file
    let file = BufReader::new(File::open("./sounds/pomodoroFinish.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();

    let session = Session::new(
        Some(Utc::now()),
        timer.work_minutes as u32,
        timer.break_minutes as u32,
    );

    let storage = Storage::new(None, "sessions.json".to_string());

    session_list.append(session);

    match storage.write(None, session_list.to_json()) {
        Ok(_) => (),
        Err(v) => panic!("There was an error while writing to file. {}", v),
    }

    println!("✅ Pomodoro Timer completed\n");

    //increment the time worked
    timer.add_worked_minutes(timer.work_minutes);

    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}

pub fn pomodoro_break_timer(timer: &Timer) {
    let break_time_sec = timer.work_minutes * 60;

    let bar = ProgressBar::new(break_time_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Break Remaining {bar:.40.cyan/gray}] {pos}/{len}s",
        )
        .unwrap()
        .progress_chars("█▓▒░"),
    );

    for _ in 0..break_time_sec {
        thread::sleep(Duration::from_secs(1));
        bar.inc(1);
    }
    // Get an output stream handle to the default sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // load the sound file
    let file = BufReader::new(File::open("./sounds/breakDone.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();
    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    println!("✅ Break is completed\n");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
