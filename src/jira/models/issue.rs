use crate::jira::models::component::Component;
use crate::jira::models::epic::Epic;
use crate::jira::models::priority::Priority;
use crate::jira::models::resolution::Resolution;
use crate::jira::models::status::Status;
use crate::jira::models::time_tracking::TimeTracking;
use crate::jira::models::user::User;
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub expand: String,
    pub id: String,
    #[serde(rename = "self")]
    pub self_: String,
    pub key: String,
    pub fields: IssueFields,

    // Not parsed from the JSON, but added later
    pub estimation_field_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueFields {
    pub creator: User,
    pub reporter: User,
    pub assignee: Option<User>,

    pub updated: String,
    pub created: String,
    #[serde(rename = "resolutiondate")]
    pub resolution_date: Option<String>,
    pub resolution: Option<Resolution>,

    pub summary: String,
    pub status: Status,
    #[serde(rename = "issuetype")]
    pub issue_type: IssueType,
    pub flagged: bool,
    pub epic: Option<Epic>,
    pub priority: Priority,

    pub components: Vec<Component>,
    pub labels: Vec<String>,

    #[serde(rename = "timespent")]
    pub time_spent: Option<i32>,
    #[serde(rename = "timetracking")]
    pub time_tracking: Option<TimeTracking>,
    #[serde(rename = "workratio")]
    pub work_ratio: i32,

    #[serde(flatten)]
    pub additional_fields: HashMap<String, Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IssueType {
    pub id: String,
    pub description: String,
    pub name: String,
    pub subtask: bool,
    pub hierarchy_level: i32,
}

impl Issue {
    pub fn is_assigned_to(&self, email: &str) -> bool {
        if let Some(assignee) = &self.fields.assignee {
            return assignee.is(email);
        }

        false
    }

    pub fn was_reported_by(&self, email: &str) -> bool {
        self.fields.reporter.is(email)
    }

    pub fn is_resolved(&self) -> bool {
        self.fields.resolution.is_some()
    }

    pub fn resolution_date(&self) -> Option<NaiveDate> {
        if let Some(resolution_date_str) = &self.fields.resolution_date {
            if let Ok(resolution_date) = NaiveDate::parse_from_str(resolution_date_str, "%+") {
                return Some(resolution_date);
            }
        }

        None
    }

    pub fn is_in_progress(&self) -> bool {
        self.fields.status.status_category.key == "indeterminate"
    }

    pub fn estimation(&self) -> Option<f64> {
        if let Some(name) = &self.estimation_field_name {
            if let Some(estimation) = self.fields.additional_fields.get(name) {
                return estimation.as_f64();
            }
        }

        None
    }
}
