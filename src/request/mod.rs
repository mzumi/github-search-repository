use super::error::*;
use super::response::*;
use super::rustc_serialize::json;
use std::collections::HashMap;

#[derive(Debug)]
pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

pub trait GitHubRequest {
    type I: Item;
    fn base_url(&self) -> String {
        "https://api.github.com".to_owned()
    }

    fn path(&self) -> String;
    fn method(&self) -> HTTPMethod;
    fn parameters(&self) -> HashMap<String, &String>;
    fn response(&self, buf: &String) -> Result<SearchResponse<Self::I>, GitHubClientError>;

    fn build_request(&self) -> String {
        let params = match self.method() {
            HTTPMethod::GET => {
                Some(self.parameters()
                    .iter()
                    .map(|(key, value)| format!("{}={}", key, value))
                    .collect::<Vec<String>>())
            }
            _ => None,
        };

        format!("{}{}?{}",
                self.base_url(),
                self.path(),
                params.map_or("".to_owned(), |p| p.join("&")))
    }
}

pub struct SearchRepositories {
    keyword: String,
}

impl SearchRepositories {
    pub fn new(keyword: String) -> SearchRepositories {
        SearchRepositories { keyword: keyword }
    }
}

impl GitHubRequest for SearchRepositories {
    type I = Repository;
    fn path(&self) -> String {
        "/search/repositories".to_owned()
    }

    fn method(&self) -> HTTPMethod {
        HTTPMethod::GET
    }

    fn parameters(&self) -> HashMap<String, &String> {
        [("q".to_owned(), &self.keyword)].iter().cloned().collect()
    }


    fn response(&self, buf: &String) -> Result<SearchResponse<Self::I>, GitHubClientError> {
        json::decode(&buf).map_err(|e| From::from(e))
    }
}


pub struct SearchUsers {
    keyword: String,
}

impl SearchUsers {
    pub fn new(keyword: String) -> SearchUsers {
        SearchUsers { keyword: keyword }
    }
}

impl GitHubRequest for SearchUsers {
    type I = User;
    fn path(&self) -> String {
        "/search/users".to_owned()
    }

    fn method(&self) -> HTTPMethod {
        HTTPMethod::GET
    }

    fn parameters(&self) -> HashMap<String, &String> {
        [("q".to_owned(), &self.keyword)].iter().cloned().collect()
    }

    fn response(&self, buf: &String) -> Result<SearchResponse<Self::I>, GitHubClientError> {
        json::decode(&buf).map_err(|e| From::from(e))
    }
}