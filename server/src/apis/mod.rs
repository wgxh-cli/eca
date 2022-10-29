pub mod res;
pub mod users;
pub mod groups;

use warp::*;
use res::*;
use crate::models::*;

pub fn index() -> filters::BoxedFilter<(Json, )> {
  let shared_db = SharedDB::default();
  let root = path!("a").map(|| Json::succ("fuck you")).boxed();

  root
    .or(users::user_router(shared_db.clone()))
    .unify()
    .or(groups::group_router(shared_db))
    .unify()
    .boxed()
}
