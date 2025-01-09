use crate::Timer;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

pub fn pomodor_work_timer(timer: &mut Timer) {
    // convert the input time to seconds
    let time_to_sec = &timer.work_time * 60;

    let bar = ProgressBar::new(time_to_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Time Remainng {bar:.40.cyan/gray}] {pos}/{len}s",
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
    timer.time_worked = timer.time_worked + timer.work_time;

    //Play the sound
    let _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(2));
}

pub fn pomodoro_break_timer(timer: &Timer) {
    let break_time_sec = timer.work_time * 60;

    let bar = ProgressBar::new(break_time_sec);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.cyan} 🍅 [Break Remainng {bar:.40.cyan/gray}] {pos}/{len}s",
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
