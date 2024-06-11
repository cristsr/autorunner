use crate::config;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Lts {
    Bool(bool),
    String(String),
}

#[derive(Deserialize, Debug)]
struct NodeVersion {
    version: String,
    lts: Option<Lts>,
}

pub async fn get_latest_node_lts() -> Result<String, reqwest::Error> {
    let node_dist_url = config::get("NODE_DIST_URL");

    let response = reqwest::get(node_dist_url)
        .await?
        .json::<Vec<NodeVersion>>()
        .await?;

    let version = response
        .into_iter()
        .find(|v| matches!(v.lts, Some(Lts::String(_))))
        .map(|v| v.version.trim_start_matches('v').to_string())
        .unwrap();

    println!("Latest node LTS version found: {}", version.clone());

    Ok(version)
}
