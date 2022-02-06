use crate::calendar::models::meeting::Meeting;
use crate::work_event::{WorkEvent, WorkEvents};
use crate::ToWorkEvents;
use chrono::{NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::UTC;
use ical::parser::ical::component::IcalCalendar;
use ical::property::Property;
use num_traits::ToPrimitive;
use regex::Regex;
use rrule::RRuleSet;

pub trait FromIcal<T> {
    fn from_icalendar(
        calendar: &IcalCalendar,
        date: &NaiveDate,
        project_prefixes: &[String],
        remove_overlaps: bool,
    ) -> T;
}

impl FromIcal<Vec<Meeting>> for Vec<Meeting> {
    fn from_icalendar(
        calendar: &IcalCalendar,
        date: &NaiveDate,
        project_prefixes: &[String],
        remove_overlaps: bool,
    ) -> Vec<Meeting> {
        let issues_regexes: Vec<Regex> = project_prefixes
            .iter()
            .map(|prefix| Regex::new(format!("{}-[0-9]+", prefix).as_str()).unwrap())
            .collect();

        let mut meetings: Vec<Meeting> = calendar
            .events
            .iter()
            .map(|event| {
                let mut meeting = Meeting::new_default();

                // These fields are used for computing the recurrence rule (if any)
                let mut has_rrule = false;
                // "rrule" properties are RRULE, RDATE, EXRULE, EXDATE and DTSTART
                let mut rrule_properties: Vec<String> = Vec::new();

                // Parse event properties
                for property in event.properties.iter() {
                    match property.name.as_str() {
                        "SUMMARY" => meeting.title = property.value.as_ref().unwrap().clone(),
                        "DESCRIPTION" => {
                            meeting.description = property.value.as_ref().unwrap().clone()
                        }
                        "DTEND" => meeting.end_time = parse_ical_date_time(property),
                        "DTSTART" | "RRULE" | "RDATE" | "EXRULE" | "EXDATE" => {
                            rrule_properties.push(format!(
                                "{}:{}",
                                property.name.clone(),
                                property.value.as_ref().unwrap().clone()
                            ));

                            match property.name.as_str() {
                                "DTSTART" => meeting.start_time = parse_ical_date_time(property),
                                "RRULE" => has_rrule = true,
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }

                // Abort early for meetings not on the specific date
                if (!has_rrule && meeting.start_time.date() != *date)
                    || (has_rrule && !check_rrule_on_date(&rrule_properties, date))
                {
                    return None;
                }

                // Parse tempo code from title, and fallback on description
                meeting.tempo_code = extract_tempo_code(meeting.title.as_str(), &issues_regexes)
                    .or_else(|| extract_tempo_code(meeting.description.as_str(), &issues_regexes));

                // Filter out meetings with no duration
                if !meeting.has_start_time() || !meeting.has_end_time() {
                    return None;
                }

                // Compute meeting duration
                meeting.duration = (meeting.end_time - meeting.start_time)
                    .num_seconds()
                    .to_i32()
                    .unwrap();

                Some(meeting)
            })
            .flatten()
            .collect();

        if !remove_overlaps {
            return meetings;
        }

        // Remove overlapping and duplicates meetings
        let mut filtered_meetings: Vec<Meeting> = Vec::new();
        while !meetings.is_empty() {
            let meeting1 = meetings.swap_remove(0);

            if meetings.is_empty() {
                // Adds the last element
                filtered_meetings.push(meeting1);
                break;
            }

            // TODO: improve meetings parsing by truncating overlapping meetings, instead of
            //       ignoring them
            let mut i = 0;
            while i < meetings.len() {
                let meeting2 = meetings.get(i).unwrap();

                // meeting1 starts before and ends after meeting2
                // -> keep meeting1
                if meeting1.start_time <= meeting2.start_time
                    && meeting1.end_time >= meeting2.end_time
                {
                    filtered_meetings.push(meeting1);
                    meetings.swap_remove(i);
                    break;
                }

                // meeting1 starts before and ends during meeting2
                // -> keep meeting1 if longer
                if meeting1.start_time < meeting2.start_time
                    && meeting1.end_time > meeting2.start_time
                    && meeting1.duration > meeting2.duration
                {
                    filtered_meetings.push(meeting1);
                    meetings.swap_remove(i);
                    break;
                }

                // meeting1 starts during and ends after meeting2
                // -> keep meeting1 if longer
                if meeting1.start_time > meeting2.start_time
                    && meeting1.start_time < meeting2.end_time
                    && meeting1.end_time > meeting2.end_time
                    && meeting1.duration > meeting2.duration
                {
                    filtered_meetings.push(meeting1);
                    meetings.swap_remove(i);
                    break;
                }

                // meeting1 starts during and ends during meeting2
                // -> skip meeting1
                if meeting1.start_time >= meeting2.start_time
                    && meeting1.end_time <= meeting2.end_time
                {
                    break;
                }

                // In any other case, meeting1 and meeting2 doesn't overlap
                // -> check other meetings
                i += 1;

                if i == meetings.len() {
                    // meeting1 did not overlap with any other meeting
                    // -> keep it
                    filtered_meetings.push(meeting1);
                    break;
                }
            }
        }

        filtered_meetings
    }
}

impl ToWorkEvents<Meeting> for Vec<Meeting> {
    fn to_events(
        self,
        _day_duration: &i32,
        _increment_duration: &i32,
        _user_email: &str,
        _date: &NaiveDate,
        default_issue_key: Option<&String>,
    ) -> WorkEvents<Meeting> {
        self.into_iter()
            .map(|meeting| {
                WorkEvent::new(
                    meeting.duration,
                    1.0,
                    meeting
                        .tempo_code
                        .clone()
                        .or_else(|| default_issue_key.cloned())
                        .unwrap_or_default(),
                    meeting.title.to_string(),
                    Some(meeting.start_time.time()),
                    meeting,
                )
            })
            .collect()
    }
}

// TODO: timezone support (the TZ is currently ignored)
fn parse_ical_date_time(property: &Property) -> NaiveDateTime {
    let str_date_time = property.value.as_ref().unwrap().clone();
    return NaiveDateTime::parse_from_str(str_date_time.as_str(), "%Y%m%dT%H%M%S").unwrap();
}

fn check_rrule_on_date(recurrence_rules: &[String], date: &NaiveDate) -> bool {
    let rrule_str = recurrence_rules.join("\n");
    let rrule_set: RRuleSet = rrule_str.parse().unwrap();

    let recurrences_on_date = rrule_set.between(
        UTC.from_local_datetime(&date.and_hms(0, 0, 0)).unwrap(),
        UTC.from_local_datetime(&date.and_hms(23, 59, 59)).unwrap(),
        true,
    );

    return recurrences_on_date
        .iter()
        .any(|rec| rec.naive_utc().date() == *date);
}

fn extract_tempo_code(text: &str, issues_regexes: &[Regex]) -> Option<String> {
    for regex in issues_regexes {
        if let Some(regex_match) = regex.find(text) {
            return Some(regex_match.as_str().to_string());
        }
    }
    None
}
