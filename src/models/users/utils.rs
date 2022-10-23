use crate::models::users::*;
use warp::*;

pub fn with_users(shared_users: SharedUsers) -> filters::BoxedFilter<(SharedUsers, )> {
  any().map(move || shared_users.clone()).boxed()
}

