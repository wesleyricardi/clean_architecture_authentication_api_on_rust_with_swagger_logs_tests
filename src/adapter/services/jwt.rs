use crate::{
  application::services::security::token_service::{Token, TokenService},
  domain::error::AppError,
};
use jsonwebtoken::{get_current_timestamp, DecodingKey, EncodingKey, Header, Validation};

pub struct JWTService<'a> {
  pub iss: String,
  pub key: &'a [u8],
}

impl TokenService for JWTService<'_> {
  fn encode(&self, sub: String, aud: String, exp_min: u64) -> Result<String, AppError> {
    let user_token = Token {
      sub,
      iss: self.iss.clone(),
      aud,
      iat: get_current_timestamp() as usize,
      exp: (get_current_timestamp() + 1000 * 60 * exp_min) as usize,
    };

    match jsonwebtoken::encode(
      &Header::default(),
      &user_token,
      &EncodingKey::from_secret(self.key),
    ) {
      Ok(token) => Ok(token),
      Err(error) => Err(AppError::internal(format!(
        "failed to encode token :{}",
        error
      ))),
    }
  }

  fn decode(&self, token: &str) -> Result<Token, AppError> {
    match jsonwebtoken::decode::<Token>(
      token,
      &DecodingKey::from_secret(self.key),
      &Validation::default(),
    ) {
      Ok(token) => Ok(token.claims),
      Err(_) => Err(AppError::unauthenticated("expired or invalid JWT token")),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  const ISS: &str = "my_app";
  const KEY: &[u8] = b"token_key";
  const SUB: &str = "some_id";
  const AUD: &str = "service_name";
  const EXP_MIN: u64 = 120;

  #[test]
  fn encode() {
    let jwt_service = JWTService {
      iss: ISS.to_owned(),
      key: KEY,
    };

    let token = jwt_service
      .encode(SUB.to_owned(), AUD.to_owned(), EXP_MIN)
      .expect("encode failed");

    let token_decoded = jsonwebtoken::decode::<Token>(
      &token,
      &DecodingKey::from_secret(KEY),
      &Validation::default(),
    )
    .expect("decode failed")
    .claims;

    assert_eq!(token_decoded.sub, SUB);
    assert_eq!(token_decoded.aud, AUD);
    assert!(token_decoded.exp > (get_current_timestamp() + 1000 * 60 * (EXP_MIN - 1)) as usize);
  }

  #[test]
  fn decode() {
    let user_token = Token {
      sub: SUB.to_owned(),
      iss: ISS.to_owned(),
      aud: AUD.to_owned(),
      iat: get_current_timestamp() as usize,
      exp: (get_current_timestamp() + 1000 * 60 * EXP_MIN) as usize,
    };

    let token = jsonwebtoken::encode(
      &Header::default(),
      &user_token,
      &EncodingKey::from_secret(KEY),
    )
    .expect("fail to build an encode token to test the encode function");

    let jwt_service = JWTService {
      iss: ISS.to_owned(),
      key: KEY,
    };

    let token_decoded = jwt_service.decode(&token).expect("decode failed");

    assert_eq!(token_decoded.sub, SUB);
    assert_eq!(token_decoded.aud, AUD);
    assert!(token_decoded.exp > (get_current_timestamp() + 1000 * 60 * (EXP_MIN - 1)) as usize);
  }

  #[test]
  fn decode_expired_token() {
    let user_token = Token {
      sub: SUB.to_owned(),
      iss: ISS.to_owned(),
      aud: AUD.to_owned(),
      iat: get_current_timestamp() as usize,
      exp: (get_current_timestamp() - 1000 * 60 * EXP_MIN) as usize,
    };

    let token = jsonwebtoken::encode(
      &Header::default(),
      &user_token,
      &EncodingKey::from_secret(KEY),
    )
    .expect("fail to build an encode token to test the encode function");

    let jwt_service = JWTService {
      iss: ISS.to_owned(),
      key: KEY,
    };

    match jwt_service.decode(&token) {
      Ok(_) => panic!("should not be success"),
      Err(error) => assert_eq!(
        error,
        AppError::unauthenticated("expired or invalid JWT token")
      ),
    }
  }
}
