use chrono;
use chrono::{NaiveDate, NaiveTime, Utc};

pub fn parse_date_from_str(date_str: &str) -> NaiveDate {
    let today = Utc::now().date().naive_utc();

    return match date_str {
        "today" => today,
        "yesterday" => today.pred(),
        _ => match NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
            Ok(x) => x,
            Err(_) => panic!("Could not parse the date {}", date_str),
        },
    };
}

pub fn date_to_tempo_format(date: &NaiveDate) -> String {
    return date.format("%Y-%m-%d").to_string();
}

pub fn time_to_tempo_format(time: &NaiveTime) -> String {
    return time.format("%H:%M:%S").to_string();
}

pub fn format_duration(duration_in_seconds: &i32) -> String {
    return format!(
        "{}h {}m {}s",
        duration_in_seconds / 3600,
        (duration_in_seconds / 60) % 60,
        duration_in_seconds % 60
    );
}
