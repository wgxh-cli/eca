pub mod create;
pub mod delete;
pub mod login;

use crate::models::users::*;
use crate::apis::res::*;
use warp::*;

pub fn user_router() -> filters::BoxedFilter<(Json, )> {
  let shared_users = SharedUsers::default();
  let root = path!("users").map(|| Json::succ("users"));

  root
    .or(create::create(shared_users.clone()))
    .unify()
    .or(delete::delete(shared_users.clone()))
    .unify()
    .or(login::login(shared_users))
    .unify()
    .boxed()
}

