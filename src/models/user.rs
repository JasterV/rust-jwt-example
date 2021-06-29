use crate::db::{schemas::User, DB};

pub struct Credentials {
    pub email: String,
    pub pwd: String,
}

pub fn find_user_by_credentials(db: &DB, credentials: &Credentials) -> Option<User> {
    db.users
        .iter()
        .find(|(_uid, user)| user.email == credentials.email && user.pw == credentials.pwd)
        .map(|(_, user)| user.clone())
}