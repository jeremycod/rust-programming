struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1: {}", user1.email);
    user1.email = String::from("test@example.com");
    println!("user1: {}", user1.email);
    let user2 = User {
        active: false,
        ..user1
    };
    println!("user2: {}", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
