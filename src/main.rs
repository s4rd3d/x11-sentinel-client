use std::thread;

mod config;
mod data_collector;
mod status;

fn main() {
    // Parse command line arguments and create application configuration
    let config = config::Config::new();
    let config2 = config.clone();

    // Start the status polling service
    thread::spawn(move || {
        status::run(config);
    });

    // Start the data collection
    data_collector::run(config2);
}
