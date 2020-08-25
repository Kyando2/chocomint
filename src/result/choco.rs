#[derive(Debug)]
pub enum BloxError {
	DecodeError,
	ApiError,
	NotFound,
	NotImplemented
}

/// Shortcut for Result<T, BloxError>
pub type BloxResult<T> = Result<T, BloxError>;