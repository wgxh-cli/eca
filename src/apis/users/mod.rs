pub mod create;

use crate::models::users::*;
use crate::apis::res::*;
use warp::*;

pub fn user_router() -> filters::BoxedFilter<(Json, )> {
  let shared_users = SharedUsers::default();
  let root = path("users").map(|| Json::succ(""));

  root
    .or(create::create(shared_users))
    .unify()
    .boxed()
}

