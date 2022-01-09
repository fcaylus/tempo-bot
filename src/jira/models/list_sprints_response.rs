use crate::jira::models::sprint::Sprint;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListSprintsResponse {
    pub max_results: i32,
    pub start_at: i32,
    pub is_last: bool,
    pub values: Vec<Sprint>,
}
