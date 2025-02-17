mod json_serializable;
mod menu;
mod migration;
mod session;
mod settings;
mod sound;
mod storage;
mod timers;
mod ui;

fn main() {
    ui::ui_loop();
}
