use crate::db::{Database, get_today_date};
use std::{process::Command, str, thread, time::Duration};
const SECONDS_USED:i32 = 5;

fn get_active_app() -> String {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first process whose frontmost is true")
        .output()
        .expect("failed to execute process");

    str::from_utf8(&output.stdout).unwrap().to_string()
}

fn track_and_print ()  -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let today: String = get_today_date();

    loop {
        let app_name = get_active_app();
        db.update_app_duration(&app_name, &today, SECONDS_USED)?;
        thread::sleep(Duration::from_secs(SECONDS_USED as u64));
        let results = db.read_all()?;
        // for result in results {
        //     println!("{:?}", result);
        // }
        
    }
}

pub fn track() {
      track_and_print().unwrap_or_else(|err| {
        println!("Error: {}", err);
    });
}