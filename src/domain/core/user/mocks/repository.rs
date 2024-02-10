use crate::domain::{
  entities::user::{UserColumns, UserData},
  error::AppError,
};

use super::super::{
  repository::MockUserRepository,
  sign_up::{self, encrypter::PasswordEncrypted},
};

pub struct FindUserBy<'a> {
  pub calls: usize,
  pub param_column_with: UserColumns<'a>,
  pub fn_returning: fn(&UserColumns<'_>) -> Result<UserData, AppError>,
}

impl<'a> Default for FindUserBy<'a> {
  fn default() -> Self {
    Self {
      calls: 1,
      param_column_with: Default::default(),
      fn_returning: |_| {
        Ok(UserData {
          ..Default::default()
        })
      },
    }
  }
}

pub struct Store<'a> {
  pub calls: usize,
  pub param_user_data: sign_up::Request<'a, String, PasswordEncrypted>,
  pub fn_returning:
    for<'b> fn(&sign_up::Request<'b, String, PasswordEncrypted>) -> Result<(), AppError>,
}

impl<'a> Default for Store<'a> {
  fn default() -> Self {
    Self {
      calls: 1,
      param_user_data: sign_up::Request {
        ..Default::default()
      },
      fn_returning: |_| Ok(()),
    }
  }
}

pub struct UpdatePassword {
  pub calls: usize,
  pub param_password: String,
  pub param_profile_id: String,
  pub fn_returning: fn(&str, &str) -> Result<(), AppError>,
}

impl Default for UpdatePassword {
  fn default() -> Self {
    Self {
      calls: 1,
      param_password: Default::default(),
      param_profile_id: Default::default(),
      fn_returning: |_, _| Ok(()),
    }
  }
}

#[derive(Default)]
pub struct Expectations<'a> {
  pub find_user_by: Option<FindUserBy<'a>>,
  pub store: Option<Store<'a>>,
  pub update_password: Option<UpdatePassword>,
}

pub fn build_mock_user_repository(expectations: Expectations<'static>) -> MockUserRepository {
  let mut repository = MockUserRepository::new();

  expectations.find_user_by.map(|value| {
    let FindUserBy {
      calls,
      param_column_with,
      fn_returning,
    } = value;

    repository
      .expect_find_user_by()
      .withf(move |column| *column == param_column_with)
      .times(calls)
      .returning(fn_returning);
  });

  expectations.store.map(|value| {
    let Store {
      calls,
      param_user_data,
      fn_returning,
    } = value;

    repository
      .expect_store()
      .times(calls)
      .withf(move |user_data| {
        if *user_data != param_user_data {
          return false;
        }

        true
      })
      .returning(fn_returning)
  });

  expectations.update_password.map(|value| {
    let UpdatePassword {
      calls,
      param_password,
      param_profile_id,
      fn_returning,
    } = value;

    repository
      .expect_update_password()
      .times(calls)
      .withf(move |password, profile_id| {
        if password.to_string() == param_password && profile_id.to_string() == param_profile_id {
          true
        } else {
          false
        }
      })
      .returning(fn_returning)
  });

  repository
}
