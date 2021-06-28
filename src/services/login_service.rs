use super::jwt_service::JwtService;
use crate::api::LoginResponse;
use crate::error::Error;
use crate::models::user::UserModel;
use crate::utils::Result;

#[derive(Debug, Clone)]
pub struct LoginService {
    jwt: JwtService,
    users: UserModel,
}

impl LoginService {
    pub fn new(jwt_service: JwtService, user_model: UserModel) -> Self {
        Self {
            jwt: jwt_service,
            users: user_model,
        }
    }

    pub fn login(&self, email: String, pwd: String) -> Result<LoginResponse> {
        match self.users.find_user_by_credentials(email, pwd) {
            Some((uid, user)) => {
                let token = self.jwt.create_jwt(uid, &user.role)?;
                Ok(LoginResponse { token })
            }
            None => Err(Error::WrongCredentialsError),
        }
    }
}
