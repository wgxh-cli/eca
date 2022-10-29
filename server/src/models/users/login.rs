use serde::{Serialize, Deserialize};
use crate::models::*;
use crate::models::users::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct LoginUser(pub String);

pub fn login_user(shared_db: SharedDB, login_user: LoginUser) -> Result<User, String> {
  let writer = shared_db.write().unwrap();
  
  writer.users.iter()
    .find(|user| login_user.0 == user.uuid())
    .cloned()
    .ok_or_else(|| "Failed to find user".to_string())
}

