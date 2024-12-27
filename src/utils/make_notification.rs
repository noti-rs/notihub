use notify_rust::Notification;

pub trait MakeNotification {
    fn make_notification(self) -> Option<Notification>;
}
