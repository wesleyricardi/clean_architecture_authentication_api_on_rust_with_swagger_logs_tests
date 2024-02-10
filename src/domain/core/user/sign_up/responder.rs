use crate::domain::core::user::{repository::UserRepository, User};

use super::{encrypter::PasswordEncrypted, Request, Saved, SignUp};

impl<'a, R: UserRepository> User<'a, SignUp<Request<'a, String, PasswordEncrypted>, Saved>, R> {
  pub fn response(self) -> String {
    self.state.request.id
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::core::user::mocks::repository::{build_mock_user_repository, Expectations};

  use super::*;

  #[test]
  fn test_response_with_right_id() {
    const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    const PASSWORD: &str = "123456789";
    let password_hash = PasswordEncrypted(PASSWORD.to_string());

    let mock_request = Request {
      id: ID.to_owned(),
      password: password_hash,
      ..Default::default()
    };

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let user = User {
      repository: &mock_repository,
      state: SignUp {
        request: mock_request,
        saved: Saved(true),
      },
    };

    let sut = user.response();

    assert_eq!(sut, ID);
  }
}
