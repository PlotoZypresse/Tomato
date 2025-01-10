mod menu;
mod timers;
mod ui;

/// Represents the values of a timer, as well as the time worked in minutes.
///
/// # Examples
///
/// ```
/// let timer = Timer { work_time: 25, break_time: 10, time_worked 0 };
/// println!("Current work/break distribution: {}/{}", timer.work_time, timer.break_time);
/// println!("Time worked in total: {}", timer.time_worked);
/// assert_eq!(timer.work_time, 25);
/// assert_eq!(timer.break_time, 10);
/// assert_eq!(timer.time_worked, 0);
/// ```

fn main() {
    ui::ui_loop();
}
