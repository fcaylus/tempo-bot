use crate::http::http_client::{Credentials, HttpClient, HttpClientConfig};
use crate::tempo::models::list_schedules_response::ListSchedulesResponse;
use crate::tempo::models::list_worklogs_response::ListWorkLogsResponse;
use crate::tempo::models::schedule::Schedule;
use crate::tempo::models::worklog::WorkLog;
use crate::utils::date::date_to_tempo_format;
use crate::TempoHttpConfig;
use chrono::NaiveDate;
use serde_json::{Number, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TempoClient {
    client: HttpClient,
    config: TempoHttpConfig,
}

impl TempoClient {
    pub fn new(config: &TempoHttpConfig) -> Self {
        Self {
            client: HttpClient::new(HttpClientConfig::new(
                "api.tempo.io",
                "core/3",
                Credentials::Bearer(config.api_key.to_string()),
            )),
            config: config.clone(),
        }
    }

    pub async fn list_schedules(&self, date: &NaiveDate) -> Vec<Schedule> {
        let date_as_str = date_to_tempo_format(date);
        let params = format!("from={}&to={}", &date_as_str, &date_as_str);

        self.client
            .get::<ListSchedulesResponse>(format!("user-schedule?{}", params).as_str())
            .await
            .results
    }

    pub async fn work_duration(&self, date: &NaiveDate) -> i32 {
        return self
            .list_schedules(date)
            .await
            .first()
            .unwrap()
            .required_seconds;
    }

    pub async fn list_worklogs(&self, date: &NaiveDate) -> Vec<WorkLog> {
        let date_as_str = date_to_tempo_format(date);
        let params = format!(
            "offset=0&limit=1000&from={}&to={}",
            &date_as_str, &date_as_str
        );

        self.client
            .get::<ListWorkLogsResponse>(
                format!("worklogs/user/{}?{}", self.config.account_id, params).as_str(),
            )
            .await
            .results
    }

    pub async fn post_worklog(
        &self,
        date: &NaiveDate,
        issue_key: &str,
        account_id: &str,
        duration: &i32,
    ) {
        let mut payload = HashMap::<String, Value>::new();
        payload.insert("issueKey".to_string(), Value::String(issue_key.to_string()));
        payload.insert(
            "timeSpentSeconds".to_string(),
            Value::Number(Number::from(*duration)),
        );
        payload.insert(
            "startDate".to_string(),
            Value::String(date_to_tempo_format(date)),
        );
        payload.insert("description".to_string(), Value::String(".".to_string()));
        payload.insert(
            "authorAccountId".to_string(),
            Value::String(account_id.to_string()),
        );

        self.client
            .post::<WorkLog>("worklogs", Some(&payload))
            .await;
    }
}