use actix_cors::Cors;
use actix_web::http;

pub fn default_cors() -> Cors {
  Cors::default()
    .allowed_origin("http://localhost:8080")
    .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b"localhost:8080"))
    .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    .allowed_header(http::header::CONTENT_TYPE)
    .max_age(3600)
}
