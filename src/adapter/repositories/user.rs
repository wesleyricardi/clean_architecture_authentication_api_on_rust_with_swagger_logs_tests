use crate::domain::{
  core::user::{repository::UserRepository, sign_up},
  entities::user::{UserColumns, UserData},
  error::AppError,
};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

pub struct UserRepositoryDB<'a, P> {
  pub pool: &'a P,
}

type NewUser<'a> = sign_up::Request<'a, String, sign_up::encrypter::PasswordEncrypted>;

#[async_trait]
impl UserRepository for UserRepositoryDB<'_, Pool<Postgres>> {
  async fn find_user_by<'a>(&self, column: &UserColumns<'a>) -> Result<UserData, AppError> {
    let (col, value) = match column {
      UserColumns::Username(username) => ("profiles.username", *username),
      UserColumns::Email(email) => ("emails.address", *email),
      UserColumns::Telephone(telephone) => ("telephones.number", *telephone),
      UserColumns::Id(id) => ("profiles.id", *id),
    };

    let user = sqlx::query!("SELECT * FROM sign_in_by($1, $2)", col, value)
      .fetch_one(self.pool)
      .await?;

    //unwrap below has been checked, no column is possible null
    Ok(UserData {
      id: user.id.unwrap(),
      name: user.name.unwrap(),
      username: user.username.unwrap(),
      password: user.password.unwrap(),
    })
  }

  async fn store<'a>(&self, user_data: &NewUser<'a>) -> Result<(), AppError> {
    sqlx::query!(
      "SELECT FROM insert_user_profile($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
      user_data.id.to_string(),
      user_data.name,
      user_data.username,
      user_data.birth_date,
      user_data.gender_id,
      user_data.password.0,
      user_data.street,
      user_data.neighborhood,
      user_data.city_id,
      user_data.postal_code,
      user_data.email_address,
      user_data.telephone_number.as_deref(),
    )
    .execute(self.pool)
    .await?;

    Ok(())
  }

  async fn update_password(&self, password: &str, profile_id: &str) -> Result<(), AppError> {
    let query_result = sqlx::query!(
      "UPDATE users 
      SET 
        password = $1
      FROM
        profiles 
      WHERE 
        profiles.user_id = users.id
        AND profiles.id = $2",
      password,
      profile_id
    )
    .execute(self.pool)
    .await?;

    if query_result.rows_affected() < 1 {
      return Err(AppError::invalid_argument(
        "wrong old password or user does not exist",
      ));
    }

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::{
    domain::error::Code,
    tests_e2e::helpers::user_repository::{assert_user, insert_user},
  };

  use super::*;
  use chrono::NaiveDate;
  use sqlx::PgPool;

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_store_new_user(pool: PgPool) -> sqlx::Result<()> {
    let sut = UserRepositoryDB { pool: &pool };

    sut
      .store(&sign_up::Request {
        id: ID.to_owned(),
        name: NAME,
        username: USERNAME,
        birth_date: NaiveDate::parse_from_str(BIRTH_DATE, "%Y-%m-%d").unwrap(),
        gender_id: GENDER_ID,
        password: sign_up::encrypter::PasswordEncrypted(PASSWORD.to_string()),
        street: STREET,
        neighborhood: NEIGHBORHOOD,
        city_id: CITY_ID,
        postal_code: POSTAL_CODE,
        email_address: EMAIL_ADDRESS,
        telephone_number: Some(TELEPHONE_NUMBER),
      })
      .await
      .unwrap();

    assert_user(
      &pool,
      &UserColumns::Username(USERNAME),
      UserData {
        id: ID.to_owned(),
        name: NAME.to_owned(),
        username: USERNAME.to_owned(),
        password: PASSWORD.to_owned(),
      },
      Some(|req_pwd, db_pwd| req_pwd == db_pwd),
    )
    .await;

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_try_store_duplicate_user(pool: PgPool) -> sqlx::Result<()> {
    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };
    let response = sut
      .store(&sign_up::Request {
        id: ID.to_owned(),
        name: NAME,
        username: USERNAME,
        birth_date: NaiveDate::parse_from_str(BIRTH_DATE, "%Y-%m-%d").unwrap(),
        gender_id: GENDER_ID,
        password: sign_up::encrypter::PasswordEncrypted(PASSWORD.to_string()),
        street: STREET,
        neighborhood: NEIGHBORHOOD,
        city_id: CITY_ID,
        postal_code: POSTAL_CODE,
        email_address: EMAIL_ADDRESS,
        telephone_number: Some(TELEPHONE_NUMBER),
      })
      .await;

    match response {
      Ok(_) => panic!("should not be success"),
      Err(error) => {
        assert_eq!(error.code, Code::AlreadyExists, "fail but wrong code");
      }
    };

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_find_user_using_username(pool: PgPool) -> sqlx::Result<()> {
    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut
      .find_user_by(&UserColumns::Username(USERNAME))
      .await
      .unwrap();

    assert_eq!(
      response,
      UserData {
        id: ID.to_string(),
        name: NAME.to_string(),
        password: PASSWORD.to_string(),
        username: USERNAME.to_string(),
      }
    );

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_find_user_using_id(pool: PgPool) -> sqlx::Result<()> {
    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut.find_user_by(&UserColumns::Id(ID)).await.unwrap();

    assert_eq!(
      response,
      UserData {
        id: ID.to_string(),
        name: NAME.to_string(),
        password: PASSWORD.to_string(),
        username: USERNAME.to_string(),
      }
    );

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_find_user_using_email(pool: PgPool) -> sqlx::Result<()> {
    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut
      .find_user_by(&UserColumns::Email(EMAIL_ADDRESS))
      .await
      .unwrap();

    assert_eq!(
      response,
      UserData {
        id: ID.to_string(),
        name: NAME.to_string(),
        password: PASSWORD.to_string(),
        username: USERNAME.to_string(),
      }
    );

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_find_user_using_telephone(pool: PgPool) -> sqlx::Result<()> {
    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut
      .find_user_by(&UserColumns::Telephone(TELEPHONE_NUMBER))
      .await
      .unwrap();

    assert_eq!(
      response,
      UserData {
        id: ID.to_string(),
        name: NAME.to_string(),
        password: PASSWORD.to_string(),
        username: USERNAME.to_string(),
      }
    );

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_search_with_invalid_user(pool: PgPool) -> sqlx::Result<()> {
    const WRONG_ID: &str = "3d59b3cc-4ba4-09a4-90cb-956c4a2df911";

    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut.find_user_by(&UserColumns::Id(WRONG_ID)).await;

    match response {
      Ok(_) => panic!("this test not expected success"),
      Err(error) => assert_eq!(error.code, Code::NotFound),
    };

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_update_user_password(pool: PgPool) -> sqlx::Result<()> {
    const NEW_PASSWORD: &str = "987654321";

    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };
    sut.update_password(NEW_PASSWORD, ID).await.unwrap();

    assert_user(
      &pool,
      &UserColumns::Id(ID),
      UserData {
        id: ID.to_owned(),
        name: NAME.to_owned(),
        username: USERNAME.to_owned(),
        password: NEW_PASSWORD.to_owned(),
      },
      Some(|new_pwd, db_pwd| new_pwd == db_pwd),
    )
    .await;

    Ok(())
  }

  #[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
  async fn test_update_user_password_with_wrong_id(pool: PgPool) -> sqlx::Result<()> {
    const NEW_PASSWORD: &str = "987654321";
    const WRONG_ID: &str = "3d59b3cc-4ba4-09a4-90cb-956c4a2df911";

    insert_user_default(&pool).await.unwrap();

    let sut = UserRepositoryDB { pool: &pool };

    let response = sut.update_password(NEW_PASSWORD, WRONG_ID).await;

    match response {
      Ok(_) => panic!("this test should no be success"),
      Err(error) => assert_eq!(
        error,
        AppError::invalid_argument("wrong old password or user does not exist")
      ),
    };

    Ok(())
  }

  const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
  const NAME: &str = "John Doe";
  const USERNAME: &str = "john.doe";
  const BIRTH_DATE: &str = "1990-01-01";
  const GENDER_ID: i32 = 1;
  const PASSWORD: &str = "123456789";
  const STREET: &str = "153 W 57th St";
  const NEIGHBORHOOD: &str = "manhattan";
  const CITY_ID: i32 = 4;
  const POSTAL_CODE: i32 = 10019;
  const EMAIL_ADDRESS: &str = "johndoe@company.com";
  const TELEPHONE_NUMBER: &str = "+1 151 999-9999";

  async fn insert_user_default(pool: &Pool<Postgres>) -> Result<(), AppError> {
    insert_user(
      pool,
      ID,
      NAME,
      USERNAME,
      BIRTH_DATE,
      GENDER_ID,
      PASSWORD,
      STREET,
      NEIGHBORHOOD,
      CITY_ID,
      POSTAL_CODE,
      EMAIL_ADDRESS,
      Some(TELEPHONE_NUMBER).as_deref(),
    )
    .await;

    Ok(())
  }
}
