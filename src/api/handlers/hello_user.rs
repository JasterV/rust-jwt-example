use warp::Reply;
use crate::utils::WebResult;

pub async fn hello_user(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello User {}", uid))
}
