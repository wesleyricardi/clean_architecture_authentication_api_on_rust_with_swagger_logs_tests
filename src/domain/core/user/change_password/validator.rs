use crate::domain::{
  core::user::{repository::UserRepository, User},
  entities::user::UserData,
  error::AppError,
  utilities::crypto::Crypto,
};

use super::*;

type UserChangePwdStateIn<'a, R> =
  User<'a, ChangePassword<Request<'a>, UserData, PasswordNotChecked, NotSaved>, R>;
type UserChangePwdStateOut<'a, R> =
  User<'a, ChangePassword<Request<'a>, UserData, PasswordChecked, NotSaved>, R>;

impl<'a, R: UserRepository> UserChangePwdStateIn<'a, R> {
  pub fn check_password(
    self,
    cryto: &'a impl Crypto,
  ) -> Result<UserChangePwdStateOut<'a, R>, AppError> {
    if !cryto.verify_password(
      &self.state.db_data.password,
      self.state.request.old_password,
    )? {
      return Err(AppError::permission_denied(
        "The given old password is invalid",
      ));
    }

    Ok(User {
      repository: self.repository,
      state: ChangePassword {
        request: self.state.request,
        db_data: self.state.db_data,
        password_checked: PasswordChecked(true),
        saved: self.state.saved,
      },
    })
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
  fn test_check_old_password_matches() {
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
      old_password: PASSWORD,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
        db_data: mock_db_data,
        request: mock_request,
        password_checked: PasswordNotChecked,
        saved: NotSaved,
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
  fn test_check_old_password_not_matches() {
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
      old_password: WRONG_PASSWORD,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: ChangePassword {
        db_data: mock_db_data,
        request: mock_request,
        password_checked: PasswordNotChecked,
        saved: NotSaved,
      },
    };

    let mut mock_crypto = MockCrypto::new();

    mock_crypto
      .expect_verify_password()
      .times(1)
      .with(predicate::eq(HASH_PASSWORD), predicate::eq(WRONG_PASSWORD))
      .returning(|_, _| Ok(false));

    let sut = user.check_password(&mock_crypto).err();

    assert_eq!(
      sut,
      Some(AppError::permission_denied(
        "The given old password is invalid",
      ))
    );
  }
}
