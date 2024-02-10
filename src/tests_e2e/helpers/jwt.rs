use jsonwebtoken::get_current_timestamp;

use crate::{
  application::services::security::token_service::TokenService,
  infra::config::services::get_jwt_service,
};

pub enum TokenExpirationExpect {
  GreatThan(u64),
}

pub fn assert_jwt(token: &str, sub: &str, aud: &str, expiration: TokenExpirationExpect) {
  let token_decoded = get_jwt_service().decode(token).unwrap();

  assert_eq!(token_decoded.sub, sub);
  assert_eq!(token_decoded.aud, aud);

  match expiration {
    TokenExpirationExpect::GreatThan(time) => {
      assert!(token_decoded.exp > (get_current_timestamp() + time) as usize)
    }
  }
}
