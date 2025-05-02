use crate::models::user::User;

pub fn create_user(email: String, name: String) -> User {
    User::create_user(email, name)
}