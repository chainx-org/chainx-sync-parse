use std::fmt;

use failure::{Backtrace, Context};
use failure_derive::Fail;

#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "{}", _0)]
    Msg(String),
    #[fail(display = "{}", _0)]
    Fmt(#[cause] ::std::fmt::Error),
    #[fail(display = "{}", _0)]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    NetAddrParse(#[cause] ::std::net::AddrParseError),
    #[fail(display = "{}", _0)]
    Send(#[cause] ::std::sync::mpsc::SendError<Vec<u8>>),
    #[fail(display = "{}", _0)]
    Recv(#[cause] ::std::sync::mpsc::RecvError),
    #[fail(display = "{}", _0)]
    Redis(#[cause] redis::RedisError),
    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "{}", _0)]
    SerdeJson(#[cause] serde_json::Error),
}

impl failure::Fail for Error {
    fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    fn cause(&self) -> Option<&failure::Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Self {
        Error { inner }
    }
}

impl From<::std::fmt::Error> for Error {
    fn from(err: ::std::fmt::Error) -> Self {
        Error::from(ErrorKind::Fmt(err))
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::from(ErrorKind::Io(err))
    }
}

impl From<::std::net::AddrParseError> for Error {
    fn from(err: ::std::net::AddrParseError) -> Self {
        Error::from(ErrorKind::NetAddrParse(err))
    }
}

impl From<::std::sync::mpsc::SendError<Vec<u8>>> for Error {
    fn from(err: ::std::sync::mpsc::SendError<Vec<u8>>) -> Self {
        Error::from(ErrorKind::Send(err))
    }
}

impl From<::std::sync::mpsc::RecvError> for Error {
    fn from(err: ::std::sync::mpsc::RecvError) -> Self {
        Error::from(ErrorKind::Recv(err))
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::from(ErrorKind::Redis(err))
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::from(ErrorKind::Reqwest(err))
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::from(ErrorKind::SerdeJson(err))
    }
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::from(ErrorKind::Msg(s.into()))
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::from(ErrorKind::Msg(s))
    }
}

pub type Result<T> = ::std::result::Result<T, Error>;
