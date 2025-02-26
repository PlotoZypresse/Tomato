use crate::json_serializable::JsonSerializable;
use crate::{
    settings::{Notifications, Settings},
    storage::Storage,
};
use notify_rust::Notification;

fn load_settings() -> Settings {
    let folder = String::from(".tomato");
    let file_name = String::from("settings.json");

    let storage = Storage::new(Some(folder), file_name.clone());

    let contents = storage.read().unwrap_or_else(|_| {
        let settings = Settings::new(25, 5, Notifications::default());
        match storage.write(settings.to_json()) {
            Ok(_) => (),
            Err(v) => panic!(
                "An error occured while writing the settings to the settings file: {}",
                v
            ),
        }
        "{}".to_string()
    });

    if contents.is_empty() || contents == "{}" {
        Settings::new(25, 5, Notifications::default())
    } else {
        Settings::from_json(&contents).expect("Could not parse the contents of file.")
    }
}

pub fn send_notification_work() {
    let settings = load_settings();
    let _ = Notification::new()
        .summary(&settings.notification.work_msg)
        .icon("firefox")
        .show();
}

pub fn send_notification_break() {
    let settings = load_settings();
    let _ = Notification::new()
        .summary(&settings.notification.break_msg)
        .icon("firefox")
        .show();
}
