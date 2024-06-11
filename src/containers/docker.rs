use crate::{actions, config, containers};
use std::process;
use std::process::Stdio;

pub async fn build_image(runner_version: String) {
    let target_arch = config::get("TARGET_ARCH");
    let node_version = containers::fetcher::get_latest_node_lts().await.unwrap();
    let mut dir = std::env::current_dir().unwrap();
    dir.push("docker");
    dir.push("Dockerfile");

    println!("Building docker image...");
    println!("Runner version: {}", runner_version.clone());
    println!("Target arch: {}", target_arch.clone());
    println!("Node version: {}", node_version.clone());

    let output = process::Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg("runner")
        .arg("-f")
        .arg(dir.to_str().unwrap())
        .arg("--build-arg")
        .arg(format!("TARGET_ARCH={}", target_arch))
        .arg("--build-arg")
        .arg(format!("NODE_VERSION={}", node_version))
        .arg("--build-arg")
        .arg(format!("RUNNER_VERSION={}", runner_version))
        .arg(".")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    if !output.status.success() {
        println!("Error running docker build!");
        process::exit(1);
    }

    println!("Docker build successful");
}

pub fn check_service() {
    println!("Checking docker service...");

    let output = process::Command::new("docker")
        .arg("ps")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    if !output.status.success() {
        println!("Docker service is not running!");
        process::exit(1);
    }

    println!("Docker service is running!");
}

pub fn create_container(data: actions::types::Runner) {
    println!("Creating container...",);

    let github_url = config::get("GITHUB_URL");
    let org_name = config::get("ORG_NAME");
    let org_url = format!("{}/{}", github_url, org_name);
    let reg_token = data.clone().token.unwrap().clone();
    let runner_name = data.clone().name.unwrap().clone();

    println!("ORG_URL: {}", org_url.clone());
    println!("REG_TOKEN: {}", reg_token.clone());
    println!("RUNNER_NAME: {}", runner_name.clone());

    let output = process::Command::new("docker")
        .arg("run")
        .arg("-dit")
        .arg("--name")
        .arg(runner_name.clone())
        .arg("-e")
        .arg(format!("ORG_URL={}", org_url))
        .arg("-e")
        .arg(format!("REG_TOKEN={}", reg_token))
        .arg("-e")
        .arg(format!("RUNNER_NAME={}", runner_name.clone()))
        .arg("runner")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    if !output.status.success() {
        println!("Error creating container {}", runner_name.clone());
        return ();
    }

    println!("Container created: {}", runner_name.clone());
}

pub fn destroy_container(name: String) {
    println!("Destroying container: {}", name);

    let output = process::Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(name.clone())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();

    if !output.status.success() {
        println!("Error destroying container: {}", name);
        return ();
    }

    println!("Container destroyed: {}", name);
}
