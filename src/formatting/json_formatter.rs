use serde::Serialize;

use crate::{
    domain::prayer_time::DailyPrayerSchedule,
    errors::FormattingError,
    formatting::PrayerFormatter,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct JsonFormatter;

#[derive(Debug, Serialize)]
struct JsonPrayerTime<'a> {
    name: &'a str,
    local_time: String,
    utc_time: String,
}

#[derive(Debug, Serialize)]
struct JsonSchedule<'a> {
    date: String,
    prayers: Vec<JsonPrayerTime<'a>>,
}

impl JsonFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl PrayerFormatter for JsonFormatter {
    fn format(&self, schedule: &DailyPrayerSchedule) -> Result<String, FormattingError> {
        let prayers = schedule
            .prayers()
            .iter()
            .map(|prayer_time| JsonPrayerTime {
                name: prayer_time.prayer_name(),
                local_time: prayer_time.local_time().format("%Y-%m-%d %H:%M:%S").to_string(),
                utc_time: prayer_time.utc_timestamp().to_rfc3339(),
            })
            .collect();

        Ok(serde_json::to_string_pretty(&JsonSchedule {
            date: schedule.date().to_string(),
            prayers,
        })?)
    }
}
