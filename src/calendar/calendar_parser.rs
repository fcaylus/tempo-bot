use bytes::Buf;
use ical::parser::ical::component::IcalCalendar;
use ical::IcalParser;
use reqwest::get;
use std::fs::File;
use std::io::BufReader;
use url::Url;

#[derive(Debug)]
pub struct CalendarParser {
    ics_file_name: String,
}

impl CalendarParser {
    pub fn new(ics_file_name: &str) -> Self {
        Self {
            ics_file_name: ics_file_name.to_string(),
        }
    }

    pub async fn parse(&self) -> Option<IcalCalendar> {
        let calendar;
        if Url::parse(&self.ics_file_name).is_ok() {
            calendar = self.parse_from_url().await;
        } else {
            calendar = self.parse_from_file();
        }

        return calendar;
    }

    fn parse_from_file(&self) -> Option<IcalCalendar> {
        let buf = BufReader::new(
            File::open(&self.ics_file_name)
                .expect(format!("Could not read ICS file from {}", self.ics_file_name).as_str()),
        );
        let reader = IcalParser::new(buf);

        for line in reader {
            return Some(line.unwrap());
        }
        return None;
    }

    async fn parse_from_url(&self) -> Option<IcalCalendar> {
        let buf = get(&self.ics_file_name)
            .await
            .expect(format!("Could not download ICS file from {}", self.ics_file_name).as_str())
            .bytes()
            .await
            .unwrap()
            .reader();
        let reader = IcalParser::new(buf);

        for line in reader {
            return Some(line.unwrap());
        }
        return None;
    }
}
