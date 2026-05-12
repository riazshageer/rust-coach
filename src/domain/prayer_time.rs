use chrono::{DateTime, Local, NaiveDate, Utc};
use salah::Prayer;
use serde::Serialize;

pub const DAILY_PRAYERS: [Prayer; 7] = [
    Prayer::Fajr,
    Prayer::Sunrise,
    Prayer::Dhuhr,
    Prayer::Asr,
    Prayer::Maghrib,
    Prayer::Isha,
    Prayer::Qiyam,
];

#[derive(Debug, Clone, Serialize)]
pub struct PrayerTime {
    prayer_name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    utc_timestamp: DateTime<Utc>,
}

impl PrayerTime {
    pub fn new(prayer: Prayer, utc_timestamp: DateTime<Utc>) -> Self {
        Self {
            prayer_name: prayer.name(),
            utc_timestamp,
        }
    }

    pub fn prayer_name(&self) -> &str {
        &self.prayer_name
    }

    pub fn utc_timestamp(&self) -> DateTime<Utc> {
        self.utc_timestamp
    }

    pub fn local_time(&self) -> DateTime<Local> {
        self.utc_timestamp.with_timezone(&Local)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyPrayerSchedule {
    date: NaiveDate,
    prayers: Vec<PrayerTime>,
}

impl DailyPrayerSchedule {
    pub fn new(date: NaiveDate, prayers: Vec<PrayerTime>) -> Self {
        Self { date, prayers }
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn prayers(&self) -> &[PrayerTime] {
        &self.prayers
    }
}
