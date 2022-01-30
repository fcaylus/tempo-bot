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
        score *= thread_rng().gen_range(0.2..=1.2);

        return score;
    }
}

impl ToWorkEvents<Issue> for Vec<Issue> {
    // TODO: make sure all the "day_duration" is filled
    fn to_events(
        self,
        day_duration: &i32,
        min_duration: &i32,
        increment_duration: &i32,
        user_email: &str,
        date: &NaiveDate,
        _default_issue_key: Option<&String>,
    ) -> WorkEvents<Issue> {
        let mut events = Vec::new();

        let scores: Vec<f64> = self
            .iter()
            .map(|issue| issue.compute_time_score(user_email, date))
            .collect();
        let score_sum: f64 = scores.iter().sum();

        let min_duration_float = f64::from(*min_duration);
        let increment_duration_float = f64::from(*increment_duration);

        for (i, issue) in self.into_iter().enumerate() {
            let score = scores.get(i).unwrap();
            let mut time: f64 = f64::from(*day_duration) * score / score_sum;

            if time < min_duration_float {
                time = 0.0;
            }

            // Round to the specified increment
            if time % increment_duration_float < increment_duration_float / 2.0 {
                time -= time % increment_duration_float;
            } else {
                time += increment_duration_float - (time % increment_duration_float);
            }

            events.push(WorkEvent::new(
                time.ceil().to_i32().unwrap(),
                score.clone(),
                issue.key.to_string(),
                "".to_string(),
                issue,
            ));
        }

        return events;
    }
}
