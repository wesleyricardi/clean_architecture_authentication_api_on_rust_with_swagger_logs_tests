use super::config::{
  cors::default_cors, routes::routes_config, services::services_config, utilities::utilities_config,
};
use crate::AppState;
use actix_web::{web, App, HttpServer};

pub async fn start_http_server(state: AppState) -> Result<(), std::io::Error> {
  let app_state = web::Data::new(state);

  HttpServer::new(move || {
    App::new()
      .wrap(default_cors())
      .app_data(app_state.clone())
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
}
