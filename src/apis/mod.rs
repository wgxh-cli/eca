pub mod res;
pub mod users;

use warp::*;
use res::*;

pub fn index() -> filters::BoxedFilter<(Json, )> {
  path("")
    .map(|| Json::succ(""))
    .or(users::user_router())
    .unify()
    .boxed()
}
