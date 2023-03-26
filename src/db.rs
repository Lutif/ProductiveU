
use rusqlite::{Connection, Result};
use chrono::Local;
const DATABASE : &str = "productiveU.db";
#[derive(Debug)]
#[derive(Clone)]
pub struct AppEntry {
    pub id: i32,
    pub app_name: String,
    pub date: String,
    pub seconds_used: i32,
}

pub struct Database {
    conn: Connection,
}
impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(DATABASE)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS app_entry (
                      id              INTEGER PRIMARY KEY,
                      app_name        TEXT NOT NULL,
                      date            TEXT NOT NULL,
                      seconds_used    INTEGER NOT NULL
                      )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn update_app_duration(&self, app_name: &str, date: &str, seconds_used: i32) -> Result<()> {
        let mut stmt = self.conn.prepare("SELECT id, app_name, date, seconds_used FROM app_entry WHERE app_name = ?1 AND date = ?2")?;
        let app_entry_iter = stmt.query_map([&app_name, &date], |row| {
            Ok(AppEntry {
                id: row.get(0).unwrap(),
                app_name: row.get(1).unwrap(),
                date: row.get(2).unwrap(),
                seconds_used: row.get(3).unwrap(),
            })
        })?;

        let mut count = 0;
        for app_entry in app_entry_iter {
            count = 1;
            let app_entry = app_entry?;
            let update = "UPDATE app_entry SET seconds_used = ?1 WHERE id = ?2";
            self.conn.execute(update, [app_entry.seconds_used + seconds_used, app_entry.id])?;
        }

        if count < 1 {
            let sample = "INSERT INTO app_entry (app_name, date, seconds_used) VALUES (?1, ?2, ?3)";
            self.conn.execute(sample, [app_name, date, &seconds_used.to_string()])?;
        }

        Ok(())
    }

    pub fn read_all(&self) -> Result<Vec<AppEntry>> {
        let mut stmt = self.conn.prepare("SELECT id, app_name, date, seconds_used FROM app_entry")?;
        let app_entry_iter = stmt.query_map([], |row| {
            Ok(AppEntry {
                id: row.get(0).unwrap(),
                app_name: row.get(1).unwrap(),
                date: row.get(2).unwrap(),
                seconds_used: row.get(3).unwrap(),
            })
        })?;

        let mut results = vec![];
        for app_entry in app_entry_iter {
            results.push(app_entry?);
        }

        Ok(results)
    }
}

pub fn get_today_date() -> String {
    Local::now().format("%d-%m-%y").to_string()
}
