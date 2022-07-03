use octocrab::Octocrab;
use octocrab::{params, models};
use std::env;

pub async fn get_pull(owner: &str, repo: &str, number: u64) -> octocrab::Result<String> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    let resp = octocrab.pulls(owner, repo).list()
        // Optional Parameters
        // Send the request
        .send()
        .await?;

    for i in resp.items.iter() {
        if i.number != number {
            continue;
        } else {
            let resp = octocrab.pulls(owner, repo).get(number)
                // Optional Parameters
                // Send the request
                .await?;

            return Ok(resp.title.unwrap());
        }
    }

    Ok(String::from("not found"))
}
