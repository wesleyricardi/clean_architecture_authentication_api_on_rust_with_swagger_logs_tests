use crate::domain::{
  core::user::{repository::UserRepository, User},
  entities::user::UserData,
  error::AppError,
  utilities::crypto::Crypto,
};

use super::*;

type UserChangePwdStateIn<'a, R> =
  User<'a, ChangePassword<Request<'a>, UserData, PasswordChecked, NotSaved>, R>;
type UserChangePwdStateOut<'a, R> =
  User<'a, ChangePassword<RequestEncrypted<'a>, UserData, PasswordChecked, NotSaved>, R>;

impl<'a, R: UserRepository> UserChangePwdStateIn<'a, R> {
  pub fn encrypt_password(
    self,
    cryto: &'a impl Crypto,
  ) -> Result<UserChangePwdStateOut<'a, R>, AppError> {
    let password_hash = cryto.hash_password(self.state.request.password)?;

    let request_encrypt = RequestEncrypted {
      profile_id: self.state.request.profile_id,
      password: password_hash.to_owned(),
    };

    Ok(User {
      repository: self.repository,
      state: ChangePassword {
        request: request_encrypt,
        db_data: self.state.db_data,
        password_checked: self.state.password_checked,
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
  fn test_encrypt_password_successfully() {
    const PASSWORD: &str = "123456789";
    const HASH_PASSWORD: &str = "mg3824m1htv8913dxjrn9ui45g801q43tj";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_request = Request {
      password: PASSWORD,
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

    let mut mock_crypto = MockCrypto::new();

    mock_crypto
      .expect_hash_password()
      .times(1)
      .with(predicate::eq(PASSWORD))
      .returning(|_| Ok(HASH_PASSWORD.to_owned()));

    let sut = user.encrypt_password(&mock_crypto).unwrap();

    assert_eq!(sut.state.request.password, HASH_PASSWORD)
  }
}
