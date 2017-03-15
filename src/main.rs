extern crate github_search_repository;
extern crate rustc_serialize;

use github_search_repository::GitHubClient;
use github_search_repository::error::*;
use github_search_repository::request::*;

use std::io::{self, Write};
use std::error::Error;

fn main() {
    print!("Enter your query here> ");
    io::stdout().flush().unwrap();

    let mut keyword = String::new();

    io::stdin()
        .read_line(&mut keyword)
        .expect("Failed to read line");

    let request = SearchRepositories::new(keyword);
    let client = GitHubClient::new();
    match client.send(&request) {
        Ok(repo) => {
            for i in repo.items {
                println!("{}/{}", i.owner.login, i.name);
            }
        }
        Err(GitHubClientError::ResponseParseError(err)) => println!("{}", err.description()),
        Err(GitHubClientError::ConnectionError(err)) => println!("{}", err.description()),
    }
}
