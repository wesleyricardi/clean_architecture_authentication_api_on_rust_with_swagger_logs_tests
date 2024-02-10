use crate::domain::error::AppError;

use mockall::automock;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
  pub sub: String,
  pub iss: String,
  pub aud: String,
  pub iat: usize,
  pub exp: usize,
}

#[automock]
pub trait TokenService: Sync + Send {
  fn encode(&self, sub: String, aud: String, exp_min: u64) -> Result<String, AppError>;
  fn decode(&self, token: &str) -> Result<Token, AppError>;
}
