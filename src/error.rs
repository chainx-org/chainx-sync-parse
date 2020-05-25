pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Msg(String),
    #[error("Fmt error: {0}")]
    Fmt(#[from] std::fmt::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    NetAddrParse(#[from] std::net::AddrParseError),
    #[error("{0}")]
    Send(#[from] std::sync::mpsc::SendError<Vec<u8>>),
    #[error("{0}")]
    Recv(#[from] std::sync::mpsc::RecvError),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Semantic version error: {0}")]
    SemVer(#[from] semver::SemVerError),
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
        match err {
            Error::Msg(msg) => rpc_error(ERROR + 1, msg),
            Error::Fmt(e) => rpc_error(ERROR + 2, e.to_string()),
            Error::Io(e) => rpc_error(ERROR + 3, e.to_string()),
            Error::NetAddrParse(e) => rpc_error(ERROR + 4, e.to_string()),
            Error::Send(e) => rpc_error(ERROR + 5, e.to_string()),
            Error::Recv(e) => rpc_error(ERROR + 6, e.to_string()),
            Error::Reqwest(e) => rpc_error(ERROR + 7, e.to_string()),
            Error::Json(e) => rpc_error(ERROR + 8, e.to_string()),
            Error::SemVer(e) => rpc_error(ERROR + 9, e.to_string()),
        }
    }
}
