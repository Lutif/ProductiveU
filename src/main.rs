mod db;

use db::{Database, get_today_date};
use std::{process::Command, str, thread, time::Duration};

fn get_active_app() -> String {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first process whose frontmost is true")
        .output()
        .expect("failed to execute process");

    str::from_utf8(&output.stdout).unwrap().to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;

    let today = get_today_date();
    let seconds_used = 5;

    loop {
        let app_name = get_active_app();
        db.update_app_duration(&app_name, &today, seconds_used)?;
        thread::sleep(Duration::from_secs(seconds_used as u64));
        let results = db.read_all()?;
        for result in results {
            println!("{:?}", result);
        }
        
    }
}