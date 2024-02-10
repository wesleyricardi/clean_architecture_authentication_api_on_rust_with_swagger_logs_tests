use chrono::NaiveDate;
use sqlx::PgPool;

use crate::domain::entities::user::{UserColumns, UserData};

pub async fn insert_user(
  pool: &PgPool,
  id: &str,
  name: &str,
  username: &str,
  birth_date: &str,
  gender_id: i32,
  password: &str,
  street: &str,
  neighborhood: &str,
  city_id: i32,
  postal_code: i32,
  email: &str,
  telephone: Option<&str>,
) {
  let birth_date_naive = NaiveDate::parse_from_str(birth_date, "%Y-%m-%d").unwrap();

  sqlx::query!(
    "SELECT FROM insert_user_profile($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
    id,
    name,
    username,
    birth_date_naive,
    gender_id,
    password,
    street,
    neighborhood,
    city_id,
    postal_code,
    email,
    telephone,
  )
  .execute(pool)
  .await
  .unwrap();
}

pub async fn assert_user(
  pool: &PgPool,
  by_column: &UserColumns<'_>,
  expect_user_data: UserData,
  check_password: Option<fn(password: &str, hash: &str) -> bool>,
) {
  let (col, value) = match by_column {
    UserColumns::Username(username) => ("profiles.username", *username),
    UserColumns::Email(email) => ("emails.address", *email),
    UserColumns::Telephone(telephone) => ("telephones.number", *telephone),
    UserColumns::Id(id) => ("profiles.id", *id),
  };

  let user = sqlx::query!("SELECT * FROM sign_in_by($1, $2)", col, value)
    .fetch_one(pool)
    .await
    .unwrap();

  assert_eq!(user.id.unwrap(), expect_user_data.id);
  assert_eq!(user.username.unwrap(), expect_user_data.username);
  assert_eq!(user.name.unwrap(), expect_user_data.name);
  if let Some(check_pwd) = check_password {
    assert!(
      check_pwd(&expect_user_data.password, &user.password.unwrap()),
      "the given password: {} is not correct",
      expect_user_data.password
    );
  }
}
