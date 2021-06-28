use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub jwt_secret: String,
}