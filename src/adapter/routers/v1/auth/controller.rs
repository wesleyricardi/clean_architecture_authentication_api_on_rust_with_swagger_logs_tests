use crate::{
  adapter::{
    repositories::user::UserRepositoryDB,
    routers::helpers::actix_bearer_token::extract_bearer_token,
    services::jwt::JWTService,
    utilities::{bcrypt::BCrypt, id_generator::NewID},
  },
  application::{
    services::Services,
    use_cases::authenticate::{user::UserUseCase, UserAuthentication},
  },
  domain::utilities::Utilities,
  AppState,
};
use actix_web::{post, put, web, HttpRequest, HttpResponse};

use super::dtos::{
  ChangeUserPasswordRequest, UserAuthenticationResponseHttp, UserRegistrationRequest,
  UserSignInRequest,
};

#[utoipa::path(
  request_body = UserSignInRequest,
  responses(
      (status = 200, description = "Sign in to an existing user by providing a valid password", body = UserAuthenticationResponseHttp),
      (status = 400, description = "Sign in with invalid data"),
      (status = 401, description = "Sign in with invalid password"),
      (status = 404, description = "No user as found with provide username, email or telephone"),
      (status = 500, description = "Internal server error")
  )
)]
#[post("/v1/auth/sign_in")]
pub async fn sign_in(
  app_state: web::Data<AppState>,
  services: web::Data<Services<JWTService<'_>>>,
  utilities: web::Data<Utilities<BCrypt, NewID>>,
  request: web::Json<UserSignInRequest>,
) -> HttpResponse {
  let use_case = UserUseCase {
    user_repository: &UserRepositoryDB {
      pool: &app_state.postgres_pool,
    },
    services: &services,
    utilities: &utilities,
  };

  match use_case.sign_in(&(&request.0).into()).await {
    Ok(user) => {
      let response: UserAuthenticationResponseHttp = user.into();
      HttpResponse::Ok().json(response)
    }
    Err(error) => error.into(),
  }
}

#[utoipa::path(
  request_body = UserRegistrationRequest,
  responses(
      (status = 201, description = "Register with valid data", body = UserAuthenticationResponseHttp),
      (status = 400, description = "Register with invalid data"),
      (status = 409, description = "Already existing user"),
      (status = 500, description = "Internal server error")
  )
)]
#[post("/v1/auth/register")]
pub async fn register(
  app_state: web::Data<AppState>,
  services: web::Data<Services<JWTService<'_>>>,
  utilities: web::Data<Utilities<BCrypt, NewID>>,
  request: web::Json<UserRegistrationRequest>,
) -> HttpResponse {
  let use_case = UserUseCase {
    user_repository: &UserRepositoryDB {
      pool: &app_state.postgres_pool,
    },
    services: &services,
    utilities: &utilities,
  };

  match use_case.register(&(&request.0).into()).await {
    Ok(user) => {
      let response: UserAuthenticationResponseHttp = user.into();
      HttpResponse::Created().json(response)
    }
    Err(error) => error.into(),
  }
}

#[utoipa::path(
  request_body = ChangeUserPasswordRequest,
  responses(
      (status = 200, description = "Password changed successfully"),
      (status = 400, description = "Received invalid data"),
      (status = 401, description = "Received JWT token invalid or expired"),
      (status = 403, description = "Received JWT token not have permission for the given user"),
      (status = 500, description = "Internal server error")
  ),
  security(
    ("api_jwt_token" = [])
  )
)]
#[put("/v1/auth/change_password")]
pub async fn change_password(
  req: HttpRequest,
  app_state: web::Data<AppState>,
  services: web::Data<Services<JWTService<'_>>>,
  utilities: web::Data<Utilities<BCrypt, NewID>>,
  request: web::Json<ChangeUserPasswordRequest>,
) -> HttpResponse {
  let token = if let Some(token) = extract_bearer_token(&req) {
    token
  } else {
    return HttpResponse::BadRequest().body("Invalid Bearer token");
  };

  let use_case = UserUseCase {
    user_repository: &UserRepositoryDB {
      pool: &app_state.postgres_pool,
    },
    services: &services,
    utilities: &utilities,
  };

  match use_case.update_password(&(&request.0).into(), token).await {
    Ok(_) => HttpResponse::Ok().body("password changed successfully"),
    Err(error) => error.into(),
  }
}
