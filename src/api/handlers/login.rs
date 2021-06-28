use crate::api::LoginRequest;
use crate::services::LoginService;
use crate::utils::WebResult;
use warp::{reject, reply, Reply};

pub async fn login_handler(
    login_service: LoginService,
    body: LoginRequest,
) -> WebResult<impl Reply> {
    let result = login_service
        .login(body.email, body.pw)
        .map_err(|err| reject::custom(err))?;
    Ok(reply::json(&result))
}
