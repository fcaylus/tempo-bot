use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkLog {
    #[serde(rename = "self")]
    pub self_: String,
    pub tempo_worklog_id: i32,
    pub jira_worklog_id: i32,
    pub issue: WorkLogIssue,
    pub time_spent_seconds: i32,
    pub billable_seconds: i32,
    pub start_date: String,
    pub start_time: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub author: WorkLogAuthor,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkLogIssue {
    #[serde(rename = "self")]
    pub self_: String,
    pub key: String,
    pub id: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkLogAuthor {
    #[serde(rename = "self")]
    pub self_: String,
    pub account_id: String,
    pub display_name: String,
}
