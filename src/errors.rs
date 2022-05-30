use serde_json;
use reqwest;


pub type EGResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    Parse(serde_json::Error),
}


impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Network(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Parse(e)
    }
}