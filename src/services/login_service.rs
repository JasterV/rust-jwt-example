use super::jwt_service::JwtService;
use crate::api::LoginResponse;
use crate::db::DB;
use crate::error::Error;
use crate::models::user::{find_user_by_credentials, Credentials};
use crate::utils::Result;

pub fn login(
    db: &DB,
    jwt_service: &JwtService,
    credentials: &Credentials,
) -> Result<LoginResponse> {
    match find_user_by_credentials(&db, &credentials) {
        Some(user) => {
            let token = jwt_service.create_jwt(&user.uid, &user.role)?;
            Ok(LoginResponse { token })
        }
        None => Err(Error::WrongCredentialsError),
    }
}
