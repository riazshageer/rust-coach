use chrono::NaiveDate;
use salah::PrayerSchedule;

use crate::{
    app::config::AppConfig,
    domain::prayer_time::{DAILY_PRAYERS, DailyPrayerSchedule, PrayerTime},
    errors::ServiceError,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct PrayerService;

impl PrayerService {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_for(
        &self,
        date: NaiveDate,
        config: &AppConfig,
    ) -> Result<DailyPrayerSchedule, ServiceError> {
        let prayer_times = PrayerSchedule::new()
            .on(date)
            .for_location(config.location().to_coordinates())
            .with_configuration(config.salah_parameters())
            .calculate()
            .map_err(ServiceError::PrayerCalculation)?;

        let prayers = DAILY_PRAYERS
            .into_iter()
            .map(|prayer| PrayerTime::new(prayer, prayer_times.time(prayer)))
            .collect();

        Ok(DailyPrayerSchedule::new(date, prayers))
    }
}
