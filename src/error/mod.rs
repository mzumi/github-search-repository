use rustc_serialize::json;
use reqwest;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum GitHubClientError {
    ResponseParseError(json::DecoderError),
    ConnectionError(reqwest::Error),
}

impl fmt::Display for GitHubClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GitHubClientError::ResponseParseError(ref err) => err.fmt(f),
            GitHubClientError::ConnectionError(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for GitHubClientError {
    fn description(&self) -> &str {
        match *self {
            GitHubClientError::ResponseParseError(ref err) => err.description(),
            GitHubClientError::ConnectionError(ref err) => err.description(),
        }
    }
}

impl From<json::DecoderError> for GitHubClientError {
    fn from(err: json::DecoderError) -> GitHubClientError {
        GitHubClientError::ResponseParseError(err)
    }
}

impl From<reqwest::Error> for GitHubClientError {
    fn from(err: reqwest::Error) -> GitHubClientError {
        GitHubClientError::ConnectionError(err)
    }
}