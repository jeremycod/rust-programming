use notification_system::{EmailNotifier, Notifier, SmsNotifier};
use std::any::Any;

fn process_notifications(notifications: &[&dyn Notifier], recipient: &str, message: &str) {
    for notification in notifications {
        notification.send(recipient, message);
    }
}
fn configure_notifier<N: Notifier>(notifier: &mut N, config: String){
    println!("Configuring notifier with: {}", config);
}
fn main() {
    println!("Hello, world!");
    let mut notifiers: Vec<Box<dyn Notifier>> = Vec::new();
    let mut email_notifier = EmailNotifier {
        sender_address: String::from("john.doe@example.com")
    };


    let mut sms_notifier = SmsNotifier {
        phone_number: String::from("123-456-7890")
    };
    notifiers.push(Box::new(email_notifier));
    notifiers.push(Box::new(sms_notifier));

    for notifier_box in notifiers.iter_mut() {
        let notifier: &mut dyn Any = notifier_box.as_mut();
        let message = "some message";
        if let Some(email_notifier) = notifier.downcast_mut::<EmailNotifier>(){
            configure_notifier(email_notifier, String::from("email configuration"));
            email_notifier.send("email recipient", &message);
        } else if let Some(sms_notifier) = notifier.downcast_mut::<SmsNotifier>(){
            configure_notifier(sms_notifier, String::from("sms configuration"));
            sms_notifier.send("sms recipient", &message);
        } else {
            println!("Unknown type. No configuration");
        }



    }
}
