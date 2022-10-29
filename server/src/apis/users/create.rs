use warp::*;
use crate::models::*;
use crate::models::users::*;
use crate::models::utils::*;
use crate::apis::res::*;

pub fn create(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  path!("users" / "create")
    .and(post())
    .and(with_shared(shared_db))
    .and(body::json::<CreateUser>())
    .map(create_user)
    .map(to_json)
    .boxed()
}
