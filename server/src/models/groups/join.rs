use crate::models::*;
use crate::models::users::*;
use crate::models::groups::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct JoinGroup {
  pub user_id: UserUUID,
  pub group_id: GroupUUID,
}

