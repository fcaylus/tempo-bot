use clap::Parser;
use env_logger::Env;
use log::info;
use tempo_bot::utils::date::parse_date_from_str;
use tempo_bot::{run, Config, JiraHttpConfig, TempoHttpConfig};

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Fabien C. fabien.caylus@gmail.com",
    about = "Log your time on Tempo automatically, based on your current tickets in Jira."
)]
pub struct Opts {
    /// The Jira host name. If you are using Jira Cloud, it's likely '<company>.atlassian.net'
    #[clap(short, long)]
    jira_host: String,
    /// The Board ID where your sprints live. Can be found in the url of your backlog/sprint page
    #[clap(short, long)]
    board_id: i32,
    /// The prefix used for your sprints, without the '#'
    #[clap(short, long)]
    sprint_prefix: String,

    /// The date to log time for. Accepted formats: 'today', 'yesterday', 'YYYY-MM-DD'
    #[clap(short, long)]
    date: String,

    /// Email (used to login in Jira)
    #[clap(short, long)]
    email: String,
    /// Jira API key. Can be generated from https://id.atlassian.com/manage/api-tokens
    #[clap(long)]
    api_key: String,

    /// Tempo API key. Can be generated from "Tempo > Settings > API integration" in your dashboard
    #[clap(long)]
    tempo_api_key: String,
    /// User's account id (used to login in Tempo). Can be found in the url of your profile page.
    #[clap(short, long)]
    account_id: String,

    /// Target working day duration (in hours). If not specified, the default day duration from Tempo is used.
    #[clap(long)]
    day_duration: Option<i32>,

    /// Increment of a work log (in minutes). Every work lok will be rounded to a multiple of this increment.
    #[clap(long, default_value = "30")]
    work_increment: i32,

    /// Minimal duration of a work (in minutes). Any work below this threshold will be skipped.
    #[clap(long, default_value = "15")]
    work_min_duration: i32,

    /// Dry run mode. If specified, no time will be logged
    #[clap(long)]
    dry_run: bool,

    /// Answer 'yes' to all question, thus bypassing any user input
    #[clap(short, long)]
    yes: bool,
}

#[tokio::main]
async fn main() {
    let env = Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let options: Opts = Opts::parse();

    let date = parse_date_from_str(options.date.as_str());

    info!("Jira Host         : {}", options.jira_host);
    info!("Board ID          : {}", options.board_id);
    info!("Sprint prefix     : {}", options.sprint_prefix);
    info!("Date              : {}", date);
    info!("Email             : {}", options.email);
    info!("Account ID        : {}", options.account_id);
    info!("API Key           : *****");
    info!("API Key for Tempo : *****");
    info!("-------------------------");
    info!("Work increment: {}m", options.work_increment);
    info!("Min work duration: {}m", options.work_min_duration);
    info!("");

    let config = Config {
        jira: JiraHttpConfig {
            host: options.jira_host,
            email: options.email,
            api_key: options.api_key,
        },
        tempo: TempoHttpConfig {
            api_key: options.tempo_api_key,
            account_id: options.account_id,
        },
        board_id: options.board_id,
        sprint_prefix: options.sprint_prefix,
        date,
        target_workday_duration_seconds: options.day_duration.map(|x| x * 3600),
        work_increment_seconds: options.work_increment * 60,
        min_work_increment_seconds: options.work_min_duration * 60,
        dry_run: options.dry_run,
        skip_confirmation: options.yes,
    };

    run(config).await;
}
