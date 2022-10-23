use serde::{Serialize, Deserialize};
use crate::models::users::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeleteUser(pub UserUUID);

pub fn delete_user(users: SharedUsers, delete_user_dto: DeleteUser) -> Result<(), String> {
  let mut writer = users.write().unwrap();

  writer.iter()
    .position(|user| user.uuid() == delete_user_dto.0)
    .map(|pos| writer.remove(pos))
    .map(|_| ())
    .ok_or_else(|| "Failed to delete user".to_string())
}
