use crate::db::DB;
use std::convert::Infallible;
use warp::Filter;

pub fn with_db(db: DB) -> impl Filter<Extract = (DB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}
