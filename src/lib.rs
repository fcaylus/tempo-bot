mod calendar;
mod http;
mod jira;
mod tempo;
pub mod utils;
mod work_event;

use crate::calendar::calendar_parser::CalendarParser;
use crate::calendar::meetings::FromIcal;
use crate::calendar::models::meeting::Meeting;
use crate::jira::jira_client::JiraClient;
use crate::jira::models::issue::Issue;
use crate::tempo::tempo_client::TempoClient;
use crate::utils::date::format_duration;
use crate::work_event::{ToWorkEvents, WorkEvents};
use chrono::NaiveDate;
use dialoguer::Confirm;
use log::Level::Info;
use log::{info, log_enabled, warn};

#[derive(Debug, Clone)]
pub struct Config {
    pub jira: JiraHttpConfig,
    pub tempo: TempoHttpConfig,

    pub board_id: i32,
    pub sprint_prefix: String,
    pub project_prefixes: Vec<String>,

    pub date: NaiveDate,

    pub target_workday_duration_seconds: Option<i32>,

    // Every logged duration will be a multiple of this increment
    pub work_increment_seconds: i32,
    // Any issue below this duration will be ignored
    pub min_work_increment_seconds: i32,

    pub calendar_ics: Option<String>,
    pub meeting_default_issue: Option<String>,

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

    let worklogs = tempo_client.list_worklogs(&config.date).await;
    let already_worked_time = worklogs
        .iter()
        .map(|worklog| worklog.time_spent_seconds)
        .sum::<i32>();
    let mut remaining_time = workday_duration - already_worked_time;

    if log_enabled!(Info) {
        info!(
            "Estimation field for the board: {}",
            estimation_field
                .as_deref()
                .unwrap_or("(no estimation field)")
        );
        info!(
            "Duration of the day: {}",
            format_duration(&workday_duration)
        );
        info!(
            "Already logged time: {}",
            format_duration(&already_worked_time)
        );
        info!("Remaining time: {}", format_duration(&remaining_time));
        info!("");
    }

    if remaining_time == 0 {
        info!("No time left to log, exiting.");
        return;
    }

    let meetings_events = fetch_meetings_events(&config).await;
    remaining_time -= meetings_events
        .iter()
        .map(|event| event.duration)
        .sum::<i32>();
    if log_enabled!(Info) {
        info!(
            "Remaining time (after meetings): {}",
            format_duration(&remaining_time)
        );
    }

    let issues_events = fetch_issues_events(
        &config,
        &jira_client,
        estimation_field.as_ref(),
        &remaining_time,
    )
    .await;

    if meetings_events.is_empty() && issues_events.is_empty() {
        warn!("No meetings or issues to log for the day, exiting.");
        return;
    }

    if config.dry_run {
        info!("Dry-run mode, exiting.");
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

    for event in meetings_events.iter() {
        event.log_to_tempo(&tempo_client, &config.date).await;
    }

    for event in issues_events.iter() {
        event.log_to_tempo(&tempo_client, &config.date).await;
    }

    info!("All logged!");
}

// TODO: ignore meetings already logged in tempo
async fn fetch_meetings_events(config: &Config) -> WorkEvents<Meeting> {
    if let Some(ics_file) = &config.calendar_ics {
        let parser = CalendarParser::new(ics_file);
        let calendar = parser.parse().await.unwrap();
        let meetings =
            Vec::<Meeting>::from_icalendar(&calendar, &config.date, &config.project_prefixes, true);

        if meetings.is_empty() {
            info!("No meeting found for the day.");
            return WorkEvents::new();
        }

        let events = meetings.to_sorted_events(
            &0,
            &0,
            &0,
            "",
            &config.date,
            config.meeting_default_issue.as_ref(),
        );

        if log_enabled!(Info) {
            info!("Found {} meetings:", events.len());
            for event in events.iter() {
                info!(
                    "- {}: {} / time: {}",
                    event.key,
                    event.event.title,
                    format_duration(&event.duration)
                );
            }
        }

        return events;
    }

    WorkEvents::new()
}

async fn fetch_issues_events(
    config: &Config,
    jira_client: &JiraClient,
    estimation_field: Option<&String>,
    remaining_time: &i32,
) -> WorkEvents<Issue> {
    let sprint = jira_client
        .first_active_sprint_for_prefix(config.board_id, config.sprint_prefix.as_str())
        .await
        .unwrap();

    info!("Found current sprint: {}", sprint.name);

    let issues = jira_client
        .list_issues_in_sprint(sprint.id, estimation_field, true)
        .await;

    if issues.is_empty() {
        info!("No issue found for the user.");
        return WorkEvents::new();
    }

    let issues_events = issues.to_sorted_events(
        remaining_time,
        &config.min_work_increment_seconds,
        &config.work_increment_seconds,
        &config.jira.email,
        &config.date,
        config.meeting_default_issue.as_ref(),
    );

    if log_enabled!(Info) {
        info!("Issues w/ scores & assigned times :");
        for event in issues_events.iter() {
            info!(
                "- {} (score: {:.2}): {} / time: {}",
                event.key,
                event.score,
                event.event.fields.summary,
                format_duration(&event.duration)
            );
        }
    }

    issues_events
}
