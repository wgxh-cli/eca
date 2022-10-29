use warp::*;
use crate::apis::res::*;
use crate::models::*;
use crate::models::users::*;
use crate::models::utils::*;

pub fn login(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  path!("users" / "login")
    .and(post())
    .and(with_shared(shared_db))
    .and(body::json::<LoginUser>())
    .map(login_user)
    .map(to_json)
    .boxed()
}
