pub mod change_password;
#[cfg(test)]
pub mod mocks;
pub mod repository;
pub mod sign_in;
pub mod sign_up;

use self::repository::UserRepository;

pub struct NoRepository;
pub struct NoState;

pub struct User<'a, State = NoState, R = NoRepository> {
  state: State,
  repository: &'a R,
}

impl<'a> User<'a> {
  pub fn new<R: UserRepository>(repository: &'a R) -> User<NoState, R> {
    User {
      state: NoState,
      repository,
    }
  }
}
