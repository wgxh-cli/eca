use crate::models::*;
use crate::models::groups::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteGroup(GroupUUID);

pub fn delete_group(shared_db: SharedDB, delete_group: DeleteGroup) -> Result<(), String> {
  let mut writer = shared_db.write().unwrap();

  writer.groups.iter()
    .position(|group| group.uuid() == delete_group.0)
    .map(|index| { writer.groups.remove(index); })
    .ok_or_else(|| "Failed to find specified group".to_string())
}
