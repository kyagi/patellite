use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Error, Result};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Issue {
    pub issue_number: u32,
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
pub struct Top {
    pub pipelines: Vec<Pipeline>,
}

#[derive(Serialize, Deserialize)]
pub struct SafeTop {
    pub pipelines: Option<Top>,
}