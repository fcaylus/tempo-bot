use crate::TempoClient;
use chrono::NaiveDate;
use log::info;

use crate::utils::date::format_duration;

pub struct WorkEvent<T> {
    pub duration: i32,
    pub score: f64,
    pub key: String,
    pub description: String,
    pub event: T,
}

pub type WorkEvents<T> = Vec<WorkEvent<T>>;

pub trait ToWorkEvents<T> {
    fn to_events(
        self,
        day_duration: &i32,
        min_duration: &i32,
        increment_duration: &i32,
        user_email: &str,
        date: &NaiveDate,
        default_issue_key: Option<&String>,
    ) -> WorkEvents<T>;

    fn to_sorted_events(
        self,
        day_duration: &i32,
        min_duration: &i32,
        increment_duration: &i32,
        user_email: &str,
        date: &NaiveDate,
        default_issue_key: Option<&String>,
    ) -> WorkEvents<T>
    where
        Self: Sized,
    {
        let mut events = self.to_events(
            day_duration,
            min_duration,
            increment_duration,
            user_email,
            date,
            default_issue_key,
        );
        events.sort_by(|a, b| b.duration.cmp(&a.duration));
        return events;
    }
}

impl<T> WorkEvent<T> {
    pub fn new(duration: i32, score: f64, key: String, description: String, event: T) -> Self {
        return Self {
            duration,
            score,
            key,
            description,
            event,
        };
    }

    pub async fn log_to_tempo(&self, tempo_client: &TempoClient, date: &NaiveDate) {
        if self.duration == 0 || self.key.is_empty() {
            return;
        }

        info!(
            "Logging {} for {}",
            format_duration(&self.duration),
            &self.key
        );

        tempo_client
            .post_worklog(date, &self.key, &self.duration, self.description.as_str())
            .await;
    }
}
