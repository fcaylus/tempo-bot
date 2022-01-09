use crate::tempo::models::schedule::Schedule;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListSchedulesResponse {
    #[serde(rename = "self")]
    pub self_: String,
    pub metadata: ListSchedulesResponseMetadata,
    pub results: Vec<Schedule>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListSchedulesResponseMetadata {
    pub count: i32,
}
