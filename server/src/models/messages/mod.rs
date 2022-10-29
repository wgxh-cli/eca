use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::models::users::*;
use crypto::{md5::Md5, digest::Digest};

pub type MessageUUID = (UserUUID, String);
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Message {
  pub content: String,
  pub created_time: String,
  pub user_id: UserUUID,
}
impl Message {
  pub fn new(content: String, user_id: UserUUID) -> Self {
    Message {
      content,
      user_id,
      created_time: Local::now().to_string(),
    }
  }

  pub fn uuid(&self) -> MessageUUID {
    let mut digest = Md5::new();
    digest.input_str(&(self.user_id.clone() + &self.content));

    (self.user_id.clone(), digest.result_str())
  }
}
