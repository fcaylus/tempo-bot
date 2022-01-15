mod http;
mod jira;
mod tempo;
pub mod utils;
mod work_event;

use crate::jira::jira_client::JiraClient;
use crate::jira::models::issue::Issue;
use crate::tempo::tempo_client::TempoClient;
use crate::utils::date::format_duration;
use crate::work_event::ToWorkEvents;
use chrono::NaiveDate;
use dialoguer::Confirm;
use log::{info, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub jira: JiraHttpConfig,
    pub tempo: TempoHttpConfig,

    pub board_id: i32,
    pub sprint_prefix: String,

    pub date: NaiveDate,

    pub target_workday_duration_seconds: Option<i32>,

    // Every logged duration will be a multiple of this increment
    pub work_increment_seconds: i32,
    // Any issue below this duration will be ignored
    pub min_work_increment_seconds: i32,

    pub dry_run: bool,
    pub skip_confirmation: bool,
}

#[derive(Debug, Clone)]
pub struct JiraHttpConfig {
    pub host: String,
    pub email: String,
    pub api_key: String,
}

#[derive(Debug, Clone)]
pub struct TempoHttpConfig {
    pub api_key: String,
    pub account_id: String,
}

pub async fn run(config: Config) {
    let jira_client = JiraClient::new(&config.jira);
    let tempo_client = TempoClient::new(&config.tempo);

    let workday_duration_tempo = tempo_client.work_duration(&config.date).await;
    let workday_duration = config
        .target_workday_duration_seconds
        .unwrap_or(workday_duration_tempo);

    let board_configuration = jira_client.get_board_configuration(config.board_id).await;
    let estimation_field = board_configuration.estimation_field_name();

    info!(
        "Estimation field for the board: {}",
        estimation_field
            .as_deref()
            .unwrap_or("(no estimation field)")
    );

    let sprint = jira_client
        .first_active_sprint_for_prefix(config.board_id, config.sprint_prefix.as_str())
        .await
        .unwrap();

    info!("Found current sprint: {:#?}", sprint);

    let issues = jira_client
        .list_issues_in_sprint(sprint.id, estimation_field, true)
        .await;

    if issues.len() == 0 {
        warn!("No issue found for the user, exiting.");
        return;
    }

    info!("Found {} issues for the user", issues.len());
    for issue in issues.iter() {
        info!("- {}: {}", issue.key, issue.fields.summary);
    }
    info!("");

    let worklogs = tempo_client.list_worklogs(&config.date).await;
    let already_worked_time = worklogs
        .iter()
        .map(|worklog| worklog.time_spent_seconds)
        .sum::<i32>();
    let remaining_time = workday_duration - already_worked_time;

    info!(
        "Duration of the day: {}",
        format_duration(&workday_duration)
    );
    info!(
        "Already logged time: {}",
        format_duration(&already_worked_time)
    );
    info!("Remaining time: {}", format_duration(&remaining_time));

    if remaining_time == 0 {
        info!("No time left to log, exiting.");
        return;
    }

    let issues_events = issues.to_sorted_events(
        &remaining_time,
        &config.min_work_increment_seconds,
        &config.work_increment_seconds,
        &config.jira.email,
        &config.date,
    );

    info!("");
    info!("Issues w/ scores & assigned times :");
    for issue_event in issues_events.iter() {
        info!(
            "- {} (score: {:.2}): {} / time: {}",
            issue_event.key,
            issue_event.score,
            issue_event.event.fields.summary,
            format_duration(&issue_event.duration)
        );
    }

    if config.dry_run {
        info!("Dry-run mode, exiting now.");
        return;
    }

    if !config.skip_confirmation
        && !Confirm::new()
            .with_prompt("Do you want to log your time, as specified above?")
            .wait_for_newline(true)
            .interact()
            .unwrap()
    {
        info!("Exiting.");
        return;
    }

    info!("Logging your time...");

    for issue_event in issues_events.iter() {
        if issue_event.duration == 0 {
            continue;
        }

        info!(
            "Logging {} for {}",
            format_duration(&issue_event.duration),
            issue_event.key
        );
        tempo_client
            .post_worklog(
                &config.date,
                &issue_event.key,
                &config.tempo.account_id,
                &issue_event.duration,
            )
            .await;
    }

    info!("All logged!");
}
