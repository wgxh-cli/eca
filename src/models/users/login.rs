use serde::{Serialize, Deserialize};
use crate::models::users::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoginUser(pub String);

pub fn login_user(users: SharedUsers, login_user: LoginUser) -> Result<User, String> {
  let writer = users.write().unwrap();
  
  writer.iter()
    .find(|user| login_user.0 == user.uuid())
    .cloned()
    .ok_or_else(|| "Failed to find user".to_string())
}

