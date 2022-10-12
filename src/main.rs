use std::thread;
use uuid::Uuid;

mod config;
mod data_collector;
mod status;

fn main() {
    // Parse command line arguments and create application configuration
    let config = config::Config::new();
    let config2 = config.clone();

    // Generate unique stream identifier.
    let stream_id = Uuid::new_v4().to_string();
    let stream_id2 = stream_id.clone();

    // Start the status polling service
    thread::spawn(move || {
        status::run(config, stream_id);
    });

    // Start the data collection
    data_collector::run(config2, stream_id2);
}
