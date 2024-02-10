use crate::{adapter::routers, adapter::routers::v1, infra::docs::swagger::ApiDoc};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes_config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(routers::index_get)
    .service(v1::auth::controller::sign_in)
    .service(v1::auth::controller::register)
    .service(v1::auth::controller::change_password)
    .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()))
    .service(actix_files::Files::new("/coverage", "./coverage/html").index_file("index.html"))
    .service(actix_files::Files::new("/docs", "./docs").index_file("index.html"))
    .service(actix_files::Files::new("/logs", "./logs").show_files_listing())
    .default_service(web::to(routers::not_found));
}
