use notify_rust::Notification;

pub fn send_notification_work() {
    let _ = Notification::new()
        .summary("Work is done!!")
        .body("Well done being productive")
        .icon("img/tomato.jpeg")
        .show();
}

pub fn send_notification_break() {
    let _ = Notification::new()
        .summary("Break is done!")
        .body("Get back to work")
        .icon("img/tomato.jpeg")
        .show();
}
