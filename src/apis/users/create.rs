use warp::*;
use crate::models::users::*;
use crate::apis::res::*;

pub fn create(users: SharedUsers) -> filters::BoxedFilter<(Json, )> {
  post()
    .and(path!("users" / "create"))
    .and(with_users(users))
    .and(body::json::<CreateUser>())
    .map(create_user)
    .map(to_json)
    .boxed()
}
