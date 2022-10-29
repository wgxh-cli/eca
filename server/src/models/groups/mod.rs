pub mod create;
pub mod delete;
pub mod join;
pub mod quit;

pub use create::*;
pub use delete::*;
pub use join::*;
pub use create::*;

use crate::models::users::UserUUID;
use serde::{Serialize, Deserialize};
use crypto::{md5::Md5, digest::Digest};
use std::sync::{RwLock, Arc};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum GroupMemberRoles {
  Owner,
  Admin,
  Member,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GroupMember {
  pub user_id: UserUUID,
  pub role: GroupMemberRoles,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Group {
  pub name: String,
  pub users: Vec<GroupMember>,
  pub index: usize,
}
pub type GroupUUID = String;
impl Group {
  pub fn uuid(&self) -> GroupUUID {
    let mut digest = Md5::new();
    digest.input_str(&(self.index.to_string() + &self.name));

    digest.result_str()
  }
}

pub type Groups = Vec<Group>;
pub type SharedGroups = Arc<RwLock<Groups>>;
