use crate::domain::{
  core::user::{repository::UserRepository, User},
  error::AppError,
};

use super::{encrypter::PasswordEncrypted, NotSaved, Request, Saved, SignUp};

impl<'a, R: UserRepository> User<'a, SignUp<Request<'a, String, PasswordEncrypted>, NotSaved>, R> {
  pub async fn store(
    self,
  ) -> Result<User<'a, SignUp<Request<'a, String, PasswordEncrypted>, Saved>, R>, AppError> {
    self.repository.store(&self.state.request).await?;

    Ok(User {
      repository: self.repository,
      state: SignUp {
        request: self.state.request,
        saved: Saved(true),
      },
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::core::user::mocks::repository::{
    build_mock_user_repository, Expectations, Store,
  };

  use super::*;

  #[tokio::test]
  async fn test_save_successfully() {
    const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    let password_hash = PasswordEncrypted("123456789".to_string());

    let mock_request = Request {
      id: ID.to_owned(),
      password: password_hash,
      ..Default::default()
    };

    let mock_repository = build_mock_user_repository(Expectations {
      store: Some(Store {
        calls: 1,
        param_user_data: mock_request.clone(),
        fn_returning: |_| Ok(()),
      }),
      ..Default::default()
    });

    let user = User {
      repository: &mock_repository,
      state: SignUp {
        request: mock_request,
        ..Default::default()
      },
    };

    let sut = user.store().await.unwrap();

    assert_eq!(sut.state.saved, Saved(true));
  }

  #[tokio::test]
  async fn test_save_failed() {
    const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    let password_hash = PasswordEncrypted("123456789".to_string());
    const DB_ERROR_MESSAGE: &str = "Some error in the database";

    let mock_request = Request {
      id: ID.to_owned(),
      password: password_hash,
      ..Default::default()
    };

    let mock_repository = build_mock_user_repository(Expectations {
      store: Some(Store {
        calls: 1,
        param_user_data: mock_request.clone(),
        fn_returning: |_| Err(AppError::database_error(DB_ERROR_MESSAGE)),
      }),
      ..Default::default()
    });

    let user = User {
      repository: &mock_repository,
      state: SignUp {
        request: mock_request,
        ..Default::default()
      },
    };

    let sut = user.store().await.err();

    assert_eq!(sut, Some(AppError::database_error(DB_ERROR_MESSAGE)));
  }
}
