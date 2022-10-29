pub mod create;
pub mod delete;
pub mod login;

use crate::models::*;
use crate::apis::res::*;
use warp::*;

pub fn user_router(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  let root = path!("users").map(|| Json::succ("users"));

  root
    .or(create::create(shared_db.clone()))
    .unify()
    .or(delete::delete(shared_db.clone()))
    .unify()
    .or(login::login(shared_db))
    .unify()
    .boxed()
}

