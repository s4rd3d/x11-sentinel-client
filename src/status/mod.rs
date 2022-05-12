use std::thread;
use std::time::Duration;

pub fn run(tx: std::sync::mpsc::Sender<&str>) -> () {
    loop {
        match tx.send("Status call succeeded, updating state...") {
            Ok(_) => (),
            Err(error) => println!("Could not send status update: {}", error)
        }
        thread::sleep(Duration::from_secs(3));
    }
}
