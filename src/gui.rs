use eframe::{
  epi::App,
  egui::{
    Vec2,
    CentralPanel,
    ScrollArea, CollapsingHeader
  }, 
  run_native
};
use crate::db::{Database,AppEntry,};

pub struct ProductiveU{
    app_entries: Vec<AppEntry>,
}

impl ProductiveU {
    pub fn new() -> Self {
    //insert 10 dummy app entries
    let mut app_entries = Database::new().unwrap().read_all().unwrap();
    app_entries.sort_by(|a, b| b.date.cmp(&a.date));
        Self { app_entries }
    }
    fn update_app_entries(&mut self, update:Vec<AppEntry>) {
        //update app_entries with the new data update
        self.app_entries = update;
    }
  }

impl App for ProductiveU {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut eframe::epi::Frame<'_>) {
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
            let mut group = CollapsingHeader::new(date.clone()).default_open(&dates[0]==date).show(ui, |ui| {
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
                columns[0].label(app_entry.app_name.clone());
                columns[1].label(app_entry.seconds_used.to_string());
                // columns[2].label(app_entry.date.clone());
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