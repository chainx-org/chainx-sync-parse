use std::error::Error as StdError;
use std::fmt;

use failure::{Backtrace, Context, Fail};

pub type Result<T> = ::std::result::Result<T, Error>;

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
    #[cfg(feature = "sync-redis")]
    #[fail(display = "{}", _0)]
    Redis(#[cause] redis::RedisError),
    #[fail(display = "{}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "{}", _0)]
    SerdeJson(#[cause] serde_json::Error),
    #[fail(display = "{}", _0)]
    SemVer(#[cause] semver::SemVerError),
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

#[cfg(feature = "sync-redis")]
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

impl From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Self {
        Error::from(ErrorKind::SemVer(err))
    }
}

const ERROR: i64 = 10000;

fn rpc_error<S: Into<String>>(code: i64, msg: S) -> jsonrpc_core::Error {
    jsonrpc_core::Error {
        code: jsonrpc_core::ErrorCode::ServerError(code),
        message: msg.into(),
        data: None,
    }
}

impl From<Error> for jsonrpc_core::Error {
    fn from(err: Error) -> Self {
        match err.kind() {
            ErrorKind::Msg(msg) => rpc_error(ERROR + 1, msg.clone()),
            ErrorKind::Fmt(e) => rpc_error(ERROR + 2, e.description()),
            ErrorKind::Io(e) => rpc_error(ERROR + 3, e.description()),
            ErrorKind::NetAddrParse(e) => rpc_error(ERROR + 4, e.description()),
            ErrorKind::Send(e) => rpc_error(ERROR + 5, e.description()),
            ErrorKind::Recv(e) => rpc_error(ERROR + 6, e.description()),
            #[cfg(feature = "sync-redis")]
            ErrorKind::Redis(e) => rpc_error(ERROR + 7, e.description()),
            ErrorKind::Reqwest(e) => rpc_error(ERROR + 8, e.description()),
            ErrorKind::SerdeJson(e) => rpc_error(ERROR + 9, e.description()),
            ErrorKind::SemVer(e) => rpc_error(ERROR + 10, e.description()),
        }
    }
}
