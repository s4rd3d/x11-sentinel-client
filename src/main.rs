use std::env;
use std::sync::mpsc;
use std::thread;

use dotenv;

mod data_collector;
mod status;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Environment file was not specified. Start the application with `make run ENV=<environment file>`");
    }

    // Load environment variables for the specified runtime
    let env = &args[1];

    // Setup environment variables from the specified env file
    dotenv::from_filename(env).ok();

    // Create communication channel between threads
    let (tx, rx) = mpsc::channel();

    // Start the status polling service
    thread::spawn(move || {
        status::run(tx);
    });

    // Start the data collection
    data_collector::run(rx);
}
