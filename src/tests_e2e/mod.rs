pub mod helpers;
use crate::{
  adapter::routers::v1::auth::dtos::UserAuthenticationResponseHttp,
  application::services::security::token_service::TokenService,
  domain::entities::user::{UserColumns, UserData},
  infra::config::{
    cors::default_cors,
    routes::routes_config,
    services::{get_jwt_service, services_config},
    utilities::utilities_config,
  },
  tests_e2e::helpers::{
    jwt::{assert_jwt, TokenExpirationExpect},
    user_repository::{assert_user, insert_user},
  },
  AppState,
};
use actix_web::{
  http::header::{ContentType, HeaderValue, AUTHORIZATION},
  test::{self},
  web, App,
};
use serde_json::json;
use sqlx::{PgPool, Result};
use uuid::Uuid;

const TWO_HOURS: u64 = 1000 * 60 * 120;
const NAME: &str = "John Doe";
const USERNAME: &str = "john.doe";
const BIRTH_DATE: &str = "1990-01-01";
const GENDER_ID: i32 = 1;
const PASSWORD: &str = "123456789";
const NEW_PASSWORD: &str = "11223344555";
const WRONG_PASSWORD: &str = "987654321";
const STREET: &str = "153 W 57th St";
const NEIGHBORHOOD: &str = "manhattan";
const CITY_ID: i32 = 4;
const POSTAL_CODE: i32 = 10019;
const EMAIL_ADDRESS: &str = "johndoe@company.com";
const TELEPHONE_NUMBER: &str = "+1 4152370800 ";

