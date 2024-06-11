use crate::config;
use crate::utils;
use reqwest;
use std::error::Error;

pub async fn get_runner_lates_version() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let github_api_url = config::get("GITHUB_API_URL");

    let url = format!("{}/repos/actions/runner/releases/latest", github_api_url);

    println!("Fetching latest github action version...");

    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Actions-Checker/0.1.0")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let version = response["tag_name"]
        .as_str()
        .unwrap()
        .to_string()
        .chars()
        .skip(1)
        .collect::<String>()
        .to_string();

    println!("Latest version found: {:}", version);

    Ok(version)
}

pub async fn create_runner_token() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let github_api_url = config::get("GITHUB_API_URL");
    let github_api_token = config::get("GITHUB_API_TOKEN");
    let org_name = config::get("ORG_NAME");
    let url = format!(
        "{}/orgs/{}/actions/runners/registration-token",
        github_api_url, org_name
    );

    println!("Fetching github action runner token...");

    let response = client
        .post(url)
        .header(reqwest::header::USER_AGENT, "Actions-Checker/0.1.0")
        .bearer_auth(github_api_token)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("response: {:?}", response);

    let token = response["token"].as_str().unwrap().to_string();

    println!("Token generated successfully: {:?}", token);

    Ok(token)
}

pub async fn get_runner_id_by_name(name: String) -> Result<i64, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let github_api_url = config::get("GITHUB_API_URL");
    let github_api_token = config::get("GITHUB_API_TOKEN");
    let org_name = config::get("ORG_NAME");
    let url = format!("{}/orgs/{}/actions/runners", github_api_url, org_name);

    println!("Fetching github action runner...");

    let response = client
        .get(url)
        .header(reqwest::header::USER_AGENT, "Actions-Checker/0.1.0")
        .bearer_auth(github_api_token)
        .query(&[("name", name)])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    // println!("response: {:?}", response);

    let id = response["runners"]
        .get(0)
        .and_then(|runner| runner.get("id").and_then(|id| id.as_i64()))
        .unwrap_or(-1);

    if id.is_positive() {
        println!("github action runner found: {:?}", id);
        return Ok(id);
    }

    println!("github action runner not found");
    return Ok(id);
}

pub async fn delete_runner(name: String) -> Result<(), Box<dyn Error>> {
    let id = get_runner_id_by_name(name.clone()).await?;

    if id.is_negative() {
        return Ok(());
    }

    let client = reqwest::Client::new();
    let github_api_url = config::get("GITHUB_API_URL");
    let github_api_token = config::get("GITHUB_API_TOKEN");
    let org_name = config::get("ORG_NAME");
    let url = format!(
        "{}/orgs/{}/actions/runners/{}",
        github_api_url, org_name, id
    );

    println!("Deleting github action runner...");

    client
        .delete(url)
        .header(reqwest::header::USER_AGENT, "Actions-Checker/0.1.0")
        .bearer_auth(github_api_token)
        .send()
        .await?;

    println!("Runner deleted successfully");

    Ok(())
}

#[allow(dead_code)]
// For practical proposals only
pub async fn _download_runner(version: String) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();

    let github_url = config::get("GITHUB_URL");

    let target_os = config::get("TARGET_OS");

    let target_arch = config::get("TARGET_ARCH");

    let url = format!(
        "{}/actions/runner/releases/download/v{}/actions-runner-{}-{}-{}.tar.gz",
        github_url,
        version.clone(),
        target_os,
        target_arch,
        version.clone()
    );

    println!("Downloading tar file...");

    let response = client
        .get(url.clone())
        .header(reqwest::header::USER_AGENT, "Actions-Checker/0.1.0")
        .header(reqwest::header::CONTENT_TYPE, "application/gzip")
        .send()
        .await?;

    println!("Tar file downloaded");

    let bytes = response.bytes().await.unwrap();

    utils::decompress_tar(bytes)?;

    Ok(())
}
