use chrono::NaiveDate;

pub struct WorkEvent<T> {
    pub duration: i32,
    pub score: f64,
    pub key: String,
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
    ) -> WorkEvents<T>;

    fn to_sorted_events(
        self,
        day_duration: &i32,
        min_duration: &i32,
        increment_duration: &i32,
        user_email: &str,
        date: &NaiveDate,
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
        );
        events.sort_by(|a, b| b.duration.cmp(&a.duration));
        return events;
    }
}

impl<T> WorkEvent<T> {
    pub fn new(duration: i32, score: f64, key: String, event: T) -> Self {
        return Self {
            duration,
            score,
            key,
            event,
        };
    }
}
