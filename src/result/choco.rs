#[derive(Debug)]
pub enum BloxError {
	DecodeError,
	ApiError,
	NotFound,
	NotImplemented
}

pub type BloxResult<T> = Result<T, BloxError>;