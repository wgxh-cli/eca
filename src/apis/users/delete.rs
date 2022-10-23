use warp::*;
use crate::models::users::*;
use crate::apis::res::*;

pub fn delete(shared_users: SharedUsers) -> filters::BoxedFilter<(Json, )> {
  path!("users" / "delete")
    .and(post())
    .and(with_users(shared_users))
    .and(body::json::<DeleteUser>())
    .map(delete_user)
    .map(to_json)
    .boxed()
}
