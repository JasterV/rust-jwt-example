use std::convert::Infallible;
use warp::Filter;
use crate::services::jwt_service::JwtService;

pub fn with_jwt(jwt_service: JwtService) -> impl Filter<Extract = (JwtService,), Error = Infallible> + Clone {
    warp::any().map(move || jwt_service.clone())
}
