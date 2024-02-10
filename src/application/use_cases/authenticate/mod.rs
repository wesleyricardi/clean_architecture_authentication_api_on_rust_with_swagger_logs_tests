#[cfg(test)]
pub(self) mod test_utils;
use serde::Serialize;
#[cfg(test)]
use test_utils::*;

pub mod user;

use crate::domain::{
  core::user::{
    change_password, sign_in,
    sign_up::{self, NotId},
  },
  entities::user::UserColumns,
  error::AppError,
  types::Password,
};
use async_trait::async_trait;
use chrono::NaiveDate;
use mockall::automock;

use validator::Validate;

#[async_trait]
#[automock]
pub trait UserAuthentication: Send + Sync {
  async fn sign_in(
    &self,
    request: &UserSignInRequest<'_>,
  ) -> Result<UserAuthenticationResponse, AppError>;
  async fn register(
    &self,
    request: &UserRegistrationRequest<'_>,
  ) -> Result<UserAuthenticationResponse, AppError>;
  async fn update_password(
    &self,
    request: &ChangeUserPasswordRequest<'_>,
    token: &str,
  ) -> Result<(), AppError>;
}

#[derive(Validate, Default)]
pub struct UserSignInRequest<'a> {
  #[validate(length(
    min = 5,
    message = "Please provide a valid username, minimum 5 characters!"
  ))]
  pub username: Option<&'a str>,
  #[validate(email)]
  pub email: Option<&'a str>,
  #[validate(length(min = 7, message = "Please provide a valid telephone number!"))]
  pub telephone: Option<&'a str>,
  #[validate(length(min = 8, message = "Password must contain minimum 8 characters!"))]
  pub password: &'a str,
}

impl<'a> TryFrom<&'a UserSignInRequest<'a>> for sign_in::Request<'a> {
  type Error = AppError;

  fn try_from(value: &'a UserSignInRequest) -> Result<Self, Self::Error> {
    let column = value
      .username
      .map(UserColumns::Username)
      .or_else(|| value.email.map(UserColumns::Email))
      .or_else(|| value.telephone.map(UserColumns::Telephone))
      .ok_or_else(|| AppError::invalid_argument("no username, email ou telephone provide"))?;

    let req = sign_in::Request {
      column,
      password: value.password,
    };

    Ok(req)
  }
}

#[derive(Validate, Default, Serialize)]
pub struct UserRegistrationRequest<'a> {
  #[validate(length(
    min = 5,
    message = "Please provide a valid name, minimum 5 characters!!"
  ))]
  pub name: &'a str,
  #[validate(length(
    min = 5,
    message = "Please provide a valid username, minimum 5 characters!"
  ))]
  pub username: &'a str,
  pub birth_date: &'a str,
  #[validate(range(min = 1))]
  pub gender_id: i32,
  #[validate(length(min = 8, message = "Password must contain minimum 8 characters!"))]
  pub password: &'a str,
  #[validate(must_match = "password")]
  pub password_repetition: &'a str,
  #[validate(length(
    min = 3,
    message = "Please provide a valid street, minimum 3 characters!!"
  ))]
  pub address_street: &'a str,
  #[validate(length(
    min = 3,
    message = "Please provide a valid neighborhood, minimum 3 characters!!"
  ))]
  pub address_neighborhood: &'a str,
  #[validate(range(min = 1))]
  pub address_city_id: i32,
  #[validate(range(min = 1))]
  pub address_postal_code: i32,
  #[validate(email)]
  pub email: &'a str,
  #[validate(phone)]
  pub telephone: Option<&'a str>,
}

impl<'a> TryFrom<&'a UserRegistrationRequest<'a>> for sign_up::Request<'a, NotId, Password> {
  type Error = AppError;

  fn try_from(value: &'a UserRegistrationRequest) -> Result<Self, Self::Error> {
    let birth_date = NaiveDate::parse_from_str(value.birth_date, "%Y-%m-%d")
      .map_err(|_| AppError::invalid_argument("birth date format invalid"))?;

    let req = sign_up::Request {
      name: value.name,
      username: value.username,
      birth_date,
      gender_id: value.gender_id,
      password: Password(value.password.to_string()),
      street: value.address_street,
      neighborhood: value.address_neighborhood,
      city_id: value.address_city_id,
      postal_code: value.address_postal_code,
      email_address: value.email,
      telephone_number: value.telephone,
      ..Default::default()
    };

    Ok(req)
  }
}

#[derive(Debug, PartialEq)]
pub struct UserAuthenticationResponse {
  pub id: String,
  pub name: String,
  pub username: String,
  pub token: String,
}

#[derive(Validate)]
pub struct ChangeUserPasswordRequest<'a> {
  #[validate(length(
    min = 5,
    message = "Please provide a valid username, minimum 5 characters!"
  ))]
  pub profile_id: &'a str,
  #[validate(length(min = 8, message = "Password must contain minimum 8 characters!"))]
  pub old_password: &'a str,
  #[validate(length(min = 8, message = "Password must contain minimum 8 characters!"))]
  pub password: &'a str,
  #[validate(must_match = "password")]
  pub password_repetition: &'a str,
}

impl<'a> From<&'a ChangeUserPasswordRequest<'a>> for change_password::Request<'a> {
  fn from(value: &'a ChangeUserPasswordRequest) -> Self {
    change_password::Request {
      profile_id: value.profile_id,
      password: value.password,
      old_password: value.old_password,
    }
  }
}
