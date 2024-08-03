mod db;
mod utils;
mod tracker;
mod ui;

use tracker::track;
use std::{thread, time::Duration};

use ui::ui;

fn main() {

    //use two threads one for tracking and one for ui
    let handle = thread::spawn(|| {
        loop {
            track();
            thread::sleep(Duration::from_secs(utils::TRACK_INTERVAL as u64));
        }
    });
    ui().unwrap();
    handle.join().unwrap();
    
}
