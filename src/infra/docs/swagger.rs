use crate::adapter::routers::{v1, v1::auth};
use utoipa::OpenApi;
use utoipa::{
  openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
  Modify,
};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
  fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
    let components = openapi
      .components
      .as_mut()
      .expect("not have components defined in macro utoipa");
    components.add_security_scheme(
      "api_jwt_token",
      SecurityScheme::Http(
        HttpBuilder::new()
          .scheme(HttpAuthScheme::Bearer)
          .bearer_format("JWT")
          .description(Some("Set the JWT Token"))
          .build(),
      ),
    )
  }
}

#[derive(OpenApi)]
#[openapi(
  info(
    title = "Rust API Skeleton",
    contact(
      name = "Wesley Ricardi",
      email = "wesley.ricardi@outlook.com"
    )
  ),
  paths(
    v1::auth::controller::sign_in,
    v1::auth::controller::register,
    v1::auth::controller::change_password,
  ),
  components(
    schemas(
      auth::dtos::UserSignInRequest,
      auth::dtos::UserRegistrationRequest,
      auth::dtos::ChangeUserPasswordRequest,
      auth::dtos::UserAuthenticationResponseHttp,
    )
  ),
  tags(
    (name = "Rust API Skeleton", description = "this api is a example of a Rust ApiRest")
  ),
  modifiers(&SecurityAddon),
)]
pub struct ApiDoc;
