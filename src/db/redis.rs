use redis::Commands;
use chrono::prelude::*;

pub fn set(issue_number: u32) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let local: DateTime<Local> = Local::now();
    let _ : () = con.set(issue_number, local.to_string())?;
    con.get(issue_number)
}

// TODO: should be String(repos + issue_number)
// in case for conflicting issue numbers among different repositories
pub fn get(issue_number: u32) -> redis::RedisResult<String> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    con.get(issue_number)
}