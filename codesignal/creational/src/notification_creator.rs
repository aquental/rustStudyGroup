use crate::notification::{EmailNotification, Notification, SmsNotification};

// TODO: Create a trait named NotificationCreator with a method named 'create_notification'.
pub trait NotificationCreator {
    fn create_notification(&self) -> Box<dyn Notification>;
}

// TODO: Create a struct named EmailNotificationCreator that implements the NotificationCreator trait
// and returns a new EmailNotification.
pub struct EmailNotificationCreator;
impl NotificationCreator for EmailNotificationCreator {
    fn create_notification(&self) -> Box<dyn Notification> {
        Box::new(EmailNotification)
    }
}

// TODO: Create a struct named SmsNotificationCreator that implements the NotificationCreator trait
// and returns a new SmsNotification.
pub struct SmsNotificationCreator;
impl NotificationCreator for SmsNotificationCreator {
    fn create_notification(&self) -> Box<dyn Notification> {
        Box::new(SmsNotification)
    }
}
