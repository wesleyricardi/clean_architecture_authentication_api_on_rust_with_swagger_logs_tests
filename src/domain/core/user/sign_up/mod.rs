pub mod encrypter;
pub mod generator;
pub mod repository;
pub mod responder;

use super::*;
use crate::domain::types::Password;
use chrono::NaiveDate;

#[derive(Default)]
pub struct NotSaved;

#[derive(Debug, PartialEq)]
pub struct Saved(pub(super) bool);

#[derive(Default)]
pub struct SignUp<Req, Save> {
  request: Req,
  pub(super) saved: Save,
}

impl<'a, R: UserRepository> User<'a, NoState, R> {
  pub fn sign_up(
    self,
    request: Request<'a, NotId, Password>,
  ) -> User<SignUp<Request<'a, NotId, Password>, NotSaved>, R> {
    User {
      repository: self.repository,
      state: SignUp {
        request,
        ..Default::default()
      },
    }
  }
}

#[derive(Default, PartialEq, Debug)]
pub struct NotId;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct Request<'a, Id, Password> {
  pub id: Id,
  pub name: &'a str,
  pub username: &'a str,
  pub birth_date: NaiveDate,
  pub gender_id: i32,
  pub password: Password,
  pub street: &'a str,
  pub neighborhood: &'a str,
  pub city_id: i32,
  pub postal_code: i32,
  pub email_address: &'a str,
  pub telephone_number: Option<&'a str>,
}

impl<'a, P> Request<'a, NotId, P> {
  pub(super) fn set_id<Id>(self, id: Id) -> Request<'a, Id, P> {
    Request {
      id,
      name: self.name,
      username: self.username,
      birth_date: self.birth_date,
      gender_id: self.gender_id,
      password: self.password,
      street: self.street,
      neighborhood: self.neighborhood,
      city_id: self.city_id,
      postal_code: self.postal_code,
      email_address: self.email_address,
      telephone_number: self.telephone_number,
    }
  }
}
impl<'a, Id> Request<'a, Id, Password> {
  pub(super) fn set_password<P>(self, password: P) -> Request<'a, Id, P> {
    Request {
      id: self.id,
      name: self.name,
      username: self.username,
      birth_date: self.birth_date,
      gender_id: self.gender_id,
      password,
      street: self.street,
      neighborhood: self.neighborhood,
      city_id: self.city_id,
      postal_code: self.postal_code,
      email_address: self.email_address,
      telephone_number: self.telephone_number,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::core::user::mocks::repository::{build_mock_user_repository, Expectations};

  use super::*;

  #[test]
  fn test_sign_up_build() {
    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });
    let mock_request = Request {
      ..Default::default()
    };

    let sut = User::new(&mock_repository).sign_up(mock_request);

    assert_eq!(
      sut.state.request,
      Request {
        ..Default::default()
      }
    );
  }
}
