use crate::jira::models::issue::Issue;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListIssuesResponse {
    pub expand: String,
    pub start_at: i32,
    pub max_results: i32,
    pub total: i32,
    pub issues: Vec<Issue>,
}
