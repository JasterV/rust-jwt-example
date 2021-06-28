extern crate tokio;

mod api;
mod config;
mod constants;
mod db;
mod error;
mod models;
mod services;
mod utils;

use config::Config;
use dotenv::dotenv;
use services::AppServices;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = envy::from_env::<Config>().unwrap();
    let db = db::DB::new();
    let app_services = AppServices::new(config, db);
    let routes = api::routes(app_services);
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
