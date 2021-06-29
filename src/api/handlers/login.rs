use crate::api::LoginRequest;
use crate::db::DB;
use crate::models::user::Credentials;
use crate::services::{jwt_service::JwtService, login_service::login};
use crate::utils::WebResult;
use warp::{reject, reply, Reply};

pub async fn login_handler(
    db: DB,
    jwt_service: JwtService,
    body: LoginRequest,
) -> WebResult<impl Reply> {
    let result = login(
        &db,
        &jwt_service,
        &Credentials {
            email: body.email,
            pwd: body.pw,
        },
    )
    .map_err(|err| reject::custom(err))?;
    Ok(reply::json(&result))
}
