use futures::future;
use nanoid::nanoid;
use std::error::Error;
use std::thread;
use tokio::runtime::Runtime;

mod actions;
mod config;
mod containers;
mod database;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = std::env::current_dir()?;
    println!("Current directory: {:?}", dir);
    containers::docker::check_service();
    database::connection::check();
    database::seed::sync_tables();
    run();
    Ok(())
}

pub fn run() {
    let handle = thread::spawn(|| loop {
        let sleep = config::get("SLEEP").parse::<u64>().unwrap();

        println!("Checking github action version...");

        Runtime::new().unwrap().block_on(async {
            let latest_version = actions::fetcher::get_runner_lates_version().await.unwrap();
            let current_version = actions::repository::get_current_version();

            // Validations
            if latest_version == current_version {
                println!(
                    "Currently running the latest version: {}",
                    latest_version.clone()
                );

                return;
            }

            println!("New version found: {}", latest_version.clone());

            // Build new image
            containers::docker::build_image(latest_version.clone()).await;

            // Destroy containers
            destroy_runners().await;

            // return;
            // Create containers
            create_runners().await;

            // Save latest gha runner version
            actions::repository::set_current_version(latest_version.clone());

            println!("Runners updated successfully");
        });

        println!("Sleeping for {} seconds...", sleep);
        thread::sleep(std::time::Duration::from_secs(sleep));
    });

    handle.join().unwrap();
}

pub async fn destroy_runners() {
    // Destroy containers
    let tasks = actions::repository::find_active_runners()
        .into_iter()
        .map(|runner| async move {
            let name = runner.clone().name.unwrap().clone();
            containers::docker::destroy_container(name.clone());
            actions::fetcher::delete_runner(name.clone()).await.unwrap();
        })
        .collect::<Vec<_>>();

    future::join_all(tasks).await;
    actions::repository::delete_runners();

    println!("Runners destroyed successfully");
}

pub async fn create_runners() {
    let runners = config::get("RUNNERS").parse::<i8>().unwrap();

    let tasks = (0..runners).into_iter().map(|_| async move {
        let prefix = config::get("RUNNER_PREFIX");
        let token = actions::fetcher::create_runner_token().await.unwrap();

        let name = if prefix.is_empty() {
            nanoid!()
        } else {
            format!("{}-{}", prefix.clone(), nanoid!())
        };

        let runner = actions::types::Runner {
            id: None,
            name: Some(name),
            token: Some(token.clone()),
        };

        containers::docker::create_container(runner.clone());
        actions::repository::save_runner(runner.clone());
    });

    future::join_all(tasks).await;
    println!("Runners created successfully");
}
