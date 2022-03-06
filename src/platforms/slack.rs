use std::collections::HashMap;
// use serde::{Deserialize, Serialize};
// use serde_json::Result as serde_json_Result;
// use reqwest::header;
use std::fs::File;

pub async fn notify(msg: &str) -> Result<(), reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("text", msg);

    // // TODO: Fix this.
    let webhook_url = get_webhook_url();
    println!("{}", webhook_url);
    // let webhook_url = &webhook_url[1..&webhook_url.len()-1];
    // let client = reqwest::Client::new();
    // client.post(webhook_url)
    //     .json(&map)
    //     .send()
    //     .await?;

    Ok(())
}

fn get_webhook_url() -> String {
    let file = File::open("config.json")
        .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
    let slack = json.get("slack")
        .expect("file should have slack key");
    slack["webhook_url"].to_string()
}