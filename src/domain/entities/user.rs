#[derive(Debug, PartialEq, Clone)]
pub enum UserColumns<'a> {
  Id(&'a str),
  Username(&'a str),
  Email(&'a str),
  Telephone(&'a str),
}

impl Default for UserColumns<'_> {
  fn default() -> Self {
    UserColumns::Id("")
  }
}

#[derive(Default, PartialEq, Debug)]
pub struct UserData {
  pub id: String,
  pub name: String,
  pub username: String,
  pub password: String,
}

#[derive(Debug, PartialEq)]
pub struct PublicUserData {
  pub id: String,
  pub username: String,
  pub name: String,
}
