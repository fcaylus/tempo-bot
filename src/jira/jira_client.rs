use crate::http::http_client::{Credentials, HttpClient, HttpClientConfig};
use crate::jira::models::board_configuration::BoardConfiguration;
use crate::jira::models::issue::Issue;
use crate::jira::models::list_issues_response::ListIssuesResponse;

use crate::jira::models::list_sprints_response::ListSprintsResponse;
use crate::jira::models::sprint::Sprint;
use crate::JiraHttpConfig;

#[derive(Debug)]
pub struct JiraClient {
    client: HttpClient,
    config: JiraHttpConfig,
}

impl JiraClient {
    pub fn new(config: &JiraHttpConfig) -> Self {
        Self {
            client: HttpClient::new(HttpClientConfig::new(
                &config.host,
                "rest/agile/1.0",
                Credentials::UsernamePassword(config.email.to_string(), config.api_key.to_string()),
            )),
            config: config.clone(),
        }
    }

    pub async fn get_board_configuration(&self, board_id: i32) -> BoardConfiguration {
        self.client
            .get::<BoardConfiguration>(format!("board/{}/configuration", board_id).as_str())
            .await
    }

    pub async fn list_active_sprints(&self, board_id: i32) -> Vec<Sprint> {
        self.client
            .get::<ListSprintsResponse>(format!("board/{}/sprint?state=active", board_id).as_str())
            .await
            .values
    }

    pub async fn first_active_sprint_for_prefix(
        &self,
        board_id: i32,
        prefix: &str,
    ) -> Option<Sprint> {
        self.list_active_sprints(board_id)
            .await
            .into_iter()
            .filter(|sprint| sprint.name.starts_with(prefix))
            .nth(0)
    }

    pub async fn list_issues_in_sprint(
        &self,
        sprint_id: i32,
        additional_fields: Option<&Vec<String>>,
        for_current_user_only: bool,
    ) -> Vec<Issue> {
        let mut fields = vec![
            "issuetype",
            "timespent",
            "resolution",
            "resolutiondate",
            "workratio",
            "created",
            "epic",
            "priority",
            "labels",
            "assignee",
            "updated",
            "status",
            "components",
            "timetracking",
            "flagged",
            "summary",
            "creator",
            "reporter",
        ];

        if let Some(f) = additional_fields {
            for field in f {
                fields.push(field.as_str())
            }
        }

        let issues = self
            .client
            .get::<ListIssuesResponse>(
                format!(
                    "sprint/{}/issue?maxResults=1000&fields={}",
                    sprint_id,
                    fields.join(",")
                )
                .as_str(),
            )
            .await
            .issues;

        if for_current_user_only {
            let email = &self.config.email;
            return issues
                .into_iter()
                .filter(|issue| issue.is_assigned_to(email) || issue.was_reported_by(email))
                .collect();
        }

        return issues;
    }
}
