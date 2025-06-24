// Define a NotificationService trait
trait NotificationService {
    fn notify(&self, message: &str);
}

struct EmailNotification;
// Implement the NotificationService trait for EmailNotification
impl NotificationService for EmailNotification {
    fn notify(&self, message: &str) {
        println!("Email sent: {}", message);
    }
}

struct BookingService<'a> {
    notification_service: Box<dyn NotificationService + 'a>,
}
impl<'a> BookingService<'a> {
    fn new(service: Box<dyn NotificationService + 'a>) -> Self {
        BookingService {
            notification_service: service,
        }
    }

    fn process_booking(&self, message: &str) {
        self.notification_service.notify(message);
    }
}

fn main() {
    let email_service = EmailNotification;
    let booking_service = BookingService::new(Box::new(email_service));
    booking_service.process_booking("Your booking is confirmed!");
}
