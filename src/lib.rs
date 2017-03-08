extern crate reqwest;
extern crate rustc_serialize;

use std::io::Read;
use error::*;
use response::*;
use request::*;

pub mod error;
pub mod response;
pub mod request;

#[derive(Debug)]
pub struct GitHubClient {
}

impl GitHubClient {
    pub fn new() -> GitHubClient {
        GitHubClient {}
    }

    pub fn send<I: Item, R: GitHubRequest<I=I>>(&self, request: &R) -> Result<SearchResponse<I>, GitHubClientError>{
        let mut response = reqwest::get(request.build_request().as_str())?;
        let mut buf = String::new();
        response.read_to_string(&mut buf).expect("Failed to read response");

        let result = request.response(&buf)?;
        Ok(result)
    }
}
