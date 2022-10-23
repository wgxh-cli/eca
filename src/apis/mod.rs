pub mod res;
pub mod users;

use warp::*;
use res::*;

pub fn index() -> filters::BoxedFilter<(Json, )> {
  let root = path!("a").map(|| Json::succ("fuck you")).boxed();

  root
    .or(users::user_router())
    .unify()
    .boxed()
}
