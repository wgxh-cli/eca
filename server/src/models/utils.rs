use warp::*;
use std::marker::{Send, Sync};

pub fn with_shared<T: Send + Sync + Clone + 'static>(shared_object: T) -> filters::BoxedFilter<(T, )> {
  any().map(move || shared_object.clone()).boxed()
}
