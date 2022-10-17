use notify_rust::Notification;
use reqwest::Error;
use serde::Deserialize;
use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::config;

//==============================================================================
// Structs
//==============================================================================

#[derive(Deserialize)]
pub struct Status {
    pub phase: String,
    pub description: String,
    pub value: f64,
}

//==============================================================================
// Public functions
//==============================================================================

pub fn run(config: config::Config, stream_id: String) -> () {
    let status_base_url = config.status_base_url.unwrap();
    let status_interval = config.status_interval.unwrap();
    let lock_utility = config.lock_utility.unwrap();
    let lock_enabled = config.lock_enabled.unwrap();
    let lock_threshold = config.lock_threshold.unwrap();
    let user_id = config.user_id.unwrap();

    let status_url = format!("{}/{}/{}", status_base_url, user_id, stream_id);

    loop {
        // Get status from the remote server.
        let status = match get_status(&status_url) {
            Ok(status) => status,
            Err(error) => panic!("Could not get status: {}", error),
        };

        // If session locking is enabled and the user's score is lower than a
        // predefined constant lock the X session by executing the lock utility
        // program.
        if lock_enabled &&
           status.phase == "verify" &&
           status.value < lock_threshold {
            Command::new(&lock_utility)
                .spawn()
                .expect("Could not lock session.");
        }

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

//==============================================================================
// Internal functions
//==============================================================================

/// Send a HTTP GET request to query the status of the client.
#[tokio::main]
async fn get_status(status_url: &String) -> Result<Status, Error> {
    let response = reqwest::get(status_url).await?;
    let status: Status = response.json().await?;

    Ok(status)
}
