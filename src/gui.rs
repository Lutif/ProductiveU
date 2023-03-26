use eframe::{
  epi::App,
  egui::{
    Vec2,
    CentralPanel,
    ScrollArea, CollapsingHeader, self
  }, 
  run_native
};
use crate::db::{Database,AppEntry,};
use std::time::{Instant, Duration};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct ProductiveU{
    app_entries: Vec<AppEntry>,
    last_update: Instant,
}
const REFRESH_RATE: u64 = 6;//in seconds

fn get_time_formated(app_entry: &AppEntry) -> (String, String, String) {
  let hours = app_entry.seconds_used / 3600;
  let minutes = (app_entry.seconds_used % 3600) / 60;
  let seconds = (app_entry.seconds_used % 3600) % 60;
  //show time two digits for each unit
  let hours = format!("{:02}", hours);
  let minutes = format!("{:02}", minutes);
  let seconds = format!("{:02}", seconds);
  (hours, minutes, seconds)
}

fn get_app_name_color(app_entry: &AppEntry) -> egui::Color32 {
  let mut hasher = DefaultHasher::new();
  app_entry.app_name.hash(&mut hasher);
  let app_name_hash = hasher.finish();
  let r = (app_name_hash as u32 % 256) as u8;
  let g = ((app_name_hash >> 8) as u32 % 256) as u8;
  let b = ((app_name_hash >> 16) as u32 % 256) as u8;
  let color = egui::Color32::from_rgb(r, g, b);
  color
}

impl ProductiveU {
    pub fn new() -> Self {
    let mut app_entries = Database::new().unwrap().read_all().unwrap();
    app_entries.sort_by(|a, b| b.date.cmp(&a.date));
        Self { app_entries , last_update: Instant::now() }
    }
    fn update_app_entries(&mut self) {
        let update = Database::new().unwrap().read_all().unwrap();
        self.app_entries = update;
    }
  }

impl App for ProductiveU {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &mut eframe::epi::Frame<'_> ) {
      
      let now = Instant::now();
      if now.duration_since(self.last_update) > Duration::from_secs(REFRESH_RATE) {
        self.update_app_entries();
        self.last_update = now;
      }
      
      CentralPanel::default().show(ctx, |ui| {
        ScrollArea::auto_sized().show(ui, |ui| {
          //get all dates from app_entries
          let mut dates = Vec::new();
          self.app_entries.iter().for_each(|app_entry| {
            if !dates.contains(&app_entry.date) {
              dates.push(app_entry.date.clone());
            }
          });
          //create separate panels for each date
          dates.iter().for_each(|date| {
            CollapsingHeader::new(date.clone()).default_open(&dates[0]==date).show(ui, |ui| {
            //get all app_entries for each date
            let mut app_entries = Vec::new();
            self.app_entries.iter().for_each(|app_entry| {
              if app_entry.date == *date {
                app_entries.push(app_entry.clone());
              }
            });
            //sort app_entries by seconds_used
            app_entries.sort_by(|a, b| b.seconds_used.cmp(&a.seconds_used));
            //create a panel for each app_entry
            app_entries.iter().for_each(|app_entry| {
              ui.columns(3, |columns| {
                //use random color for label text
                let color = get_app_name_color(app_entry);             let text_style = egui::TextStyle::Body;
                columns[0].add(egui::Label::new(app_entry.app_name.clone()).text_style(text_style).text_color(color));
                let (hours, minutes, seconds) = get_time_formated(app_entry);
                columns[1].add(egui::Label::new(format!("{}:{}:{}", hours, minutes, seconds)).text_style(text_style).text_color(egui::Color32::WHITE));
              });
            });
          });
          });       
        });
      });

    }

    fn name(&self) -> &str { 
      "ProductiveU!"
    }
    fn setup(
            &mut self,
            _ctx: &eframe::egui::CtxRef,
            _frame: &mut eframe::epi::Frame<'_>,
            _storage: Option<&dyn eframe::epi::Storage>,
        ) {
        //loaded once at the start of the app
        
    }
  
}

pub fn ui() {
    let app = ProductiveU::new();
    let mut win_option = eframe::NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(300.0, 600.0));
    run_native(Box::new(app), win_option);
}