use std::fmt::Debug;
use winnow::error::{ContextError, ErrMode, ParseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
}

impl From<ErrMode<ContextError>> for Error {
    fn from(err: ErrMode<ContextError>) -> Self {
        Self::Parse(format!("{}", err))
    }
}

impl<I: Debug> From<ParseError<I, ContextError<I>>> for Error {
    fn from(err: ParseError<I, ContextError<I>>) -> Self {
        Self::Parse(format!("{:?}", err))
    }
}
