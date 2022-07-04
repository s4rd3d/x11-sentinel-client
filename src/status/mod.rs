use notify_rust::Notification;
use reqwest::Error;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

use crate::config;

#[derive(Deserialize)]
pub struct Status {
    pub phase: String,
    pub description: String,
    pub value: f32,
}

/// Send a HTTP GET request to query the status of the client.
#[tokio::main]
async fn get_status(status_url: &String) -> Result<Status, Error> {
    let response = reqwest::get(status_url).await?;
    let status: Status = response.json().await?;

    Ok(status)
}

pub fn run(config: config::Config) -> () {
    let status_url = config.status_url.unwrap();
    let status_interval = config.status_interval.unwrap();

    loop {
        // Get status from the remote server.
        let status = match get_status(&status_url) {
            Ok(status) => status,
            Err(error) => panic!("Could not get status: {}", error),
        };

        // Notify the user.
        match Notification::new()
            .appname("X11 Sentinel Client")
            .summary("X11 Sentinel Client Status Update")
            .body(&format!(
                "Phase: {}, Description: {}, value: {}",
                status.phase, status.description, status.value
            ))
            .show()
        {
            Ok(_handle) => (),
            Err(error) => println!("Could not show notification: {}", error),
        };

        // Sleep for a configured amount of time.
        thread::sleep(Duration::from_secs(status_interval));
    }
}
