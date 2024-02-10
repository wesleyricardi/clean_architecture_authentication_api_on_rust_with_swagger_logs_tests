use actix_web::{HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response<'a> {
  pub error: &'a str,
  pub description: &'a str,
}

pub async fn not_found() -> impl Responder {
  let data = Response {
    error: "not_found",
    description: "the requested route does not exist",
  };

  HttpResponse::NotFound().json(data)
}
