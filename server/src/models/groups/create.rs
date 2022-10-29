use crate::models::*;
use crate::models::groups::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CreateGroup {
  pub name: String,
  pub users: Vec<GroupMember>,
}

pub fn create_group(shared_db: SharedDB, create_group: CreateGroup) -> Result<GroupUUID, String> {
  let mut writer = shared_db.write().unwrap();

  Ok(Group {
    name: create_group.name,
    users: create_group.users,
    index: writer.groups.len(),
  })
  .inspect(move |group| {
    writer.groups.push(group.clone())
  })
  .map(|group| group.uuid())
}

