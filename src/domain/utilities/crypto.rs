use mockall::automock;

use crate::domain::error::AppError;

#[automock]
pub trait Crypto: Sync + Send {
  fn hash_password(&self, password: &str) -> Result<String, AppError>;
  fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AppError>;
}
