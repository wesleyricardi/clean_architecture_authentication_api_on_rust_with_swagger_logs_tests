use crate::infra::{config, http::start_http_server};
use log::info;
use sqlx::{Pool, Postgres};

mod adapter;
mod application;
mod domain;
mod infra;
#[cfg(test)]
mod tests_e2e;

pub struct AppState {
  pub postgres_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  config::log::setup_logger().expect("log configuration failed!");

  info!("starting postgres pool creation");
  let postgres_pool = config::database::postgres_pool(None)
    .await
    .expect("postgres pool creation failed!");

  info!("starting application");
  start_http_server(AppState { postgres_pool }).await
}
