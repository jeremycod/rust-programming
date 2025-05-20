use std::any::Any;

pub trait Notifier: Any {
    fn send(&self, recipient: &str, message: &str);
}

pub struct EmailNotifier {
    pub sender_address: String
}
pub struct SmsNotifier {
    pub phone_number: String
}
pub struct UserSpecificNotifier {
    user_id: u32,
    notifier: Box<dyn Notifier>
}

impl Notifier for EmailNotifier {
    fn send(&self, recipient: &str, message: &str) {
        println!("Sending email from {} to {}: {}",
                 self.sender_address, recipient, message);
    }
}

impl Notifier for SmsNotifier {
    fn send(&self, recipient: &str, message: &str) {
        println!("Sending SMS from {} to {}: {}",
                 self.phone_number, recipient, message);
    }
}
impl UserSpecificNotifier {
    fn notify(&self, message: &str){
        println!("Preparing notification for user {}", self.user_id);
        self.notifier.send(&self.get_recipient(), message);
        println!("Notification process completed for user {}", self.user_id);
    }
    // Assume we have a way to get the recipient based on user_id
    fn get_recipient(&self) -> String {
        // In a real system, this might involve looking up user data
        format!("user_{}@example.com", self.user_id)
    }
}
