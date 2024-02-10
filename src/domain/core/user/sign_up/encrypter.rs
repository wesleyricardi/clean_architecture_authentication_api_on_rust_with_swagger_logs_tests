use crate::domain::{
  core::user::{repository::UserRepository, User},
  error::AppError,
  utilities::crypto::Crypto,
};

use super::*;

#[derive(PartialEq, Default, Clone)]
pub struct PasswordEncrypted(pub String);

type UserSignUpStateIn<'a, Id, R> = User<'a, SignUp<Request<'a, Id, Password>, NotSaved>, R>;
type UserSignUpStateOut<'a, Id, R> =
  User<'a, SignUp<Request<'a, Id, PasswordEncrypted>, NotSaved>, R>;

impl<'a, Id, R: UserRepository> UserSignUpStateIn<'a, Id, R> {
  pub fn encrypt_password(
    self,
    cryto: &'a impl Crypto,
  ) -> Result<UserSignUpStateOut<'a, Id, R>, AppError> {
    let password_hash = PasswordEncrypted(cryto.hash_password(&self.state.request.password.0)?);

    Ok(User {
      repository: self.repository,
      state: SignUp {
        request: self.state.request.set_password(password_hash),
        saved: self.state.saved,
      },
    })
  }
}

#[cfg(test)]
mod tests {
  use mockall::predicate;

  use crate::domain::{
    core::user::mocks::repository::{build_mock_user_repository, Expectations},
    utilities::crypto::MockCrypto,
  };

  use super::*;

  #[test]
  fn test_encrypt_password_successfully() {
    const PASSWORD: &str = "123456789";
    const HASH_PASSWORD: &str = "mg3824m1htv8913dxjrn9ui45g801q43tj";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_request = Request {
      id: NotId,
      password: Password(PASSWORD.to_string()),
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: SignUp {
        request: mock_request,
        ..Default::default()
      },
    };

    let mut mock_crypto = MockCrypto::new();

    mock_crypto
      .expect_hash_password()
      .times(1)
      .with(predicate::eq(PASSWORD))
      .returning(|_| Ok(HASH_PASSWORD.to_owned()));
    let sut = user.encrypt_password(&mock_crypto).unwrap();

    assert_eq!(sut.state.request.password.0, HASH_PASSWORD);
  }
}
