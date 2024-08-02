use crate::utils::{get_today_date, get_website_from_url};
use crate::db::{Database, AppMeta, AppType, AppCategory};
use std::{process::Command, str, };

fn get_chrome_active_tab() -> String {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Google Chrome\" to return URL of active tab of front window")
        .output()
        .expect("failed to execute process");

    str::from_utf8(&output.stdout).unwrap().to_string().trim().to_string()
}

fn get_active_app() -> String {
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first process whose frontmost is true")
        .output()
        .expect("failed to execute process");

    str::from_utf8(&output.stdout).unwrap().to_string().trim().to_string()
}

fn track_and_print ()  -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new()?;
    let today: String = get_today_date();

    let app_name = get_active_app();
    match app_name.as_str() {
        "Google Chrome" => {
            let tab = get_chrome_active_tab();
            let website = get_website_from_url(tab.as_str());

            db.insert_app_session(AppMeta{
                app_name: website.to_string(),
                description: Some(tab),
                app_type: Some(AppType::Website),
                category: Some(AppCategory::Other),
                id: None,
            })?;
        },
        _ => {
            db.insert_app_session(AppMeta {
                app_name: app_name,
                description: None,
                app_type: Some(AppType::App),
                category: Some(AppCategory::Other),
                id: None,
            })?;
        }
        
    }

    let results: Vec<(String, String)> = db.read_all_for_date(today.to_owned())?;
    for result in results {
        println!("{} => {}", result.0, result.1);
    }
        
    return Ok(());
}

pub fn track() {
      track_and_print().unwrap_or_else(|err| {
        println!("Error: {}", err);
    });
}