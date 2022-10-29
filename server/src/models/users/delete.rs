use serde::{Serialize, Deserialize};
use crate::models::*;
use crate::models::users::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DeleteUser(pub UserUUID);

pub fn delete_user(shared_db: SharedDB, delete_user_dto: DeleteUser) -> Result<(), String> {
  let mut writer = shared_db.write().unwrap();

  writer.users.iter()
    .position(|user| user.uuid() == delete_user_dto.0)
    .map(|pos| writer.users.remove(pos))
    .map(|_| ())
    .ok_or_else(|| "Failed to delete user".to_string())
}
