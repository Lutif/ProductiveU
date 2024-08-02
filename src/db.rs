
use rusqlite::{Connection, Result, params};
use crate::utils::{get_today_date, get_time_now, get_time_now_after_seconds,sec_to_time, TRACK_INTERVAL};

const DATABASE : &str = "productiveU.db";


/* 
- one table to store app meta data: name , description, category
- one table to store app usage session (session is one continues usage of an app): app_id, start_time, end_time, duration, date
- one table to store hourly usage: date , hour, session_ids
*/

// data=> YYYY-MM-DD HH:MM:SS.SSS
const CREATE_APP_TABLE: &str = "CREATE TABLE IF NOT EXISTS app_meta (
                            id              INTEGER PRIMARY KEY,
                            app_name        TEXT NOT NULL,
                            description     TEXT,
                            category        TEXT,
                            app_type        TEXT
                                )" ;
const CREATE_APP_NAME_INDEX: &str = "CREATE INDEX IF NOT EXISTS idx_app_name ON app_meta (app_name)";
const INSERT_APP_META: &str = "INSERT INTO app_meta (app_name, description, category, app_type) VALUES (?, ?, ?, ?)";
const CREATE_APP_SESSION_TABLE: &str = "CREATE TABLE IF NOT EXISTS app_session (
                            id              INTEGER PRIMARY KEY,
                            app_id          INTEGER NOT NULL,
                            start_time      TEXT NOT NULL,
                            end_time        TEXT NOT NULL,
                            duration        INTEGER NOT NULL,
                            date            TEXT NOT NULL
                                )";
const CREATE_APP_SESSION_DATE_INDEX: &str = "CREATE INDEX IF NOT EXISTS idx_app_session_date ON app_session (date)";
const CREATE_APP_SESSION_APP_ID_INDEX: &str = "CREATE INDEX IF NOT EXISTS idx_app_session_app_id ON app_session (app_id)";
const CREATE_HOURS_TABLE: &str = "CREATE TABLE IF NOT EXISTS hourly_usage (
                            id              INTEGER PRIMARY KEY,
                            date            TEXT NOT NULL,
                            hour            INTEGER NOT NULL,
                            session_ids     TEXT NOT NULL
                                )";

const FIND_APP_META_BY_NAME: &str = "SELECT id FROM app_meta WHERE app_name = ?";
const INSERT_APP_SESSION: &str = "INSERT INTO app_session (app_id, start_time, end_time, duration, date) VALUES (?, ?, ?, ?, ?)";
const UPDATE_APP_SESSION: &str = "UPDATE app_session SET end_time = ?, duration = ? WHERE id = ?";

//todo: remove where
const GET_LAST_SESSION : &str = "SELECT id, app_id, start_time, end_time, duration, date FROM app_session ORDER BY id DESC LIMIT 1";

pub enum AppCategory {
    Social,
    Work,
    Entertainment,
    Other,
}
impl ToString for AppCategory {
    fn to_string(&self) -> String {
        match self {
            AppCategory::Social => "Social".to_string(),
            AppCategory::Work => "Work".to_string(),
            AppCategory::Entertainment => "Entertainment".to_string(),
            AppCategory::Other => "Other".to_string(),
        }
    }
}
pub enum AppType {
    Website,
    App,
}
impl ToString for AppType {
    fn to_string(&self) -> String {
        match self {
            AppType::Website => "Website".to_string(),
            AppType::App => "App".to_string(),
        }
    }
}
pub struct AppMeta {
    pub id: Option<u32>,
    pub app_name: String,
    pub description: Option<String>,
    pub category: Option<AppCategory>,
    pub app_type: Option<AppType>,
}
#[derive(Debug)]
pub struct  AppSession {
    pub id: u32,
    pub app_id: u32,
    pub start_time: String,
    pub end_time: String,
    pub duration: i32,
    pub date: String,
}
pub struct Database {
    conn: Connection,
}
impl Database {
    pub fn new() -> Result<Self> {
        let conn = Connection::open(DATABASE)?;
        conn.execute(CREATE_APP_TABLE,[])?;
        conn.execute(CREATE_APP_NAME_INDEX,[])?;
        conn.execute(CREATE_APP_SESSION_TABLE,[])?;
        conn.execute(CREATE_APP_SESSION_DATE_INDEX,[])?;
        conn.execute(CREATE_APP_SESSION_APP_ID_INDEX,[])?;
        conn.execute(CREATE_HOURS_TABLE,[])?;
        Ok(Self { conn })
    }

    fn find_or_insert_app(&self, app:AppMeta) -> Result<usize, rusqlite::Error> {

        let app_id = self.conn.query_row(
            FIND_APP_META_BY_NAME,
            params![app.app_name.to_owned()],
            |row| {
                Ok(row.get(0)?)
            },
        ).unwrap_or_else(|_| {
            self.conn.execute(INSERT_APP_META,
                [
                    &app.app_name, 
                    &app.description.unwrap_or_default(), 
                    &app.category.unwrap_or_else(|| AppCategory::Other).to_string(), 
                    &app.app_type.unwrap_or_else(||AppType::App).to_string()
                    ],
            ).unwrap();
            return self.conn.last_insert_rowid() as usize;
        });
        return Ok(app_id);
    
    }
    pub fn insert_app_session(&self, app:AppMeta) -> Result<u32, rusqlite::Error> {    
        let app_id = self.find_or_insert_app(app).unwrap() as u32;
        let last_session = self.conn.query_row(
            GET_LAST_SESSION,params![],
            |row| {
                Ok(AppSession {
                    id: row.get(0)?,
                    app_id: row.get(1)?,
                    start_time: row.get(2)?,
                    end_time: row.get(3)?,
                    duration: row.get(4)?,
                    date: row.get(5)?,
                })
            },
        );
        println!("{:?}", last_session);
        match last_session {
            Ok(session ) => {
                if session.app_id == app_id  {
                    self.conn.execute(
                        UPDATE_APP_SESSION,
                        params![get_time_now_after_seconds(TRACK_INTERVAL), session.duration + TRACK_INTERVAL, session.id],
                    ).unwrap();
                } else {
                    let start_time = get_time_now();
                    let end_time = get_time_now_after_seconds(TRACK_INTERVAL);
                    let duration = TRACK_INTERVAL;
                    let date = get_today_date();
                    self.conn.execute(
                        INSERT_APP_SESSION,
                        params![app_id, start_time, end_time, duration, date],
                    ).unwrap();
                }
            },
            Err(_) => {
                let start_time = get_time_now();
                let end_time = get_time_now_after_seconds(TRACK_INTERVAL);
                let duration = TRACK_INTERVAL;
                let date = get_today_date();
                self.conn.execute(
                    INSERT_APP_SESSION,
                    params![app_id, start_time, end_time, duration, date],
                ).unwrap();
            }
        }

        return Ok(0);
    }
    pub fn read_all_for_date(&self, date:String) -> Result<Vec<(String, String)>, rusqlite::Error> {
        //read all sessions from that date
        //join app_meta to get app name
        //combine all sessions for same app_id, add duration
        //return app_name, duration
        let sql = "SELECT app_meta.app_name, SUM(app_session.duration) FROM app_session JOIN app_meta ON app_session.app_id = app_meta.id WHERE app_session.date = ? GROUP BY app_session.app_id";
        let mut stmt = self.conn.prepare(sql)?;
        let results = stmt.query_map(params![date], |row| {
            Ok((row.get(0)?, sec_to_time(row.get(1)?)))
        })?;
        let mut sessions = Vec::new();
        for result in results {
            sessions.push(result.unwrap());
        }
        return Ok(sessions);
    }
}