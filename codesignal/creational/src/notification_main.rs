mod notification;
mod notification_creator;

use notification_creator::{EmailNotificationCreator, NotificationCreator, SmsNotificationCreator};

fn main() {
    // TODO: Use the creator structs to create both types of notifications and call their 'send' methods.
    let email_creator = EmailNotificationCreator;
    let email_notification = email_creator.create_notification();
    email_notification.send();

    let sms_creator = SmsNotificationCreator;
    let sms_notification = sms_creator.create_notification();
    sms_notification.send();
}
