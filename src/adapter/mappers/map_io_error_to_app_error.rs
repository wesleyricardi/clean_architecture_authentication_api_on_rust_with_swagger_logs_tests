use crate::domain::error::AppError;

impl From<std::io::Error> for AppError {
  fn from(error: std::io::Error) -> Self {
    match error.kind() {
      std::io::ErrorKind::NotFound => AppError::io_error("An entity was not found, often a file."),
      std::io::ErrorKind::PermissionDenied => AppError::io_error("The operation lacked the necessary privileges to complete."),
      std::io::ErrorKind::ConnectionRefused => AppError::io_error("The connection was refused by the remote server."),
      std::io::ErrorKind::ConnectionReset => AppError::io_error("The connection was aborted (terminated) by the remote server"),
      std::io::ErrorKind::ConnectionAborted => AppError::io_error("The connection was aborted (terminated) by the remote server."),
      std::io::ErrorKind::NotConnected => AppError::io_error("The network operation failed because it was not connected yet."),
      std::io::ErrorKind::AddrInUse => AppError::io_error("A socket address could not be bound because the address is already in use elsewhere."),
      std::io::ErrorKind::AddrNotAvailable => AppError::io_error("A nonexistent interface was requested or the requested address was not local."),
      std::io::ErrorKind::BrokenPipe => AppError::io_error("The operation failed because a pipe was closed."),
      std::io::ErrorKind::AlreadyExists => AppError::io_error("An entity already exists, often a file."),
      std::io::ErrorKind::WouldBlock => AppError::io_error("The operation needs to block to complete, but the blocking operation was requested to not occur."),
      std::io::ErrorKind::InvalidInput => AppError::io_error("A parameter was incorrect."),
      std::io::ErrorKind::InvalidData => AppError::io_error("Data not valid for the operation were encountered."),
      std::io::ErrorKind::TimedOut => AppError::io_error("The I/O operation's timeout expired, causing it to be canceled."),
      std::io::ErrorKind::WriteZero => AppError::io_error("An error returned when an operation could not be completed because a call to [write] returned [Ok(0)]."),
      _ => AppError::io_error("Another IO error"),
    }
  }
}
