use crate::{
  adapter::utilities::{bcrypt::BCrypt, id_generator::NewID},
  domain::utilities::Utilities,
};
use actix_web::web;

pub fn utilities_config(cfg: &mut web::ServiceConfig) {
  cfg.app_data(web::Data::new(Utilities {
    crypto: BCrypt { cost: 8 },
    id_generator: NewID,
  }));
}
