use serde::{Deserialize, Serialize};
use std::env;
use std::process;
use reqwest::header;

#[derive(Serialize, Deserialize)]
pub struct Issue {
    pub issue_number: u64,
    pub is_epic: bool,
    pub position: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub issues: Vec<Issue>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub pipelines: Vec<Pipeline>,
}

#[derive(Serialize, Deserialize)]
pub struct SafeResponse {
    pub pipelines: Option<Response>,
}

fn get_token() -> String {
    let token= match env::var(String::from("zenhub_auth_token")) {
        Ok(val) => val,
        Err(_err) => {
            process::exit(1);
        }
    };
    token
}

pub fn get_client() -> Result<reqwest::Client, reqwest::Error> {
    // https://docs.rs/reqwest/0.11.9/reqwest/struct.ClientBuilder.html
    let mut headers = header::HeaderMap::new();
    headers.insert("X-Authentication-Token", header::HeaderValue::from_str(&get_token()).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build();

    match client {
        Ok(val) => Ok(val),
        Err(err) => Err(err),
    }
}