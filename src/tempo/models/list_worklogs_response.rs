use crate::tempo::models::worklog::WorkLog;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkLogsResponse {
    #[serde(rename = "self")]
    pub self_: String,
    pub metadata: ListWorkLogsResponseMetadata,
    pub results: Vec<WorkLog>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListWorkLogsResponseMetadata {
    pub count: i32,
    pub offset: i32,
    pub limit: i32,
}
