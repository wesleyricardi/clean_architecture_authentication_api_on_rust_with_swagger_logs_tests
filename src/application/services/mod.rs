use self::security::token_service::TokenService;
pub mod security;
pub struct Services<Token: TokenService> {
  pub token: Token,
}
