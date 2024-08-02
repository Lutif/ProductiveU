mod db;
mod utils;
mod tracker;

use tracker::track;
use std::{thread, time::Duration};

fn main() {
    loop {
        track();
        thread::sleep(Duration::from_secs(utils::TRACK_INTERVAL as u64));
    }
}
