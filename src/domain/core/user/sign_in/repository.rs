use crate::domain::{
  core::user::repository::UserRepository, entities::user::UserData, error::AppError,
};

use super::*;

impl<'a, R: UserRepository> User<'a, SignIn<Request<'a>, NoDbData, PasswordNotChecked>, R> {
  pub async fn get_user(
    self,
  ) -> Result<User<'a, SignIn<Request<'a>, UserData, PasswordNotChecked>, R>, AppError> {
    let db_data = self
      .repository
      .find_user_by(&self.state.request.column)
      .await?;

    let user = User {
      repository: self.repository,
      state: SignIn {
        db_data,
        request: self.state.request,
        password_checked: self.state.password_checked,
      },
    };

    Ok(user)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::domain::core::user::mocks::repository::{
    build_mock_user_repository, Expectations, FindUserBy,
  };

  #[tokio::test]
  async fn test_get_user_db_data_successfully() {
    const UUID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    const NAME: &str = "john doe";
    const USERNAME: &str = "john_doe";
    const PASSWORD: &str = "n8dCo3qY60UuIiXt8zrSLWGXceduTgnO";

    let mock_repository = build_mock_user_repository(Expectations {
      find_user_by: Some(FindUserBy {
        calls: 1,
        fn_returning: |_| {
          Ok(UserData {
            id: UUID.to_string(),
            name: NAME.to_string(),
            username: USERNAME.to_string(),
            password: PASSWORD.to_string(),
          })
        },
        param_column_with: UserColumns::Username(USERNAME),
      }),
      ..Default::default()
    });

    let mock_request = Request {
      column: UserColumns::Username(USERNAME),
      password: PASSWORD,
    };

    let user = User {
      repository: &mock_repository,
      state: SignIn {
        request: mock_request,
        ..Default::default()
      },
    };

    let sut = user.get_user().await.unwrap();

    assert_eq!(
      sut.state.db_data,
      UserData {
        id: UUID.to_string(),
        name: NAME.to_string(),
        username: USERNAME.to_string(),
        password: PASSWORD.to_string(),
      }
    );
    assert_eq!(
      sut.state.request,
      Request {
        column: UserColumns::Username(USERNAME),
        password: PASSWORD,
      },
      "Get user data failed"
    )
  }

  #[tokio::test]
  async fn test_fail_get_user_db_data() {
    const USERNAME: &str = "john_doe";
    const PASSWORD: &str = "n8dCo3qY60UuIiXt8zrSLWGXceduTgnO";
    const DB_ERROR_MESSAGE: &str = "Some error in the database";

    let mock_repository = build_mock_user_repository(Expectations {
      find_user_by: Some(FindUserBy {
        calls: 1,
        fn_returning: |_| Err(AppError::database_error(DB_ERROR_MESSAGE)),
        param_column_with: UserColumns::Username(USERNAME),
      }),
      ..Default::default()
    });

    let mock_request = Request {
      column: UserColumns::Username(USERNAME),
      password: PASSWORD,
    };

    let user = User {
      repository: &mock_repository,
      state: SignIn {
        request: mock_request,
        ..Default::default()
      },
    };

    let sut = user.get_user().await.err();

    assert_eq!(
      sut,
      Some(AppError::database_error(DB_ERROR_MESSAGE)),
      "Get user data do not failed"
    );
  }
}
