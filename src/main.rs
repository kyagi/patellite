mod platforms;
pub use crate::platforms::zenhub;
pub use crate::platforms::slack;

mod db;
pub use crate::db::redis as my_redis;

mod config;
pub use crate::config::setup;
use crate::platforms::github;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = zenhub::get_client().unwrap();
    let c = setup::get("config.json").unwrap();

    let endpoints = setup::get_endpoints(&c).unwrap();
    for e in endpoints.iter() {
        // impl InfoUrl for String
        // https://docs.rs/reqwest/latest/reqwest/trait.IntoUrl.html
        println!("{}", e.url.as_ref().unwrap());
        let body = client.get(e.url.as_ref().unwrap().as_str()).send().await?.text().await?;
        let body_json: zenhub::Response = serde_json::from_str(&body)?;

        do_only_if_specific_pipeline(&body_json.pipelines,&e.repository.repository_name, &c.github.organization, &c.zenhub.target_pipeline).await?;
    }

    Ok(())
}

async fn do_only_if_specific_pipeline(pipelines: &Vec<zenhub::Pipeline>, github_repository_name: &str,  github_organization: &str, target_pipeline: &str) -> Result<(), Box<dyn std::error::Error>> {
    for p in pipelines {
        // TODO: fix this.
        match p.name {
            _ if p.name == target_pipeline => {
                for i in &p.issues {
                    check_and_notify(i.issue_number, github_repository_name, github_organization).await?;
                }
            }
            _ => ()
        }
    }
    Ok(())
}

async fn check_and_notify(issue_number: u64, github_repository_name: &str, github_organization: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", issue_number);
    let result1 = my_redis::get(issue_number);
    match result1 {
        Err(_i) => {
            let title = github::get_pull(github_organization, github_repository_name, issue_number).await?;
            println!(":new: <https://github.com/{}/{}/pull/{}|{}>", github_organization, github_repository_name, issue_number, title);
            let m = format!(":new: <https://github.com/{}/{}/pull/{}|{}>", github_organization, github_repository_name, issue_number, title);
            slack::notify(&m).await?;
            let _result = my_redis::set(issue_number);
        },
        Ok(_i) => (),
    };
    Ok(())
}

