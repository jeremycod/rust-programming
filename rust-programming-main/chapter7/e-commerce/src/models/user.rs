pub struct User {
    email: String,
    pub name: String
}

impl User {
    pub fn create_user(email: String, name: String) -> User {
        User {
            email,
            name
        }
    }
}