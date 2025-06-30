// TODO: Create a trait named Notification with a method named 'send'.
pub trait Notification {
    fn send(&self);
}
// TODO: Create a struct named EmailNotification that implements the Notification trait
// and the 'send' method to output "Sending Email notification."
pub struct EmailNotification;

impl Notification for EmailNotification {
    fn send(&self) {
        println!("Sending Email notification.");
    }
}

// TODO: Create a struct named SmsNotification that implements the Notification trait
// and the 'send' method to output "Sending SMS notification."
pub struct SmsNotification;

impl Notification for SmsNotification {
    fn send(&self) {
        println!("Sending SMS notification.");
    }
}
