use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::models::users::*;

pub type MessageUUID = (UserUUID, String);
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Message {
  pub content: String,
  pub created_time: SystemTime,
  pub user_id: UserUUID,
}
impl Message {
  pub fn uuid(&self) -> MessageUUID {
    return (self.user_id.clone(), self.created_time.)
  }
}
