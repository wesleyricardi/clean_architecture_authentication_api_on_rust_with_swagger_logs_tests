use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
  entities::user::{UserColumns, UserData},
  error::AppError,
};

use super::sign_up::{self, encrypter::PasswordEncrypted};

#[automock]
#[async_trait]
pub trait UserRepository: Sync + Send {
  async fn find_user_by<'a>(&self, column: &UserColumns<'a>) -> Result<UserData, AppError>;
  async fn store<'a>(
    &self,
    user_data: &sign_up::Request<'a, String, PasswordEncrypted>,
  ) -> Result<(), AppError>;
  async fn update_password(&self, password: &str, profile_id: &str) -> Result<(), AppError>;
}
