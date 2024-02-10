use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/")]
pub async fn index_get() -> HttpResponse {
  let data = json!({
      "name":  env!("CARGO_PKG_NAME"),
      "version": env!("CARGO_PKG_VERSION"),
      "documentation": "https://localhost:3000/docs",
      "about": env!("CARGO_PKG_DESCRIPTION")
  });

  HttpResponse::Ok().json(data)
}
