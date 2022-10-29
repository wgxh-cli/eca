use warp::*;
use crate::models::*;
use crate::models::users::*;
use crate::models::utils::*;
use crate::apis::res::*;

pub fn delete(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  path!("users" / "delete")
    .and(post())
    .and(with_shared(shared_db))
    .and(body::json::<DeleteUser>())
    .map(delete_user)
    .map(to_json)
    .boxed()
}
