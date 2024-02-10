use super::*;

pub mod encrypter;
pub mod repository;
pub mod validator;

#[derive(Default)]
pub struct NoDbData;

#[derive(Default)]
pub struct PasswordNotChecked;

#[derive(Debug, PartialEq)]
pub struct PasswordChecked(pub(super) bool);

#[derive(Default)]
pub struct NotSaved;

#[derive(Debug, PartialEq)]
pub struct UpdateSaved;

#[derive(Default)]
pub struct ChangePassword<Req, Db, PwdCheck, Save> {
  request: Req,
  db_data: Db,
  password_checked: PwdCheck,
  saved: Save,
}

impl<'a, R: UserRepository> User<'a, NoState, R> {
  pub fn change_password(
    self,
    request: Request<'a>,
  ) -> User<ChangePassword<Request<'a>, NoDbData, PasswordNotChecked, NotSaved>, R> {
    User {
      repository: self.repository,
      state: ChangePassword {
        request,
        ..Default::default()
      },
    }
  }
}

#[derive(Default, Debug, PartialEq)]
pub struct Request<'a> {
  pub profile_id: &'a str,
  pub password: &'a str,
  pub old_password: &'a str,
}

#[derive(Default)]
pub struct RequestEncrypted<'a> {
  pub(super) profile_id: &'a str,
  pub(super) password: String,
}

#[cfg(test)]
mod tests {
  use crate::domain::core::user::mocks::repository::{build_mock_user_repository, Expectations};

  use super::*;

  #[test]
  fn test_sign_build() {
    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });
    let mock_request = Request {
      ..Default::default()
    };

    let sut = User::new(&mock_repository).change_password(mock_request);

    assert_eq!(
      sut.state.request,
      Request {
        ..Default::default()
      }
    );
  }
}
