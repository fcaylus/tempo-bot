use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Meeting {
    pub title: String,
    pub description: String,
    pub tempo_code: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub duration: i32,
}

impl Meeting {
    pub fn new_default() -> Self {
        Self {
            title: "".to_string(),
            description: "".to_string(),
            tempo_code: None,
            start_time: NaiveDateTime::from_timestamp(0, 0),
            end_time: NaiveDateTime::from_timestamp(0, 0),
            duration: 0,
        }
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.timestamp() != 0
    }

    pub fn has_end_time(&self) -> bool {
        self.end_time.timestamp() != 0
    }
}
