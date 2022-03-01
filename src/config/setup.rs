use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::iter::Map;
use url::{Url, ParseError};

#[derive(Serialize, Deserialize, Debug)]
pub struct Zenhub {
    pub workspace_id: String,
    pub target_pipeline: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    pub repository_id: String,
    pub repository_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Github {
    pub organization: String,
    pub repositories: Vec<Repository>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Slack {
    pub webhook_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub zenhub: Zenhub,
    pub github: Github,
    pub slack: Slack,
}

pub struct Endpoint {
    pub github: Github,
    pub zenhub: Zenhub,
    pub endpoint:Result<Url, ParseError>
}

pub fn get<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `Config`.
    let c: Config = serde_json::from_reader(reader)?;

    // Return the `Config`.
    Ok(c)
}

pub fn create_zenhub_endpoint(id: &str, repositories: &Vec<Repository>) -> [Vec<String>; 2] {
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for r in repositories {
        let url1 = format!("https://api.zenhub.com/p2/workspaces/{}/repositories", id);
        let url2 = format!("{}/{}/board", url1, r.repository_id);
        vec1.push(url2);

        let a = format!("{}", r.repository_name);
        vec2.push(a);
    }
    [vec1, vec2]
}