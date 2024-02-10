use crate::{adapter::services::jwt::JWTService, application::services::Services};
use actix_web::web;

pub fn services_config(cfg: &mut web::ServiceConfig) {
  cfg.app_data(web::Data::new(get_services()));
}

pub fn get_services<'a>() -> Services<JWTService<'a>> {
  Services {
    token: get_jwt_service(),
  }
}

pub fn get_jwt_service<'a>() -> JWTService<'a> {
  JWTService {
    iss: env!("CARGO_PKG_NAME").to_string(),
    key: b"JWT_SECRET",
  }
}
