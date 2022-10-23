pub mod utils;
pub mod create;

pub use utils::*;
pub use create::*;

use serde::{Serialize, Deserialize};
use std::sync::{
  RwLock,
  Arc,
};
use crypto::{
  digest::Digest,
  md5::Md5
};

pub type UUID = String;
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct User {
  pub name: String,
  pub pawd: String,
  pub email: String,
}
impl User {
  pub fn uuid(&self) -> UUID {
    let mut digest = Md5::new();
    digest.input_str(&(self.email.clone() + &self.pawd));
    digest.result_str()
  }
}
pub type Users = Vec<User>;
pub type SharedUsers = Arc<RwLock<Users>>;
