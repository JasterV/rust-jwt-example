pub mod schemas;
use schemas::{Role, User};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DB {
    pub users: HashMap<String, User>,
}

impl DB {
    pub fn new() -> Self {
        let mut users = HashMap::new();
        users.insert(
            String::from("1"),
            User {
                uid: String::from("1"),
                email: String::from("user@userland.com"),
                pw: String::from("1234"),
                role: Role::User,
            },
        );
        users.insert(
            String::from("2"),
            User {
                uid: String::from("2"),
                email: String::from("admin@adminaty.com"),
                pw: String::from("4321"),
                role: Role::Admin,
            },
        );
        Self { users }
    }
}
