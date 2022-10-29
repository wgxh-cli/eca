use crate::apis::*;
use crate::models::*;
use crate::models::groups::*;
use crate::models::utils::*;
use warp::*;

pub fn create(shared_db: SharedDB) -> filters::BoxedFilter<(Json, )> {
  path!("groups" / "create")
    .and(with_shared(shared_db))
    .and(body::json::<CreateGroup>())
    .map(create_group)
    .map(to_json)
    .boxed()
}
