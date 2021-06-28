use crate::constants::BEARER;
use crate::db::schemas::Role;
use crate::error::Error;
use crate::services::JwtService;
use crate::utils::{Result, WebResult};
use warp::{
    filters::header::headers_cloned,
    http::header::{HeaderMap, HeaderValue, AUTHORIZATION},
    reject, Filter, Rejection,
};

pub fn with_auth(
    jwt_service: JwtService,
    role: Role,
) -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    headers_cloned()
        .and(warp::any().map(move || jwt_service.clone()))
        .and(warp::any().map(move || role.clone()))
        .and_then(authorize)
}

async fn authorize(
    headers: HeaderMap<HeaderValue>,
    jwt_service: JwtService,
    role: Role,
) -> WebResult<String> {
    match jwt_from_header(&headers) {
        Ok(jwt) => {
            let decoded = jwt_service.validate(jwt)?;
            if role == Role::Admin && Role::from_str(&decoded.claims.role) != Role::Admin {
                return Err(reject::custom(Error::NoPermissionError));
            }
            Ok(decoded.claims.sub)
        }
        Err(e) => return Err(reject::custom(e)),
    }
}

fn jwt_from_header(headers: &HeaderMap<HeaderValue>) -> Result<String> {
    let header = match headers.get(AUTHORIZATION) {
        Some(v) => v,
        None => return Err(Error::NoAuthHeaderError),
    };
    let auth_header = match std::str::from_utf8(header.as_bytes()) {
        Ok(v) => v,
        Err(_) => return Err(Error::NoAuthHeaderError),
    };
    if !auth_header.starts_with(BEARER) {
        return Err(Error::InvalidAuthHeaderError);
    }
    Ok(auth_header.trim_start_matches(BEARER).to_owned())
}
