use crate::session::SessionList;
use crate::settings::Settings;
use crate::timers::Timer;
use crate::ui;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about = "A simple pomodoro application.",
    long_about = "This program runs pomodoro work/break cycles, with statistics and settings.\n\
                  For any command, be aware that there can be multiple parameters, e.g. see `tomato run --help`"
)]
struct Opts {
    #[command(subcommand)]
    command: Option<Command>, // Allow running with no subcommand
}

#[derive(Subcommand)]
enum Command {
    /// Runs a Pomodoro session with configurable work/break times.
    Run {
        #[arg(long, help = "Duration of work")]
        work: Option<u64>,

        // `break_` because `break` is a reserved keyword
        #[arg(long, help = "Duration of break")]
        break_: Option<u64>,
    },
    /// Change the default work/break times.
    SetDefaults {},
    /// Show the statistics for your pomodoro sessions.
    Stats {},
}

pub fn parse_opts(sessions: &mut SessionList, settings: &mut Settings) {
    let opts = Opts::parse();

    let mut timer: Timer = Timer {
        work_minutes: settings.work_time,
        break_minutes: settings.break_time,
        total_worked_minutes: sessions.total_work_minutes(),
    };

    match &opts.command {
        Some(Command::Run {
            work: work_time,
            break_: break_time,
        }) => {
            let work_time = work_time.unwrap_or(settings.work_time);
            let break_time = break_time.unwrap_or(settings.break_time);
            let mut timer: Timer = Timer {
                work_minutes: work_time,
                break_minutes: break_time,
                total_worked_minutes: sessions.total_work_minutes(),
            };
            ui::start_cycle(&mut timer, sessions, settings);
        }
        Some(Command::SetDefaults {}) => {
            ui::user_input(&mut timer, settings);
        }
        Some(Command::Stats {}) => {
            ui::stats(&mut timer);
        }
        None => {
            ui::ui_loop(sessions, settings);
        }
    }
}
