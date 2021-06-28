use warp::Reply;
use crate::utils::WebResult;

pub async fn hello_admin(uid: String) -> WebResult<impl Reply> {
    Ok(format!("Hello Admin {}", uid))
}
