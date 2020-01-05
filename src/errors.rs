use failure::{Backtrace, Context, Fail};
use serde_json::Error as SerdeError;
use std::env::VarError;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn new(inner: Context<ErrorKind>) -> Self {
        Error { inner }
    }

    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "std::env::var error")]
    EnvVar(VarError),

    #[fail(display = "serde error")]
    Serde(SerdeError),

    #[fail(display = "not found from English-Japanese Dictionary: {}", en)]
    NotFound { en: String },
}

impl From<VarError> for ErrorKind {
    fn from(err: VarError) -> Self {
        ErrorKind::EnvVar(err)
    }
}

impl From<SerdeError> for ErrorKind {
    fn from(err: SerdeError) -> Self {
        ErrorKind::Serde(err)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(ctx: Context<ErrorKind>) -> Error {
        Error { inner: ctx }
    }
}

impl From<VarError> for Error {
    fn from(err: VarError) -> Self {
        let kind = ErrorKind::from(err);
        Error::from(kind)
    }
}

impl From<SerdeError> for Error {
    fn from(err: SerdeError) -> Self {
        let kind = ErrorKind::from(err);
        Error::from(kind)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
