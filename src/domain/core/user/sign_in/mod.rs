use super::repository::UserRepository;
pub(super) use super::*;
use crate::domain::entities::user::UserColumns;
pub mod repository;
pub mod responder;
pub mod validator;

#[derive(Default)]
pub struct NoDbData;

#[derive(Default)]
pub struct PasswordNotChecked;

#[derive(Debug, PartialEq)]
pub struct PasswordChecked(pub(super) bool);

#[derive(Default)]
pub struct SignIn<Req, Db, PwdCheck> {
  request: Req,
  db_data: Db,
  password_checked: PwdCheck,
}

#[derive(Default, PartialEq, Debug)]
pub struct Request<'a> {
  pub column: UserColumns<'a>,
  pub password: &'a str,
}

impl<'a, R: UserRepository> User<'a, NoState, R> {
  pub fn sign_in(
    self,
    request: Request<'a>,
  ) -> User<SignIn<Request<'a>, NoDbData, PasswordNotChecked>, R> {
    User {
      repository: self.repository,
      state: SignIn {
        request,
        ..Default::default()
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::core::user::mocks::repository::{build_mock_user_repository, Expectations};

  use super::*;

  #[test]
  fn test_sign_in_build() {
    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });
    let mock_request = Request {
      ..Default::default()
    };

    let user_sign_in = User::new(&mock_repository).sign_in(mock_request);

    assert_eq!(
      user_sign_in.state.request,
      Request {
        ..Default::default()
      }
    );
  }
}
