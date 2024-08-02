use chrono::Local;

pub fn sec_to_time(seconds: i32) -> String {
  let hours = seconds / 3600;
  let minutes = (seconds % 3600) / 60;
  let seconds = seconds % 60;
  return format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

pub fn get_today_date() -> String {
  //return YYYY-MM-DD
  let date = Local::now();
  return date.format("%Y-%m-%d").to_string();
}

pub fn get_time_now() -> String {
  //return HH:MM:SS.SSS
  let date = Local::now();
  return date.format("%H:%M:%S").to_string();
}

pub fn get_time_now_after_seconds(seconds: i32) -> String {
  //return HH:MM:SS.SSS
  let date = Local::now();
  let new_date = date + chrono::Duration::seconds(seconds as i64);
  return new_date.format("%H:%M:%S").to_string();
}

pub fn get_website_from_url(url: &str) -> String {
  // Split the URL by '/' and get the domain part
  let domain = url.split('/').nth(2).unwrap();
  
  // Split the domain by '.'
  let parts: Vec<&str> = domain.split('.').collect();
  
  // Determine the main domain based on the number of parts
  let website = if parts.len() == 2 {
      parts[0].to_string()
  } else if parts.len() > 2 && parts[0] == "www" {
      parts[1].to_string()
  } else {
      let a = parts[parts.len() - 2].to_string() + " " + parts[parts.len() - 3];
      a.to_string()
  };
  
  return website
}

pub const TRACK_INTERVAL: i32 = 5;




#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_website_from_url() {
        let url = "https://mail.google.com/mail/u/1/#spam";
        let website = get_website_from_url(url);
        assert_eq!(website, "google mail");
    }
    #[test]
    fn test_get_website_from_url_2() {
        let url = "https://www.youtube.com/watch?v=9bZkp7q19f0";
        let website = get_website_from_url(url);
        assert_eq!(website, "youtube");
    }
    #[test]
    fn test_get_website_from_url_3() {
        let url = " https://www.linkedin.com/feed/";
        let website = get_website_from_url(url);
        assert_eq!(website, "linkedin");
    }
    #[test]
    fn test_get_website_from_url_4() {
        let url =         "https://github.com/firefliesai/dashboard-ff/pull/3243/files";
        let website = get_website_from_url(url);
        assert_eq!(website, "github");
    }
}