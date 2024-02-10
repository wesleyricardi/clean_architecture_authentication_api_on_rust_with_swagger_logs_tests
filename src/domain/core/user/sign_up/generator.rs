use crate::domain::{
  core::user::{repository::UserRepository, User},
  utilities::id_generator::IDGenerator,
};

use super::*;

impl<'a, P, R: UserRepository> User<'a, SignUp<Request<'a, NotId, P>, NotSaved>, R> {
  pub fn create_id(
    self,
    id_generator: &'a impl IDGenerator,
  ) -> User<'a, SignUp<Request<'a, String, P>, NotSaved>, R> {
    let id = id_generator.new_uuid();

    User {
      repository: self.repository,
      state: SignUp {
        request: self.state.request.set_id(id),
        saved: self.state.saved,
      },
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::{
    core::user::mocks::repository::{build_mock_user_repository, Expectations},
    utilities::id_generator::MockIDGenerator,
  };

  use super::*;

  #[test]
  fn test_create_id_successfully() {
    const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
    const PASSWORD: &str = "123456789";

    let mock_repository = build_mock_user_repository(Expectations {
      ..Default::default()
    });

    let mock_request = Request {
      id: NotId,
      password: PASSWORD,
      ..Default::default()
    };

    let user = User {
      repository: &mock_repository,
      state: SignUp {
        request: mock_request,
        ..Default::default()
      },
    };

    let mut mock_id_generator = MockIDGenerator::new();

    mock_id_generator
      .expect_new_uuid()
      .times(1)
      .returning(|| ID.to_owned());

    let sut = user.create_id(&mock_id_generator);

    assert_eq!(sut.state.request.id, ID);
  }
}
