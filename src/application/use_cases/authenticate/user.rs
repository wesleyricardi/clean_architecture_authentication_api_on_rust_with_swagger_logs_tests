use super::{
  ChangeUserPasswordRequest, UserAuthentication, UserAuthenticationResponse,
  UserRegistrationRequest, UserSignInRequest,
};
use crate::{
  application::services::{security::token_service::TokenService, Services},
  domain::{
    core::user::{repository::UserRepository, User},
    error::AppError,
    utilities::{crypto::Crypto, id_generator::IDGenerator, Utilities},
  },
};
use async_trait::async_trait;
use validator::Validate;

pub struct UserUseCase<'a, Repository, Token: TokenService, C: Crypto, ID: IDGenerator> {
  pub user_repository: &'a Repository,
  pub services: &'a Services<Token>,
  pub utilities: &'a Utilities<C, ID>,
}

#[async_trait]
impl<Repository: UserRepository, Token: TokenService, C: Crypto, ID: IDGenerator> UserAuthentication
  for UserUseCase<'_, Repository, Token, C, ID>
{
  async fn sign_in(
    &self,
    request: &UserSignInRequest<'_>,
  ) -> Result<UserAuthenticationResponse, AppError> {
    request
      .validate()
      .map_err(|err| AppError::invalid_argument(err.to_string()))?;

    let user = User::new(self.user_repository)
      .sign_in(request.try_into()?)
      .get_user()
      .await?
      .check_password(&self.utilities.crypto)?
      .response();

    let token =
      self
        .services
        .token
        .encode(user.id.clone(), String::from("authentication_user"), 120)?;

    Ok(UserAuthenticationResponse {
      id: user.id,
      name: user.name,
      username: user.username,
      token,
    })
  }

  async fn register(
    &self,
    request: &UserRegistrationRequest<'_>,
  ) -> Result<UserAuthenticationResponse, AppError> {
    request
      .validate()
      .map_err(|err| AppError::invalid_argument(err.to_string()))?;

    let user_id = User::new(self.user_repository)
      .sign_up(request.try_into()?)
      .encrypt_password(&self.utilities.crypto)?
      .create_id(&self.utilities.id_generator)
      .store()
      .await?
      .response();

    let token = self.services.token.encode(
      user_id.to_string(),
      String::from("authentication_user"),
      120,
    )?;

    Ok(UserAuthenticationResponse {
      id: user_id.to_string(),
      name: request.name.to_string(),
      username: request.username.to_string(),
      token,
    })
  }

  async fn update_password(
    &self,
    request: &ChangeUserPasswordRequest<'_>,
    token: &str,
  ) -> Result<(), AppError> {
    let token_decoded = self.services.token.decode(token)?;
    if token_decoded.aud != "authentication_user" {
      return Err(AppError::unauthenticated(
        "Given token is not valid for this service",
      ));
    } else if token_decoded.sub != request.profile_id {
      return Err(AppError::permission_denied(
        "Given token not have permission for this profile",
      ));
    }

    request
      .validate()
      .map_err(|err| AppError::invalid_argument(err.to_string()))?;

    User::new(self.user_repository)
      .change_password(request.into())
      .get_user()
      .await?
      .check_password(&self.utilities.crypto)?
      .encrypt_password(&self.utilities.crypto)?
      .save()
      .await?;

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::super::*;
  use super::*;
  use crate::{
    application::services::security::token_service::MockTokenService,
    domain::{
      core::user::mocks::repository::{build_mock_user_repository, Expectations},
      error::Code,
      utilities::{crypto::MockCrypto, id_generator::MockIDGenerator},
    },
  };

  #[tokio::test]
  async fn test_sign_in_successfully_with_username() {
    let request = UserSignInRequest {
      username: Some(USERNAME),
      password: PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &repository_find_by_username_successfully(),
      services: &Services {
        token: token_service_encode(),
      },
      utilities: &Utilities {
        crypto: crypto_verify_successfully(),
        id_generator: MockIDGenerator::new(),
      },
    };

    let response = sut.sign_in(&request).await.unwrap();

    assert_eq!(response, user_auth_response())
  }

  #[tokio::test]
  async fn test_sign_in_successfully_with_email() {
    let request = UserSignInRequest {
      email: Some(EMAIL_ADDRESS),
      password: PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &repository_find_by_email_successfully(),
      services: &Services {
        token: token_service_encode(),
      },
      utilities: &Utilities {
        crypto: crypto_verify_successfully(),
        id_generator: MockIDGenerator::new(),
      },
    };

    let response = sut.sign_in(&request).await.unwrap();

    assert_eq!(response, user_auth_response())
  }

  #[tokio::test]
  async fn test_sign_in_successfully_with_telephone() {
    let request = UserSignInRequest {
      telephone: Some(TELEPHONE_NUMBER),
      password: PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &repository_find_by_telephone_successfully(),
      services: &Services {
        token: token_service_encode(),
      },
      utilities: &Utilities {
        crypto: crypto_verify_successfully(),
        id_generator: MockIDGenerator::new(),
      },
    };

    let response = sut.sign_in(&request).await.unwrap();

    assert_eq!(response, user_auth_response())
  }

  #[tokio::test]
  async fn test_sign_in_with_wrong_password() {
    let request = UserSignInRequest {
      username: Some(USERNAME),
      password: WRONG_PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &repository_find_by_username_successfully(),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: crypto_verify_wrong_password(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.sign_in(&request).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => {
        assert_eq!(error, AppError::unauthenticated("invalid password"))
      }
    }
  }

  #[tokio::test]
  async fn test_sign_in_with_nonexistent_username() {
    let request = UserSignInRequest {
      username: Some(NONEXISTENT_USERNAME),
      password: WRONG_PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &repository_find_by_username_not_found(),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.sign_in(&request).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => {
        assert_eq!(
          error,
          AppError::not_found("not found user with given username")
        )
      }
    }
  }

  #[tokio::test]
  async fn test_sign_in_with_empty_password() {
    let request = UserSignInRequest {
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &build_mock_user_repository(Expectations {
        ..Default::default()
      }),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.sign_in(&request).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => {
        assert_eq!(
          error,
          AppError::invalid_argument("password: Password must contain minimum 8 characters!")
        )
      }
    }
  }

  #[tokio::test]
  async fn test_sign_in_with_no_method() {
    let request = UserSignInRequest {
      password: PASSWORD,
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &build_mock_user_repository(Expectations {
        ..Default::default()
      }),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.sign_in(&request).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => {
        assert_eq!(
          error,
          AppError::invalid_argument("no username, email ou telephone provide")
        )
      }
    }
  }

  #[tokio::test]
  async fn test_register_successfully() {
    let request = &user_register_request();

    let sut = UserUseCase {
      user_repository: &repository_save_successfully(),
      services: &Services {
        token: token_service_encode(),
      },
      utilities: &Utilities {
        crypto: crypto_hash_successfully(),
        id_generator: generate_id_successfully(),
      },
    };

    let response = sut.register(request.try_into().unwrap()).await.unwrap();

    assert_eq!(response, user_auth_response())
  }

  #[tokio::test]
  async fn test_register_already_existing() {
    let request = &user_register_request();

    let sut = UserUseCase {
      user_repository: &repository_save_already_existing(),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: crypto_hash_successfully(),
        id_generator: generate_id_successfully(),
      },
    };

    match sut.register(request.try_into().unwrap()).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => assert_eq!(error, AppError::already_exists("already exists")),
    }
  }

  #[tokio::test]
  async fn test_register_with_empty_fields() {
    let request = &UserRegistrationRequest {
      ..Default::default()
    };

    let sut = UserUseCase {
      user_repository: &build_mock_user_repository(Expectations {
        ..Default::default()
      }),
      services: &Services {
        token: MockTokenService::new(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.register(request.try_into().unwrap()).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => assert_eq!(error.code, Code::InvalidArgument),
    }
  }

  #[tokio::test]
  async fn test_change_password_successfully() {
    let request = &user_change_password_request();

    let sut = UserUseCase {
      user_repository: &repository_change_password_successfully(),
      services: &Services {
        token: token_service_decode_successfully(),
      },
      utilities: &Utilities {
        crypto: crypto_verify_and_hash_successfully(),
        id_generator: MockIDGenerator::new(),
      },
    };

    let response = sut.update_password(request, TOKEN).await.unwrap();

    assert_eq!(response, ());
  }

  #[tokio::test]
  async fn test_change_password_nonexisting_id() {
    let request = &ChangeUserPasswordRequest {
      profile_id: NONEXISTENT_ID,
      ..user_change_password_request()
    };

    let sut = UserUseCase {
      user_repository: &build_mock_user_repository(Expectations {
        ..Default::default()
      }),
      services: &Services {
        token: token_service_decode_successfully(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.update_password(request, TOKEN).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => assert_eq!(
        error,
        AppError::permission_denied("Given token not have permission for this profile",)
      ),
    }
  }

  #[tokio::test]
  async fn test_change_password_with_expired_token() {
    let request = &ChangeUserPasswordRequest {
      profile_id: ID,
      ..user_change_password_request()
    };

    let sut = UserUseCase {
      user_repository: &build_mock_user_repository(Expectations {
        ..Default::default()
      }),
      services: &Services {
        token: token_service_decode_fail(),
      },
      utilities: &Utilities {
        crypto: MockCrypto::new(),
        id_generator: MockIDGenerator::new(),
      },
    };

    match sut.update_password(request, EXPIRED_TOKEN).await {
      Ok(_) => panic!("should not be success"),
      Err(error) => assert_eq!(
        error,
        AppError::unauthenticated("expired or invalid JWT token",)
      ),
    }
  }
}
