use reqwest::header;
use std::env;
use std::process;

mod platforms;
pub use crate::platforms::zenhub;
pub use crate::platforms::slack;

mod db;
pub use crate::db::redis as my_redis;

mod config;
pub use crate::config::setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let token_key= "zenhub_auth_token";
    let token= match env::var(token_key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, token_key);
            process::exit(1);
        }
    };

    // https://docs.rs/reqwest/0.11.9/reqwest/struct.ClientBuilder.html
    let mut headers = header::HeaderMap::new();
    headers.insert("X-Authentication-Token", header::HeaderValue::from_str(&token).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let c = setup::get("config.json").unwrap();
    println!("{:#?}", c);
    let [v1, v2] = setup::create_zenhub_endpoint(&c.zenhub.workspace_id, &c.github.repositories);
    println!("{:?}", v1);

    for (pos, x) in v1.iter().enumerate() {
        println!("----{}", x);
        println!("####{}", v2[pos]);
        let body = client.get(x).send().await?.text().await?;
        let t: zenhub::Top = serde_json::from_str(&body)?;
        f1(&t.pipelines, &v2[pos], &c.github.organization, &c.zenhub.target_pipeline).await?;
    }

    Ok(())
}

async fn f2(issue_number: u32, v2: &str, o: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", issue_number);
    let result1 = my_redis::get(issue_number);
    match result1 {
        Err(i) => {
            println!("NEW! https://github.com/{}/{}/pull/{}", o, v2, issue_number);
            let m = format!("NEW! https://github.com/{}/{}/pull/{}", o, v2, issue_number);
            slack::notify(&m).await?;
        },
        Ok(i) => (),
    };
    let result = my_redis::set(issue_number);
    println!("{:?}", result.unwrap());
    Ok(())
}

async fn f1(v: &Vec<zenhub::Pipeline>, v2: &str, o: &str, tp: &str) -> Result<(), Box<dyn std::error::Error>> {
    for p in v {
        // TODO: fix this.
        match p.name {
            _ if p.name == tp => {
                for i in &p.issues {
                    f2(i.issue_number, v2, o).await?;
                }
            }
            _ => ()
        }
    }
    Ok(())
}
