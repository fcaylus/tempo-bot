mod http;
mod jira;
mod tempo;
pub mod utils;

use crate::jira::jira_client::JiraClient;
use crate::jira::models::issue::Issue;
use crate::tempo::tempo_client::TempoClient;
use crate::utils::date::format_duration;
use chrono::NaiveDate;
use dialoguer::Confirm;
use log::info;
use num_traits::ToPrimitive;
use std::collections::HashMap;

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

    let additional_fields = board_configuration.to_additional_fields_for_issues();
    let issues = jira_client
        .list_issues_in_sprint(sprint.id, additional_fields.as_ref(), true)
        .await;

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

    let mut issues_with_time_score = HashMap::<String /* key */, f64 /* score */>::new();
    for issue in issues.iter() {
        issues_with_time_score.insert(
            issue.key.clone(),
            issue.compute_time_score(
                estimation_field.as_deref(),
                config.jira.email.as_str(),
                &config.date,
            ),
        );
    }
    let mut issues_sorted = issues.into_iter().collect::<Vec<Issue>>();
    issues_sorted.sort_by(|a, b| {
        let a_entry = issues_with_time_score
            .entry(a.key.clone())
            .or_default()
            .clone();
        let b_entry = issues_with_time_score
            .entry(b.key.clone())
            .or_default()
            .clone();
        return b_entry.partial_cmp(&a_entry).unwrap();
    });

    info!("");
    info!("Ordered issue by score:");
    for issue in issues_sorted.iter() {
        info!(
            "- {} (score: {}): {}",
            issue.key,
            issues_with_time_score.entry(issue.key.clone()).or_default(),
            issue.fields.summary
        )
    }

    let times = assign_time_to_issues(
        &remaining_time,
        &config.min_work_increment_seconds,
        &config.work_increment_seconds,
        &issues_sorted,
        &issues_with_time_score,
    );

    info!("");
    info!("Issues w/ assigned times :");
    for issue in issues_sorted.iter() {
        info!(
            "- {}: {} / time: {}",
            issue.key,
            issue.fields.summary,
            format_duration(times.get(issue.key.as_str()).unwrap())
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

    for issue in issues_sorted.iter() {
        let duration = times.get(issue.key.as_str()).unwrap();
        if *duration == 0 {
            continue;
        }

        info!("Logging {} for {}", format_duration(duration), issue.key);
        tempo_client
            .post_worklog(&config.date, &issue.key, &config.tempo.account_id, duration)
            .await;
    }

    info!("All logged!");
}

fn assign_time_to_issues(
    remaining_time: &i32,
    min_duration: &i32,
    increment_duration: &i32,
    issues: &Vec<Issue>,
    scores_map: &HashMap<String, f64>,
) -> HashMap<String /* key */, i32 /* time */> {
    let mut times = HashMap::<String /* key */, i32 /* time*/>::new();
    let score_sum = scores_map.values().sum::<f64>();

    let min_duration_float = f64::from(*min_duration);
    let increment_duration_float = f64::from(*increment_duration);

    for issue in issues {
        let mut time: f64 =
            f64::from(*remaining_time) * scores_map.get(issue.key.as_str()).unwrap() / score_sum;

        if time < min_duration_float {
            time = 0.0;
        }

        // Round to the specified increment
        if time % increment_duration_float < increment_duration_float / 2.0 {
            time -= time % increment_duration_float;
        } else {
            time += increment_duration_float - (time % increment_duration_float);
        }

        times.insert(issue.key.clone(), time.ceil().to_i32().unwrap());
    }

    return times;
}
