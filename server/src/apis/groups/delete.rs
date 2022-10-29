use crate::apis::res::*;
use crate::models::*;
use crate::models::groups::*;
use crate::models::utils::*;
use warp::*;

pub fn delete(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  path!("groups" / "delete")
    .and(with_shared(shared_db))
    .and(body::json::<DeleteGroup>())
    .map(delete_group)
    .map(to_json)
    .boxed()
}
