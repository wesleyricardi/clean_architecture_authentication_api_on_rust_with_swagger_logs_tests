use crate::domain::{
  core::user::repository::UserRepository,
  entities::user::{PublicUserData, UserData},
};

use super::*;

impl<'a, R: UserRepository> User<'a, SignIn<Request<'a>, UserData, PasswordChecked>, R> {
  pub fn response(self) -> PublicUserData {
    PublicUserData {
      id: self.state.db_data.id,
      username: self.state.db_data.username,
      name: self.state.db_data.name,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::domain::core::user::mocks::repository::{build_mock_user_repository, Expectations};

  #[test]
  fn test_response_with_right_data() {
    const UUID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    const NAME: &str = "john doe";
    const USERNAME: &str = "john_doe";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_request = Request {
      ..Default::default()
    };

    let mock_db_data = UserData {
      id: UUID.to_string(),
      name: NAME.to_string(),
      username: USERNAME.to_string(),
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: SignIn {
        request: mock_request,
        db_data: mock_db_data,
        password_checked: PasswordChecked(true),
      },
    };

    let sut = user.response();

    assert_eq!(
      sut,
      PublicUserData {
        id: UUID.to_string(),
        name: NAME.to_string(),
        username: USERNAME.to_string()
      }
    )
  }
}
