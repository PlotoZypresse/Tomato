use indicatif::{ProgressBar, ProgressStyle};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

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
pub struct Timer {
    pub work_minutes: u64,
    pub break_minutes: u64,
    pub total_worked_minutes: u64,
}

pub fn pomodoro_work_timer(timer: &mut Timer) {
    // convert the input time to seconds
    let time_to_sec = &timer.work_time * 60;

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
    let file = BufReader::new(File::open("sounds/pomodoroFinish.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();

    println!("✅ Pomodoro Timer completed\n");

    //increment the time worked
    timer.total_worked_minutes = timer.total_worked_minutes + timer.work_minutes;

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
    let file = BufReader::new(File::open("sounds/breakDone.mp3").unwrap());
    //decode sound file into a source
    let source = Decoder::new(file).unwrap();
    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    println!("✅ Break is completed\n");

    std::thread::sleep(std::time::Duration::from_secs(2));
}
