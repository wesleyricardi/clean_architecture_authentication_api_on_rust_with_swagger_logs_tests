use crate::domain::{error::AppError, utilities::crypto::Crypto};

pub struct BCrypt {
  pub cost: u32,
}

impl Crypto for BCrypt {
  fn hash_password(&self, password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, self.cost).map_err(|_| AppError::internal("hash password failed"))
  }
  fn verify_password(&self, hash: &str, password: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash).map_err(|err| AppError::internal(err.to_string()))
  }
}
