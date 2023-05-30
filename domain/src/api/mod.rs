use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
pub struct User {
    id: u64,
    username: String,
}

impl From<CreateUser> for User {
    fn from(value: CreateUser) -> Self {
        User {
            id: 1337,
            username: value.username,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
}