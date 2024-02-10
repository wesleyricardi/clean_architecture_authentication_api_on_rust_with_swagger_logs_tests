use crate::domain::{
  core::user::{repository::UserRepository, User},
  entities::user::{UserColumns, UserData},
  error::AppError,
};

use super::*;

impl<'a, R: UserRepository>
  User<'a, ChangePassword<Request<'a>, NoDbData, PasswordNotChecked, NotSaved>, R>
{
  pub async fn get_user(
    self,
  ) -> Result<
    User<'a, ChangePassword<Request<'a>, UserData, PasswordNotChecked, NotSaved>, R>,
    AppError,
  > {
    let user_data = self
      .repository
      .find_user_by(&UserColumns::Id(self.state.request.profile_id))
      .await?;

    Ok(User {
      repository: self.repository,
      state: ChangePassword {
        request: self.state.request,
        db_data: user_data,
        password_checked: self.state.password_checked,
        saved: self.state.saved,
      },
    })
  }
}

impl<'a, R: UserRepository>
  User<'a, ChangePassword<RequestEncrypted<'a>, UserData, PasswordChecked, NotSaved>, R>
{
  pub async fn save(
    self,
  ) -> Result<
    User<'a, ChangePassword<RequestEncrypted<'a>, UserData, PasswordChecked, UpdateSaved>, R>,
    AppError,
  > {
    self
      .repository
      .update_password(&self.state.request.password, self.state.request.profile_id)
      .await?;

    Ok(User {
      repository: self.repository,
      state: ChangePassword {
        request: self.state.request,
        db_data: self.state.db_data,
        password_checked: self.state.password_checked,
        saved: UpdateSaved,
      },
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::domain::{
    core::user::mocks::repository::{
      build_mock_user_repository, Expectations, FindUserBy, UpdatePassword,
    },
    entities::user::UserColumns,
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
        param_column_with: UserColumns::Id(UUID),
      }),
      ..Default::default()
    });

    let mock_request = Request {
      profile_id: UUID,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
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
    )
  }

  #[tokio::test]
  async fn test_fail_get_user_db_data() {
    const DB_ERROR_MESSAGE: &str = "Some error in the database";

    let mock_repository = build_mock_user_repository(Expectations {
      find_user_by: Some(FindUserBy {
        fn_returning: |_| Err(AppError::database_error(DB_ERROR_MESSAGE)),
        ..Default::default()
      }),
      ..Default::default()
    });

    let mock_request = Request {
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
        request: mock_request,
        ..Default::default()
      },
    };

    let sut = user.get_user().await.err();

    assert_eq!(sut, Some(AppError::database_error(DB_ERROR_MESSAGE)))
  }

  #[tokio::test]
  async fn test_save_update_successfully() {
    let mock_repository = build_mock_user_repository(Expectations {
      update_password: Some(UpdatePassword {
        ..Default::default()
      }),
      ..Default::default()
    });

    let mock_request = RequestEncrypted {
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
        request: mock_request,
        db_data: UserData {
          ..Default::default()
        },
        password_checked: PasswordChecked(true),
        saved: NotSaved,
      },
    };

    let sut = user.save().await.unwrap();

    assert_eq!(sut.state.saved, UpdateSaved)
  }

  #[tokio::test]
  async fn test_save_update_failed() {
    const DB_ERROR_MESSAGE: &str = "Some error in the database";

    let mock_repository = build_mock_user_repository(Expectations {
      update_password: Some(UpdatePassword {
        fn_returning: |_, _| Err(AppError::database_error(DB_ERROR_MESSAGE)),
        ..Default::default()
      }),
      ..Default::default()
    });

    let mock_request = RequestEncrypted {
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
        request: mock_request,
        db_data: UserData {
          ..Default::default()
        },
        password_checked: PasswordChecked(true),
        saved: NotSaved,
      },
    };

    let sut = user.save().await.err();

    assert_eq!(sut, Some(AppError::database_error(DB_ERROR_MESSAGE)))
  }
}
