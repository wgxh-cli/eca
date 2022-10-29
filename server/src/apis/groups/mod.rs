pub mod create;
pub mod delete;

use crate::apis::*;
use crate::models::*;
use warp::*;

pub fn group_router(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  let root = path!("groups").map(|| Json::succ(""));

  root
    .or(create::create(shared_db.clone()))
    .unify()
    .or(delete::delete(shared_db))
    .unify()
    .boxed()
}
