use crate::jira::models::component::Component;
use crate::jira::models::epic::Epic;
use crate::jira::models::priority::{Priority, PriorityLevel};
use crate::jira::models::resolution::Resolution;
use crate::jira::models::status::Status;
use crate::jira::models::time_tracking::TimeTracking;
use crate::jira::models::user::User;
use chrono::NaiveDate;
use rand::{thread_rng, Rng};
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

        return false;
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

        return None;
    }

    pub fn is_in_progress(&self) -> bool {
        self.fields.status.status_category.key == "indeterminate"
    }

    pub fn estimation(&self, estimation_field_name: Option<&str>) -> Option<f64> {
        if let Some(name) = estimation_field_name {
            if let Some(estimation) = self.fields.additional_fields.get(name) {
                return estimation.as_f64();
            }
        }

        return None;
    }

    pub fn compute_time_score(
        &self,
        estimation_field_name: Option<&str>,
        user_email: &str,
        date: &NaiveDate,
    ) -> f64 {
        let mut score: f64 = self.estimation(estimation_field_name).unwrap_or(1.0);

        if self.is_assigned_to(user_email) {
            score *= 2.0;
        }

        if self.is_in_progress() {
            score *= 2.0;
        }

        if self.is_resolved() && self.resolution_date() != Some(*date) {
            score /= 3.0;
        }

        match self.fields.priority.level() {
            PriorityLevel::Highest => score *= 1.5,
            PriorityLevel::High => score *= 1.2,
            PriorityLevel::Low => score *= 0.8,
            _ => (),
        }

        // Add a bit of randomness
        score *= thread_rng().gen_range(0.2..=1.2);

        return score;
    }
}
