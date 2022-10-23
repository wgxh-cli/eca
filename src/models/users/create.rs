use crate::models::users::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CreateUser {
  pub name: String,
  pub pawd: String,
  pub email: String,
}
impl From<CreateUser> for User {
  fn from(CreateUser {
    name,
    pawd,
    email,
  }: CreateUser) -> Self {
    User {
      name,
      pawd,
      email,
    }
  }
}

pub fn create_user(users: SharedUsers, create_user_dto: CreateUser) -> Result<UserUUID, String> {
  let mut writer = users.write().unwrap();
  println!("THIS IS A TEST LINE");

  Some(User::from(create_user_dto))
    .filter(|user| writer.iter().all(|auser| auser.email != user.email))
    .inspect(move |user| writer.push(user.clone()))
    .map(|user| user.uuid())
    .ok_or_else(|| "Failed to create user: duplicated email address".to_string())
}
