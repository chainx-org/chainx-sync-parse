use failure::Fail;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Msg(String),
    #[fail(display = "Fmt error: {}", _0)]
    Fmt(#[cause] ::std::fmt::Error),
    #[fail(display = "Io error: {}", _0)]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "{}", _0)]
    NetAddrParse(#[cause] ::std::net::AddrParseError),
    #[fail(display = "{}", _0)]
    Send(#[cause] ::std::sync::mpsc::SendError<Vec<u8>>),
    #[fail(display = "{}", _0)]
    Recv(#[cause] ::std::sync::mpsc::RecvError),
    #[cfg(feature = "sync-redis")]
    #[fail(display = "Redis error: {}", _0)]
    Redis(#[cause] redis::RedisError),
    #[fail(display = "Reqwest error: {}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "Json error: {}", _0)]
    Json(#[cause] serde_json::Error),
    #[fail(display = "Semantic version error: {}", _0)]
    SemVer(#[cause] semver::SemVerError),
}

impl<'a> From<&'a str> for Error {
    fn from(s: &'a str) -> Self {
        Error::Msg(s.into())
    }
}
impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Msg(s)
    }
}

impl From<::std::fmt::Error> for Error {
    fn from(err: ::std::fmt::Error) -> Self {
        Error::Fmt(err)
    }
}

impl From<::std::io::Error> for Error {
    fn from(err: ::std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<::std::net::AddrParseError> for Error {
    fn from(err: ::std::net::AddrParseError) -> Self {
        Error::NetAddrParse(err)
    }
}

impl From<::std::sync::mpsc::SendError<Vec<u8>>> for Error {
    fn from(err: ::std::sync::mpsc::SendError<Vec<u8>>) -> Self {
        Error::Send(err)
    }
}

impl From<::std::sync::mpsc::RecvError> for Error {
    fn from(err: ::std::sync::mpsc::RecvError) -> Self {
        Error::Recv(err)
    }
}

#[cfg(feature = "sync-redis")]
impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        Error::Redis(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Self {
        Error::SemVer(err)
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
        use std::error::Error as StdError;
        match err {
            Error::Msg(msg) => rpc_error(ERROR + 1, msg),
            Error::Fmt(e) => rpc_error(ERROR + 2, e.description()),
            Error::Io(e) => rpc_error(ERROR + 3, e.description()),
            Error::NetAddrParse(e) => rpc_error(ERROR + 4, e.description()),
            Error::Send(e) => rpc_error(ERROR + 5, e.description()),
            Error::Recv(e) => rpc_error(ERROR + 6, e.description()),
            #[cfg(feature = "sync-redis")]
            Error::Redis(e) => rpc_error(ERROR + 7, e.description()),
            Error::Reqwest(e) => rpc_error(ERROR + 8, e.description()),
            Error::Json(e) => rpc_error(ERROR + 9, e.description()),
            Error::SemVer(e) => rpc_error(ERROR + 10, e.description()),
        }
    }
}
