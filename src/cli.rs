use crate::session::SessionList;
use crate::settings::Settings;
use crate::timers::Timer;
use crate::ui;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Option<Command>, // Allow running with no subcommand
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        work: Option<u64>,

        // `break_` because `break` is a reserved keyword
        #[arg(long)]
        break_: Option<u64>,
    },
}

pub fn parse_opts(sessions: &mut SessionList, settings: &mut Settings) {
    let opts = Opts::parse();

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
            ui::start_cycle(&mut timer, sessions);
        }
        None => {
            ui::ui_loop(sessions, settings);
        }
    }
}
