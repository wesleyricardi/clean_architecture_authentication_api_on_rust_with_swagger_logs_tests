use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
pub enum Code {
  Unknown,
  InvalidArgument,
  NotFound,
  AlreadyExists,
  PermissionDenied,
  Internal,
  Unauthenticated,
  DatabaseError,
  SQLError,
  OIError,
}

#[derive(Debug, PartialEq)]
pub struct ErrorSource(pub String);

impl fmt::Display for ErrorSource {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for ErrorSource {}

#[derive(Debug, PartialEq)]
pub struct AppError {
  pub code: Code,
  pub message: String,
  pub details: Option<String>,
  pub source: Option<ErrorSource>,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

impl Error for AppError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    if let Some(source) = &self.source {
      Some(source)
    } else {
      None
    }
  }

  fn description(&self) -> &str {
    if let Some(details) = &self.details {
      return details;
    }

    &self.message
  }

  fn cause(&self) -> Option<&dyn Error> {
    self.source()
  }
}

impl AppError {
  pub fn new(
    code: Code,
    message: impl Into<String>,
    details: Option<String>,
    source: Option<ErrorSource>,
  ) -> AppError {
    AppError {
      code,
      message: message.into(),
      details,
      source,
    }
  }
  pub fn unknown(message: impl Into<String>) -> AppError {
    Self {
      code: Code::Unknown,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn invalid_argument(message: impl Into<String>) -> AppError {
    Self {
      code: Code::InvalidArgument,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn not_found(message: impl Into<String>) -> AppError {
    Self {
      code: Code::NotFound,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn already_exists(message: impl Into<String>) -> AppError {
    Self {
      code: Code::AlreadyExists,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn permission_denied(message: impl Into<String>) -> AppError {
    Self {
      code: Code::PermissionDenied,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn internal(message: impl Into<String>) -> AppError {
    Self {
      code: Code::Internal,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn unauthenticated(message: impl Into<String>) -> AppError {
    Self {
      code: Code::Unauthenticated,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn database_error(message: impl Into<String>) -> AppError {
    Self {
      code: Code::DatabaseError,
      message: message.into(),
      details: None,
      source: None,
    }
  }
  pub fn sql_error(message: impl Into<String>) -> AppError {
    Self {
      code: Code::SQLError,
      message: message.into(),
      details: None,
      source: None,
    }
  }

  pub fn io_error(message: impl Into<String>) -> AppError {
    Self {
      code: Code::OIError,
      message: message.into(),
      details: None,
      source: None,
    }
  }
}
