use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTracking {
    pub time_spent: Option<String>,
    pub time_spent_seconds: Option<i32>,
    pub remaining_estimate: Option<String>,
    pub remaining_estimate_seconds: Option<i32>,
}
