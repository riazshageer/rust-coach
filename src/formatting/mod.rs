pub mod json_formatter;
pub mod terminal_formatter;

use crate::{domain::prayer_time::DailyPrayerSchedule, errors::FormattingError};

pub trait PrayerFormatter {
    fn format(&self, schedule: &DailyPrayerSchedule) -> Result<String, FormattingError>;
}
