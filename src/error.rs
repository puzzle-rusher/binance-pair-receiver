use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub(crate) enum Error {
    ParseUrl(String),
    Io(String),
    Request(reqwest::Error),
    UnknownResponse(String),
    ResponseParsingError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}

impl From<tokio::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value.to_string())
    }
}