#[sqlx::test]
async fn test_get_index(pool: PgPool) -> Result<()> {
  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::get().uri("/").to_request();
  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 200);
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_register(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool.clone(),
    RequestRegisterDefault {
      ..Default::default()
    },
  )
  .await;
  assert_eq!(res.status(), 201);

  let body: UserAuthenticationResponseHttp = test::read_body_json(res).await;

  assert_eq!(body.name, NAME);
  assert_eq!(body.username, USERNAME);
  assert_jwt(
    &body.token,
    &body.id,
    "authentication_user",
    TokenExpirationExpect::GreatThan(TWO_HOURS - 1),
  );
  assert_user(
    &pool,
    &UserColumns::Id(&body.id),
    UserData {
      id: body.id.clone(),
      name: NAME.to_string(),
      username: USERNAME.to_string(),
      password: PASSWORD.to_string(),
    },
    Some(|pwd, hash| bcrypt::verify(pwd, hash).unwrap()),
  )
  .await;
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_duplicate_register(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  insert_user(
    &pool,
    &Uuid::new_v4().to_string(),
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 409);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_username(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      username: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_name(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      name: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_birth_date(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      birth_date: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_gender_id_0(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      gender_id: 0,
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_password(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      password: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_repetition_password(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      password_repetition: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_street(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      address_street: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_neighborhood(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      address_neighborhood: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_city_id_0(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      address_city_id: 0,
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_postal_code_0(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      address_postal_code: 0,
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_email(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      email: "",
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_telephone(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      telephone: Some(""),
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_register_with_empty_not_match_password(pool: PgPool) -> Result<()> {
  let res = test_register_with_default(
    pool,
    RequestRegisterDefault {
      password_repetition: WRONG_PASSWORD,
      ..Default::default()
    },
  )
  .await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_sign_with_username(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  insert_user(
    &pool,
    &Uuid::new_v4().to_string(),
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::post()
    .uri("/v1/auth/sign_in")
    .insert_header(ContentType::json())
    .set_payload(
      json!({
        "username": USERNAME,
        "password": PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 200);

  let body: UserAuthenticationResponseHttp = test::read_body_json(res).await;

  assert_eq!(body.name, NAME);
  assert_eq!(body.username, USERNAME);
  assert_jwt(
    &body.token,
    &body.id,
    "authentication_user",
    TokenExpirationExpect::GreatThan(TWO_HOURS - 1),
  );
  assert_user(
    &pool,
    &UserColumns::Id(&body.id),
    UserData {
      id: body.id.clone(),
      name: NAME.to_string(),
      username: USERNAME.to_string(),
      password: PASSWORD.to_string(),
    },
    Some(|pwd, hash| bcrypt::verify(pwd, hash).unwrap()),
  )
  .await;
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_sign_with_email(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  insert_user(
    &pool,
    &Uuid::new_v4().to_string(),
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::post()
    .uri("/v1/auth/sign_in")
    .insert_header(ContentType::json())
    .set_payload(
      json!({
        "email": EMAIL_ADDRESS,
        "password": PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 200);

  let body: UserAuthenticationResponseHttp = test::read_body_json(res).await;

  assert_eq!(body.name, NAME);
  assert_eq!(body.username, USERNAME);
  assert_jwt(
    &body.token,
    &body.id,
    "authentication_user",
    TokenExpirationExpect::GreatThan(TWO_HOURS - 1),
  );
  assert_user(
    &pool,
    &UserColumns::Id(&body.id),
    UserData {
      id: body.id.clone(),
      name: NAME.to_string(),
      username: USERNAME.to_string(),
      password: PASSWORD.to_string(),
    },
    Some(|pwd, hash| bcrypt::verify(pwd, hash).unwrap()),
  )
  .await;
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_sign_with_telephone(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  insert_user(
    &pool,
    &Uuid::new_v4().to_string(),
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::post()
    .uri("/v1/auth/sign_in")
    .insert_header(ContentType::json())
    .set_payload(
      json!({
        "telephone": TELEPHONE_NUMBER,
        "password": PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 200);

  let body: UserAuthenticationResponseHttp = test::read_body_json(res).await;

  assert_eq!(body.name, NAME);
  assert_eq!(body.username, USERNAME);
  assert_jwt(
    &body.token,
    &body.id,
    "authentication_user",
    TokenExpirationExpect::GreatThan(TWO_HOURS - 1),
  );
  assert_user(
    &pool,
    &UserColumns::Id(&body.id),
    UserData {
      id: body.id.clone(),
      name: NAME.to_string(),
      username: USERNAME.to_string(),
      password: PASSWORD.to_string(),
    },
    Some(|pwd, hash| bcrypt::verify(pwd, hash).unwrap()),
  )
  .await;
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_sign_with_username_given_wrong_password(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  insert_user(
    &pool,
    &Uuid::new_v4().to_string(),
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::post()
    .uri("/v1/auth/sign_in")
    .insert_header(ContentType::json())
    .set_payload(
      json!({
        "username": USERNAME,
        "password": WRONG_PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 401);
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_change_password_successfully(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  let id = Uuid::new_v4().to_string();
  insert_user(
    &pool,
    &id,
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let token = get_jwt_service()
    .encode(id.clone(), "authentication_user".to_string(), 10)
    .unwrap();

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::put()
    .uri("/v1/auth/change_password")
    .insert_header(ContentType::json())
    .append_header((
      AUTHORIZATION,
      HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    ))
    .set_payload(
      json!({
        "profile_id": id,
        "old_password": PASSWORD,
        "password": NEW_PASSWORD,
        "password_repetition": NEW_PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 200);
  Ok(())
}

#[sqlx::test(fixtures("countries", "states", "cities", "genders"))]
async fn test_change_password_given_old_wrong_password(pool: PgPool) -> Result<()> {
  let hash_pwd = bcrypt::hash(PASSWORD, 5).unwrap();
  let id = Uuid::new_v4().to_string();
  insert_user(
    &pool,
    &id,
    NAME,
    USERNAME,
    BIRTH_DATE,
    GENDER_ID,
    &hash_pwd,
    STREET,
    NEIGHBORHOOD,
    CITY_ID,
    POSTAL_CODE,
    EMAIL_ADDRESS,
    Some(TELEPHONE_NUMBER),
  )
  .await;

  let token = get_jwt_service()
    .encode(id.clone(), "authentication_user".to_string(), 10)
    .unwrap();

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::put()
    .uri("/v1/auth/change_password")
    .insert_header(ContentType::json())
    .append_header((
      AUTHORIZATION,
      HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    ))
    .set_payload(
      json!({
        "profile_id": id,
        "old_password": WRONG_PASSWORD,
        "password": NEW_PASSWORD,
        "password_repetition": NEW_PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 403);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_change_password_given_not_match_new_password(pool: PgPool) -> Result<()> {
  let id = Uuid::new_v4().to_string();

  let token = get_jwt_service()
    .encode(id.clone(), "authentication_user".to_string(), 10)
    .unwrap();

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::put()
    .uri("/v1/auth/change_password")
    .insert_header(ContentType::json())
    .append_header((
      AUTHORIZATION,
      HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    ))
    .set_payload(
      json!({
        "profile_id": id,
        "old_password": PASSWORD,
        "password": NEW_PASSWORD,
        "password_repetition": WRONG_PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 400);
  Ok(())
}

#[sqlx::test(migrations = false)]
async fn test_change_password_given_wrong_token_for_profile_id(pool: PgPool) -> Result<()> {
  let id = Uuid::new_v4().to_string();
  let other_id = Uuid::new_v4().to_string();

  let token = get_jwt_service()
    .encode(id.clone(), "authentication_user".to_string(), 10)
    .unwrap();

  let authorization = format!("Bearer {}", token);

  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::put()
    .uri("/v1/auth/change_password")
    .insert_header(ContentType::json())
    .append_header((
      AUTHORIZATION,
      HeaderValue::from_str(&authorization).unwrap(),
    ))
    .set_payload(
      json!({
        "profile_id": other_id,
        "old_password": PASSWORD,
        "password": NEW_PASSWORD,
        "password_repetition": NEW_PASSWORD,
      })
      .to_string(),
    )
    .to_request();

  let res = test::call_service(&mut app, req).await;

  assert_eq!(res.status(), 403);
  Ok(())
}

struct RequestRegisterDefault<'a> {
  name: &'a str,
  username: &'a str,
  birth_date: &'a str,
  gender_id: i32,
  password: &'a str,
  password_repetition: &'a str,
  address_street: &'a str,
  address_neighborhood: &'a str,
  address_city_id: i32,
  address_postal_code: i32,
  email: &'a str,
  telephone: Option<&'a str>,
}

impl Default for RequestRegisterDefault<'_> {
  fn default() -> Self {
    RequestRegisterDefault {
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
}

async fn test_register_with_default(
  pool: PgPool,
  request: RequestRegisterDefault<'_>,
) -> actix_web::dev::ServiceResponse<actix_web::body::EitherBody<actix_web::body::BoxBody>> {
  let mut app = test::init_service(
    App::new()
      .wrap(default_cors())
      .app_data(web::Data::new(AppState {
        postgres_pool: pool.clone(),
      }))
      .configure(services_config)
      .configure(utilities_config)
      .configure(routes_config),
  )
  .await;

  let req = test::TestRequest::post()
    .uri("/v1/auth/register")
    .insert_header(ContentType::json())
    .set_payload(
      json!({
        "username": request.username,
        "name": request.name,
        "gender_id": request.gender_id,
        "birth_date": request.birth_date,
        "password": request.password,
        "password_repetition": request.password_repetition,
        "address_street": request.address_street,
        "address_neighborhood": request.address_neighborhood,
        "address_city_id": request.address_city_id,
        "address_postal_code": request.address_postal_code,
        "email": request.email,
        "telephone": request.telephone
      })
      .to_string(),
    )
    .to_request();

  test::call_service(&mut app, req).await
}
