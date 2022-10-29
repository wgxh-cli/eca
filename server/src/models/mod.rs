pub mod users;
pub mod groups;
pub mod messages;
pub mod utils;

use users::*;
use groups::*;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug, PartialEq, Default)]
pub struct DB {
  pub users: Users,
  pub groups: Groups,
}
pub type SharedDB = Arc<RwLock<DB>>;
