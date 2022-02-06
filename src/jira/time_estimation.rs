use crate::jira::models::priority::PriorityLevel;
use crate::work_event::{ToWorkEvents, WorkEvent, WorkEvents};
use crate::Issue;
use chrono::NaiveDate;
use num_traits::ToPrimitive;
use rand::{thread_rng, Rng};

impl Issue {
    pub fn compute_time_score(&self, user_email: &str, date: &NaiveDate) -> f64 {
        let mut score: f64 = self.estimation().unwrap_or(1.0);

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
        score *= thread_rng().gen_range(0.7..=1.3);

        score
    }
}

impl ToWorkEvents<Issue> for Vec<Issue> {
    fn to_events(
        self,
        day_duration: &i32,
        increment_duration: &i32,
        user_email: &str,
        date: &NaiveDate,
        _default_issue_key: Option<&String>,
    ) -> WorkEvents<Issue> {
        let mut events = Vec::new();

        // Compute score for each issue and order them by descending order
        let mut scores: Vec<IssueWithScore> = self
            .into_iter()
            .map(|issue| IssueWithScore {
                score: issue.compute_time_score(user_email, date),
                issue,
            })
            .collect();
        scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        let score_sum: f64 = scores.iter().map(|x| x.score).sum();
        let mut time_sum: i32 = 0;

        let increment_duration_f64 = f64::from(*increment_duration);

        for issue_with_score in scores.into_iter() {
            let issue = issue_with_score.issue;
            let score = issue_with_score.score;
            let mut time: f64 = f64::from(*day_duration) * score / score_sum;

            // Round to the specified increment
            if time % increment_duration_f64 < increment_duration_f64 / 2.0 {
                time -= time % increment_duration_f64;
            } else {
                time += increment_duration_f64 - (time % increment_duration_f64);
            }

            let mut time_i32 = time.round().to_i32().unwrap();

            // Round to at least 1 increment if there is some remaining time
            if time_i32 == 0 && time_sum < *day_duration {
                time_i32 = *increment_duration;
            }

            time_sum += time_i32;

            events.push(WorkEvent::new(
                time_i32,
                score,
                issue.key.to_string(),
                "".to_string(),
                None,
                issue,
            ));
        }

        events
    }
}

struct IssueWithScore {
    pub issue: Issue,
    pub score: f64,
}
