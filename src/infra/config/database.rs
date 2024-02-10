use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};

pub async fn postgres_pool(db_url: Option<String>) -> Result<Pool<Postgres>, Error> {
  let db_url =
    db_url.unwrap_or(std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var"));

  PgPoolOptions::new()
    .acquire_timeout(Duration::from_secs(30))
    .max_connections(100)
    .connect(&db_url)
    .await
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_connection() {
    let pool = postgres_pool(None).await.unwrap();

    let res = sqlx::query!("SELECT 1 + 1 as sum")
      .fetch_one(&pool)
      .await
      .unwrap();

    assert_eq!(res.sum, Some(2));
  }
}
