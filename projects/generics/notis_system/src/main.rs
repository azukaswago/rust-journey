#[derive(Debug)]

enum NotificationType {
    Email(String),
    Sms,
    Push,
}

#[derive(Debug)]
struct Notification<T> {
    message: T,
    priority: u32,
}

impl Notification<NotificationType> {
    fn send(&self) {
        println!("Sending {:?}", self.message)
    }
}

impl<T> Notification<T> {
    fn get_priority(&self) -> u32 {
        self.priority
    }
}

fn main() {
    let notis_one = Notification {
        message: "on the roads i use my shank, in jail, i use my razor",
        priority: 7,
    };

    let notis_two = Notification {
        message: String::from("Azuka might be the GOAT"),
        priority: 10,
    };

    let notis_three = Notification {
        message: NotificationType::Email(String::from("Elfrida is my GOAT")),
        priority: 10,
    };

    notis_three.send();

    let priority_one = notis_one.get_priority();
    let priority_two = notis_two.get_priority();
    let priority_three = notis_three.get_priority();

    println!("Priority of notis_one: {}", priority_one);
    println!("Priority of notis_two: {}", priority_two);
    println!("Priority of notis_three: {}", priority_three);
}
