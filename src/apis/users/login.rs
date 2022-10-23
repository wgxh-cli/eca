use warp::*;
use crate::apis::res::*;
use crate::models::users::*;

pub fn login(users: SharedUsers) -> filters::BoxedFilter<(Json, )> {
  path!("users" / "login")
    .and(post())
    .and(with_users(users))
    .and(body::json::<LoginUser>())
    .map(login_user)
    .map(to_json)
    .boxed()
}
