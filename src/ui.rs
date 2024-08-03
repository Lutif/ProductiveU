use iced::widget::{Button, Text, Row, Column};
use iced:: {Element, Sandbox, Settings};

use crate::utils::get_today_date;
use crate::db::Database;

pub fn ui() -> iced::Result {
  PView::run(Settings::default())
}

struct AppUsage {
    app_name: String,
    usage: String,
}

struct PView {
    apps_usage: Vec<AppUsage>,
    db: Database,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RefreshPressed,
}

impl Sandbox for PView {
    type Message = Message;

    fn new() -> Self {
      let db = Database::new().unwrap();
      let today: String = get_today_date();
      let results: Vec<(String, String)> = db.read_all_for_date(today.to_owned()).unwrap();
      let apps_usage = results.iter().map(|(app_name, usage)| {
          AppUsage {
              app_name: app_name.to_owned(),
              usage: usage.to_owned(),
          }
      }).collect();
        Self { apps_usage, db }
    }

    fn title(&self) -> String {
        String::from("Productive U - Be mindful of your time")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RefreshPressed => {
                let today: String = get_today_date();
                let results: Vec<(String, String)> = self.db.read_all_for_date(today.to_owned()).unwrap();
                self.apps_usage = results.iter().map(|(app_name, usage)| {
                    AppUsage {
                        app_name: app_name.to_owned(),
                        usage: usage.to_owned(),
                    }
                }).collect();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut base = Column::new().padding(20).spacing(10);

        let header = Row::new()
        .push(Text::new("Be mindful of your time").size(30))
        .push(Button::new("Refresh").on_press(Message::RefreshPressed).padding(10))
        .spacing(40);
        base = base.push(header);

        let table_header = Row::new()
            .push(Text::new("App Name").width(iced::Length::Fill).size(20))
            .push(Text::new("Used Today").width(iced::Length::Fill).size(20));
        base = base.push(table_header);

        for app_usage in self.apps_usage.iter() {
            let row = Row::new()
                .push(Text::new(&app_usage.app_name).width(iced::Length::Fill))
                .push(Text::new(&app_usage.usage).width(iced::Length::Fill));
            base = base.push(row);
        }

        base.into()
    }
}
