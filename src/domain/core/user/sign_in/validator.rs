use super::*;
use crate::domain::{
  core::user::repository::UserRepository, entities::user::UserData, error::AppError,
  utilities::crypto::Crypto,
};

impl<'a, R: UserRepository> User<'a, SignIn<Request<'a>, UserData, PasswordNotChecked>, R> {
  pub fn check_password(
    self,
    cryto: &'a impl Crypto,
  ) -> Result<User<'a, SignIn<Request<'a>, UserData, PasswordChecked>, R>, AppError> {
    let req_password = self.state.request.password;
    let hash_password = &self.state.db_data.password;

    if !cryto.verify_password(hash_password, req_password)? {
      return Err(AppError::unauthenticated("invalid password"));
    }

    let user = User {
      repository: self.repository,
      state: SignIn {
        password_checked: PasswordChecked(true),
        db_data: self.state.db_data,
        request: self.state.request,
      },
    };

    Ok(user)
  }
}

#[cfg(test)]
mod tests {
  use mockall::predicate;

  use super::*;
  use crate::domain::{
    core::user::mocks::repository::{build_mock_user_repository, Expectations},
    utilities::crypto::MockCrypto,
  };

  #[test]
  fn test_check_password_with_valid_password() {
    const PASSWORD: &str = "123456789";
    const HASH_PASSWORD: &str = "mg3824m1htv8913dxjrn9ui45g801q43tj";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_db_data = UserData {
      password: HASH_PASSWORD.to_owned(),
      ..Default::default()
    };

    let mock_request = Request {
      password: PASSWORD,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: SignIn {
        request: mock_request,
        db_data: mock_db_data,
        ..Default::default()
      },
    };

    let mut mock_crypto = MockCrypto::new();

    mock_crypto
      .expect_verify_password()
      .times(1)
      .with(predicate::eq(HASH_PASSWORD), predicate::eq(PASSWORD))
      .returning(|_, _| Ok(true));

    let sut = user.check_password(&mock_crypto).unwrap();

    assert!(sut.state.password_checked.0);
  }

  #[test]
  fn test_check_password_with_wrong_password() {
    const HASH_PASSWORD: &str = "mg3824m1htv8913dxjrn9ui45g801q43tj";
    const WRONG_PASSWORD: &str = "987654321";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_db_data = UserData {
      password: HASH_PASSWORD.to_owned(),
      ..Default::default()
    };

    let mock_request = Request {
      password: WRONG_PASSWORD,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: SignIn {
        request: mock_request,
        db_data: mock_db_data,
        ..Default::default()
      },
    };

    let mut mock_crypto = MockCrypto::new();

    mock_crypto
      .expect_verify_password()
      .times(1)
      .with(predicate::eq(HASH_PASSWORD), predicate::eq(WRONG_PASSWORD))
      .returning(|_, _| Ok(false));

    let sut = user.check_password(&mock_crypto).err();

    assert_eq!(sut, Some(AppError::unauthenticated("invalid password")));
  }
}
