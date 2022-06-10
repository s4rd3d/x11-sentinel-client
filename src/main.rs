use std::env;
use std::sync::mpsc;
use std::thread;

use dotenv;

mod data_collector;
mod status;

fn main() {
    // Load environment variables for the specified runtime
    let runtime_env =
        env::var("RUNTIME_ENV").expect("`$RUNTIME_ENV` environment variable is not specified.");

    // Setup environment variables from the specified env file
    dotenv::from_filename(format!("env.{}", runtime_env)).ok();

    // Create communication channel between threads
    let (tx, rx) = mpsc::channel();

    // Start the status polling service
    thread::spawn(move || {
        status::run(tx);
    });

    // Start the data collection
    data_collector::run(rx);
}
