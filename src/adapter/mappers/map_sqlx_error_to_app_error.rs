use crate::domain::error::AppError;
use sqlx::Error;

impl From<Error> for AppError {
  fn from(error: Error) -> Self {
    match error {
      Error::RowNotFound => AppError::not_found("DB: nothing found with given parameters"),
      Error::Database(err) => match err.code().as_deref() {
          Some("23505") => AppError::already_exists("DB: insert or update on table violates unique constraint"),
          Some("23514") => AppError::database_error( "insert or update on table violates check verification"),
          Some("23506") => AppError::database_error( "delete on table violates foreign key constraint"),
          Some("23503") => AppError::database_error( "insert or update on table violates foreign key constraint"),
          Some("23502") => AppError::database_error( "insert or update on table violates not null"),
          Some(code) => AppError::database_error( format!("error code: {}, message: {}", code, err.message())),
          None => AppError::database_error( "error unknown"),
      },
      Error::Configuration(_) => AppError::sql_error( "Error occurred while parsing a connection string"),
      Error::Io(_) => AppError::sql_error( "Error communicating with the database backend"),
      Error::Tls(_) => AppError::sql_error( "Error occurred while attempting to establish a TLS connection"),
      Error::Protocol(_) => AppError::sql_error( "Unexpected or invalid data encountered while communicating with the database"),
      Error::TypeNotFound { type_name: _ } => AppError::sql_error( "Type in query doesn't exist. Likely due to typo or missing user type"),
      Error::ColumnIndexOutOfBounds { index: _, len: _ } => AppError::sql_error( "Column index was out of bounds"),
      Error::ColumnNotFound(_) => AppError::sql_error( "No column found for the given name"),
      Error::ColumnDecode { index: _, source: _ } => AppError::sql_error( "Error occurred while decoding a value from a specific column"),
      Error::Decode(_) => AppError::sql_error( "Error occurred while decoding a value from a specific column"),
      Error::PoolTimedOut => AppError::sql_error( "A [Pool::acquire] timed out due to connections not becoming available or because another task encountered too many errors while trying to open a new connection"),
      Error::PoolClosed => AppError::sql_error( "[Pool::close] was called while we were waiting in [Pool::acquire]"),
      Error::WorkerCrashed => AppError::sql_error( "[Pool::close] was called while we were waiting in [Pool::acquire]"),
      Error::Migrate(_) => AppError::sql_error( "migrate Error"),
      _ => AppError::sql_error( "Error unknown"),
      }
  }
}
