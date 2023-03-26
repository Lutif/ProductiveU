mod db;
mod gui;
mod tracker;
use gui::ui;
use tracker::track;
use std::thread;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //spawn a thread to track the app usage
    let tracker_thread_handle = thread::spawn(|| {
        print!("Starting Tracker\n");
        track();
    });
    ui();
    tracker_thread_handle.join().unwrap();

   Ok(())
}