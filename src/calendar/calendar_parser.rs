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

        calendar
    }

    fn parse_from_file(&self) -> Option<IcalCalendar> {
        let buf = BufReader::new(
            File::open(&self.ics_file_name)
                .unwrap_or_else(|_| panic!("Could not read ICS file from {}", self.ics_file_name)),
        );
        let mut reader = IcalParser::new(buf);

        if let Some(line) = reader.next() {
            return Some(line.unwrap());
        }

        None
    }

    async fn parse_from_url(&self) -> Option<IcalCalendar> {
        let buf = get(&self.ics_file_name)
            .await
            .unwrap_or_else(|_| panic!("Could not download ICS file from {}", self.ics_file_name))
            .bytes()
            .await
            .unwrap()
            .reader();
        let mut reader = IcalParser::new(buf);

        if let Some(line) = reader.next() {
            return Some(line.unwrap());
        }

        None
    }
}
