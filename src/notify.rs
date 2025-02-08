use notify_rust::Notification;

pub fn send_notification() {
    let _ = Notification::new()
        .summary("Test1")
        .body("Test2")
        .icon("test")
        .show();
}
