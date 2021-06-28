use crate::db::{schemas::User, DB};

#[derive(Debug, Clone)]
pub struct UserModel {
    db: DB,
}

impl UserModel {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub fn find_user_by_credentials(&self, email: String, pwd: String) -> Option<(&String, &User)> {
        self.db
            .users
            .iter()
            .find(|(_uid, user)| user.email == email && user.pw == pwd)
    }
}
