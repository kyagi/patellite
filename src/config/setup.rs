use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
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

// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-annotations-in-struct-definitions
pub struct Endpoint<'a> {
    pub github: &'a Github,
    pub repository: &'a Repository,
    pub zenhub: &'a Zenhub,
    pub url: Result<Url, ParseError>
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

pub fn get_endpoints (c: &Config) -> Result<Vec<Endpoint>, Box<dyn Error>> {
    let mut endpoints: Vec<Endpoint> = Vec::new();
    for r in &c.github.repositories {
        let url = format!("https://api.zenhub.com/p2/workspaces/{}/repositories", &c.zenhub.workspace_id);
        let url2 = format!("{}/{}/board", url, &r.repository_id);
        let url3: Result<Url, ParseError> = Url::parse(&url2);
        endpoints.push(Endpoint { repository: r, github: &c.github, zenhub: &c.zenhub, url: url3 });
    }

    // Return the `Endpoint`.
    Ok(endpoints)
}
