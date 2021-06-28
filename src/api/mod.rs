mod filters;
mod handlers;

use crate::db::schemas::Role;
use crate::services::AppServices;
use filters::auth::with_auth;
use handlers::{handle_rejection, hello_admin, hello_user, login_handler};
use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub pw: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    message: String,
    status: String,
}

pub fn routes(
    app_services: AppServices,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let jwt_service = app_services.jwt_service.clone();
    let login_service = app_services.login_service.clone();

    let login_route = warp::path!("login")
        .and(warp::post())
        .and(warp::any().map(move || login_service.clone()))
        .and(warp::body::json())
        .and_then(login_handler);

    let user_route = warp::path!("user")
        .and(with_auth(jwt_service.clone(), Role::User))
        .and_then(hello_user);

    let admin_route = warp::path!("admin")
        .and(with_auth(jwt_service.clone(), Role::Admin))
        .and_then(hello_admin);

    login_route
        .or(user_route)
        .or(admin_route)
        .recover(handle_rejection)
}
