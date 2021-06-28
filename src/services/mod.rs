mod jwt_service;
mod login_service;

use crate::config::Config;
use crate::db::DB;
pub use crate::models::user::UserModel;
pub use jwt_service::JwtService;
pub use login_service::LoginService;

#[derive(Debug, Clone)]
pub struct AppServices {
    pub login_service: LoginService,
    pub jwt_service: JwtService,
}

impl AppServices {
    pub fn new(config: Config, db: DB) -> Self {
        let jwt_service = JwtService::new(config.jwt_secret.clone());
        let users_model = UserModel::new(db);
        let login_service = LoginService::new(jwt_service.clone(), users_model);
        AppServices {
            jwt_service,
            login_service,
        }
    }
}
