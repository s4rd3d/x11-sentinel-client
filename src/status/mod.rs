use notify_rust::Notification;
use reqwest::Error;
use serde::Deserialize;
use std::env;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
pub struct Status {
    pub phase: String,
    pub description: String,
    pub value: f32,
}

/// Send a HTTP GET request to query the status of the client.
#[tokio::main]
async fn get_status() -> Result<Status, Error> {
    let request_url =
        env::var("APP_STATUS_URL").expect("Could not find `APP_STATUS_URL` environment variable.");

    let response = reqwest::get(&request_url).await?;
    let status: Status = response.json().await?;

    Ok(status)
}

pub fn run(tx: std::sync::mpsc::Sender<Status>) -> () {
    let status_interval: u64 = env::var("STATUS_INTERVAL")
        .expect("Could not find `STATUS_INTERVAL` environment variable.")
        .parse()
        .unwrap();

    loop {
        // Get status from the remote server.
        let status = match get_status() {
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

        // Send status update to the main thread.
        match tx.send(status) {
            Ok(_) => (),
            Err(error) => println!("Could not send status update: {}", error),
        }

        // Sleep for a configured amount of time.
        thread::sleep(Duration::from_secs(status_interval));
    }
}
