use crate::session::SessionList;
use crate::settings::Settings;

mod cli;
mod json_serializable;
mod menu;
mod notify;
mod session;
mod settings;
mod sound;
mod storage;
mod timers;
mod ui;

fn main() {
    let mut sessions =
        SessionList::load_sessions(".tomato".to_string(), "sessions.json".to_string());
    let mut settings = Settings::load_settings(".tomato".to_string(), "settings.json".to_string());

    cli::parse_opts(&mut sessions, &mut settings);
}
