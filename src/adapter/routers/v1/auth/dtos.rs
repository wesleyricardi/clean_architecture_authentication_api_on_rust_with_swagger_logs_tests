use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::use_cases::{authenticate, authenticate::UserAuthenticationResponse};

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct UserSignInRequest {
  #[schema(example = "john.doe")]
  pub username: Option<String>,
  #[schema(example = "johndoe@company.com")]
  pub email: Option<String>,
  #[schema(example = "+1 151 999-9999")]
  pub telephone: Option<String>,
  #[schema(example = "12345678")]
  pub password: String,
}

impl<'a> From<&'a UserSignInRequest> for authenticate::UserSignInRequest<'a> {
  fn from(data: &'a UserSignInRequest) -> Self {
    authenticate::UserSignInRequest {
      username: data.username.as_deref(),
      email: data.email.as_deref(),
      telephone: data.telephone.as_deref(),
      password: &data.password,
    }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct UserRegistrationRequest {
  #[schema(example = "John Doe")]
  pub name: String,
  #[schema(example = "john.doe")]
  pub username: String,
  #[schema(example = "1999-12-31")]
  pub birth_date: String,
  #[schema(example = 1)]
  pub gender_id: i32,
  #[schema(example = "12345678")]
  pub password: String,
  #[schema(example = "12345678")]
  pub password_repetition: String,
  #[schema(example = "153 W 57th St")]
  pub address_street: String,
  #[schema(example = "manhattan")]
  pub address_neighborhood: String,
  #[schema(example = 4)]
  pub address_city_id: i32,
  #[schema(example = 10019)]
  pub address_postal_code: i32,
  #[schema(example = "johndoe@company.com")]
  pub email: String,
  #[schema(example = "+1 151 999-9999")]
  pub telephone: Option<String>,
}

impl<'a> From<&'a UserRegistrationRequest> for authenticate::UserRegistrationRequest<'a> {
  fn from(data: &'a UserRegistrationRequest) -> Self {
    authenticate::UserRegistrationRequest {
      name: &data.name,
      username: &data.username,
      birth_date: &data.birth_date,
      gender_id: data.gender_id,
      password: &data.password,
      password_repetition: &data.password_repetition,
      address_street: &data.address_street,
      address_neighborhood: &data.address_neighborhood,
      address_city_id: data.address_city_id,
      address_postal_code: data.address_postal_code,
      email: &data.email,
      telephone: data.telephone.as_deref(),
    }
  }
}

#[derive(Deserialize, ToSchema)]
pub struct ChangeUserPasswordRequest {
  #[schema(example = "77285939-bdaa-4c5f-9805-3873fca3396e")]
  pub profile_id: String,
  #[schema(example = "12345678")]
  pub old_password: String,
  #[schema(example = "87654321")]
  pub password: String,
  #[schema(example = "87654321")]
  pub password_repetition: String,
}

impl<'a> From<&'a ChangeUserPasswordRequest> for authenticate::ChangeUserPasswordRequest<'a> {
  fn from(value: &'a ChangeUserPasswordRequest) -> Self {
    authenticate::ChangeUserPasswordRequest {
      profile_id: &value.profile_id,
      old_password: &value.old_password,
      password: &value.password,
      password_repetition: &value.password_repetition,
    }
  }
}

#[derive(Serialize, ToSchema, Clone, Debug, Deserialize)]
pub struct UserAuthenticationResponseHttp {
  #[schema(example = 1)]
  pub id: String,
  #[schema(example = "john.doe")]
  pub username: String,
  #[schema(example = "johndoe@company.com")]
  pub name: String,
  #[schema(
    example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
  )]
  pub token: String,
}

impl From<UserAuthenticationResponse> for UserAuthenticationResponseHttp {
  fn from(value: UserAuthenticationResponse) -> Self {
    UserAuthenticationResponseHttp {
      id: value.id,
      username: value.username.to_string(),
      name: value.name.to_string(),
      token: value.token,
    }
  }
}
