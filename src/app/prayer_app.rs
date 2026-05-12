use chrono::Local;

use crate::{
    app::config::AppConfig,
    domain::prayer_time::DailyPrayerSchedule,
    errors::AppError,
    formatting::PrayerFormatter,
    services::prayer_service::PrayerService,
};

pub struct PrayerApp {
    config: AppConfig,
    prayer_service: PrayerService,
}

impl PrayerApp {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            prayer_service: PrayerService::new(),
        }
    }

    pub fn schedule_for_today(&self) -> Result<DailyPrayerSchedule, AppError> {
        Ok(self
            .prayer_service
            .calculate_for(Local::now().date_naive(), &self.config)?)
    }

    pub fn render_today<F>(&self, formatter: &F) -> Result<String, AppError>
    where
        F: PrayerFormatter,
    {
        // The app owns the orchestration flow, but it only borrows the formatter.
        // This keeps presentation replaceable without introducing a service container.
        let schedule = self.schedule_for_today()?;

        Ok(formatter.format(&schedule)?)
    }
}
