use crate::{
    domain::prayer_time::{DailyPrayerSchedule, PrayerTime},
    errors::FormattingError,
    formatting::PrayerFormatter,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TerminalFormatter;

impl TerminalFormatter {
    pub fn new() -> Self {
        Self
    }

    fn format_entry(prayer_time: &PrayerTime) -> String {
        format!(
            "{:<12}: {}",
            prayer_time.prayer_name(),
            prayer_time.local_time().format("%I:%M %p")
        )
    }
}

impl PrayerFormatter for TerminalFormatter {
    fn format(&self, schedule: &DailyPrayerSchedule) -> Result<String, FormattingError> {
        let lines = schedule
            .prayers()
            .iter()
            .map(Self::format_entry)
            .collect::<Vec<_>>();

        Ok(format!(
            "Prayer times for {}\n{}",
            schedule.date().format("%Y-%m-%d"),
            lines.join("\n")
        ))
    }
}
