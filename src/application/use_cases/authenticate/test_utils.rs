use super::*;
use crate::{
  application::services::security::token_service::{MockTokenService, Token},
  domain::{
    core::user::{
      mocks::repository::{
        build_mock_user_repository, Expectations, FindUserBy, Store, UpdatePassword,
      },
      repository::MockUserRepository,
      sign_up::{self, encrypter::PasswordEncrypted},
    },
    entities::user::{UserColumns, UserData},
    error::AppError,
    utilities::{crypto::MockCrypto, id_generator::MockIDGenerator},
  },
};
use chrono::NaiveDate;
use jsonwebtoken::get_current_timestamp;
use mockall::predicate;

pub(super) const ID: &str = "cc3b95d3-4ba4-4a90-bc09-119fd2a4c659";
pub(super) const NAME: &str = "John Doe";
pub(super) const USERNAME: &str = "john.doe";
pub(super) const BIRTH_DATE: &str = "1990-01-01";
pub(super) const GENDER_ID: i32 = 1;
pub(super) const PASSWORD: &str = "123456789";
pub(super) const STREET: &str = "153 W 57th St";
pub(super) const NEIGHBORHOOD: &str = "manhattan";
pub(super) const CITY_ID: i32 = 4;
pub(super) const POSTAL_CODE: i32 = 10019;
pub(super) const EMAIL_ADDRESS: &str = "johndoe@company.com";
pub(super) const TELEPHONE_NUMBER: &str = "+1 4152370800";

pub(super) const NONEXISTENT_ID: &str = "h2hv39f3-9b1s-8f9n-jd92-09df3h9vvd9fg2";
pub(super) const NONEXISTENT_USERNAME: &str = "jane.doe";
pub(super) const WRONG_PASSWORD: &str = "987654321";
pub(super) const NEW_PASSWORD: &str = "1122334455";
pub(super) const HASH_PASSWORD: &str = "mg3824m1htv8913dx6jrn9ui45g801q43tj";
pub(super) const HASH_NEW_PASSWORD: &str = "mg3824m1htv8913dx6jrn9ui45g801q43tj";
pub(super) const TOKEN: &str =
  "eyJ0eXAiJIUzI1NiJ9.eyJzdWIiOiJjNmLTRmODEt.Lmp2I-8YKRucS1zkoCf6dwlI2J4";
pub(super) const EXPIRED_TOKEN: &str = "F1dGhl7bnRpY2F0aW9uX3iOjE2OTgyN.F1dlb6nRpY2F0aW9uX3VzZXIiLCJpYXQi.Lmp2I-PdCEORucS1zBoltEzCf3dwlI2J4";

pub(super) fn user_register_request<'a>() -> UserRegistrationRequest<'a> {
  UserRegistrationRequest {
    name: NAME,
    username: USERNAME,
    birth_date: BIRTH_DATE,
    gender_id: GENDER_ID,
    password: PASSWORD,
    password_repetition: PASSWORD,
    address_street: STREET,
    address_neighborhood: NEIGHBORHOOD,
    address_city_id: CITY_ID,
    address_postal_code: POSTAL_CODE,
    email: EMAIL_ADDRESS,
    telephone: Some(TELEPHONE_NUMBER),
  }
}

pub(super) fn user_change_password_request<'a>() -> ChangeUserPasswordRequest<'a> {
  ChangeUserPasswordRequest {
    profile_id: ID,
    old_password: PASSWORD,
    password: NEW_PASSWORD,
    password_repetition: NEW_PASSWORD,
  }
}

pub(super) fn user_auth_response() -> UserAuthenticationResponse {
  UserAuthenticationResponse {
    id: ID.to_owned(),
    name: NAME.to_owned(),
    token: TOKEN.to_owned(),
    username: USERNAME.to_owned(),
  }
}

pub(super) fn user_data() -> UserData {
  UserData {
    id: ID.to_string(),
    name: NAME.to_string(),
    username: USERNAME.to_string(),
    password: HASH_PASSWORD.to_string(),
  }
}

pub(super) fn repository_find_by_username_successfully() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    find_user_by: Some(FindUserBy {
      calls: 1,
      fn_returning: |_| Ok(user_data()),
      param_column_with: UserColumns::Username(USERNAME),
    }),
    ..Default::default()
  })
}

pub(super) fn repository_find_by_email_successfully() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    find_user_by: Some(FindUserBy {
      calls: 1,
      fn_returning: |_| Ok(user_data()),
      param_column_with: UserColumns::Email(EMAIL_ADDRESS),
    }),
    ..Default::default()
  })
}

pub(super) fn repository_find_by_telephone_successfully() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    find_user_by: Some(FindUserBy {
      calls: 1,
      fn_returning: |_| Ok(user_data()),
      param_column_with: UserColumns::Telephone(TELEPHONE_NUMBER),
    }),
    ..Default::default()
  })
}

