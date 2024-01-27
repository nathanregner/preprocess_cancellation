use crate::py::GCodeError;
use pyo3::exceptions::{PyOSError, PyUnicodeError};
use pyo3::PyErr;
use std::fmt::Debug;
use winnow::error::{ErrMode, ParseError, TreeError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Parse(String),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("GCode already supports cancellation")]
    AlreadySupported,
    #[error("Unclosed object: {0}")]
    UnclosedObject(String),
}

impl<I: Debug> From<ErrMode<TreeError<I>>> for Error {
    fn from(err: ErrMode<TreeError<I>>) -> Self {
        Self::Parse(format!("{}", err))
    }
}

impl<I: Debug> From<ParseError<I, TreeError<I>>> for Error {
    fn from(err: ParseError<I, TreeError<I>>) -> Self {
        Self::Parse(format!("{:?}", err))
    }
}

impl From<Error> for PyErr {
    fn from(value: Error) -> Self {
        match value {
            Error::Parse(message) => GCodeError::new_err(format!("Parse error: {message}")),
            Error::Io(io) => PyOSError::new_err(io),
            Error::Utf8(err) => PyUnicodeError::new_err(err),
            Error::AlreadySupported => GCodeError::new_err("GCode already supports cancellation"),
            Error::UnclosedObject(id) => {
                GCodeError::new_err(format!("Unclosed object detected: {}", id))
            }
        }
    }
}
