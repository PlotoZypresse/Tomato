use notify_rust::Notification;

pub fn send_notification() {
    let _ = Notification::new()
        .summary("Work is done!!")
        .body("Well done being productive")
        .icon("tomato.jpeg")
        .show();
}