pub(super) fn repository_find_by_username_not_found() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    find_user_by: Some(FindUserBy {
      calls: 1,
      fn_returning: |_| Err(AppError::not_found("not found user with given username")),
      param_column_with: UserColumns::Username(NONEXISTENT_USERNAME),
    }),
    ..Default::default()
  })
}

pub(super) fn request_repository_save<'a>() -> sign_up::Request<'a, String, PasswordEncrypted> {
  let password_hash = PasswordEncrypted(HASH_PASSWORD.to_string());
  let birth_date = NaiveDate::parse_from_str(BIRTH_DATE, "%Y-%m-%d").unwrap();

  sign_up::Request {
    id: ID.to_owned(),
    name: NAME,
    username: USERNAME,
    birth_date,
    gender_id: GENDER_ID,
    password: password_hash,
    street: STREET,
    neighborhood: NEIGHBORHOOD,
    city_id: CITY_ID,
    postal_code: POSTAL_CODE,
    email_address: EMAIL_ADDRESS,
    telephone_number: Some(TELEPHONE_NUMBER),
  }
}

pub(super) fn repository_save_successfully() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    store: Some(Store {
      calls: 1,
      param_user_data: request_repository_save(),
      fn_returning: |_| Ok(()),
    }),
    ..Default::default()
  })
}

pub(super) fn repository_save_already_existing() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    store: Some(Store {
      calls: 1,
      param_user_data: request_repository_save(),
      fn_returning: |_| Err(AppError::already_exists("already exists")),
    }),
    ..Default::default()
  })
}

pub(super) fn repository_change_password_successfully() -> MockUserRepository {
  build_mock_user_repository(Expectations {
    update_password: Some(UpdatePassword {
      calls: 1,
      param_password: HASH_NEW_PASSWORD.to_owned(),
      param_profile_id: ID.to_owned(),
      fn_returning: |_, _| Ok(()),
    }),
    find_user_by: Some(FindUserBy {
      calls: 1,
      fn_returning: |_| Ok(user_data()),
      param_column_with: UserColumns::Id(ID),
    }),
    ..Default::default()
  })
}

pub(super) fn token_service_encode() -> MockTokenService {
  let mut token_service_mock = MockTokenService::new();

  token_service_mock
    .expect_encode()
    .times(1)
    .with(
      predicate::eq(ID.to_owned()),
      predicate::eq(String::from("authentication_user")),
      predicate::eq(120),
    )
    .returning(|_, _, _| Ok(TOKEN.to_owned()));

  token_service_mock
}

pub(super) fn token_service_decode_successfully() -> MockTokenService {
  let mut token_service_mock = MockTokenService::new();

  token_service_mock
    .expect_decode()
    .times(1)
    .with(predicate::eq(TOKEN))
    .returning(|_| {
      Ok(Token {
        sub: ID.to_owned(),
        iss: String::from("my_app"),
        aud: String::from("authentication_user"),
        iat: get_current_timestamp() as usize,
        exp: (get_current_timestamp() + 1000 * 60 * 120) as usize,
      })
    });

  token_service_mock
}

pub(super) fn token_service_decode_fail() -> MockTokenService {
  let mut token_service_mock = MockTokenService::new();

  token_service_mock
    .expect_decode()
    .times(1)
    .with(predicate::eq(EXPIRED_TOKEN))
    .returning(|_| Err(AppError::unauthenticated("expired or invalid JWT token")));

  token_service_mock
}

pub(super) fn crypto_verify_successfully() -> MockCrypto {
  let mut crypto = MockCrypto::new();

  crypto
    .expect_verify_password()
    .times(1)
    .with(predicate::eq(HASH_PASSWORD), predicate::eq(PASSWORD))
    .returning(|_, _| Ok(true));

  crypto
}

pub(super) fn crypto_verify_wrong_password() -> MockCrypto {
  let mut crypto = MockCrypto::new();

  crypto
    .expect_verify_password()
    .times(1)
    .with(predicate::eq(HASH_PASSWORD), predicate::eq(WRONG_PASSWORD))
    .returning(|_, _| Ok(false));

  crypto
}

pub(super) fn crypto_hash_successfully() -> MockCrypto {
  let mut crypto = MockCrypto::new();

  crypto
    .expect_hash_password()
    .times(1)
    .with(predicate::eq(PASSWORD))
    .returning(|_| Ok(HASH_PASSWORD.to_owned()));

  crypto
}

pub(super) fn crypto_verify_and_hash_successfully() -> MockCrypto {
  let mut crypto = crypto_verify_successfully();

  crypto
    .expect_hash_password()
    .times(1)
    .with(predicate::eq(NEW_PASSWORD))
    .returning(|_| Ok(HASH_NEW_PASSWORD.to_owned()));

  crypto
}

pub(super) fn generate_id_successfully() -> MockIDGenerator {
  let mut generator = MockIDGenerator::new();

  generator
    .expect_new_uuid()
    .times(1)
    .returning(|| ID.to_owned());

  generator
}
