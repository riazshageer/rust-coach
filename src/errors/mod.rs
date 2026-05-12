use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("latitude must be within -90..=90 degrees, got {0}")]
    InvalidLatitude(f64),
    #[error("longitude must be within -180..=180 degrees, got {0}")]
    InvalidLongitude(f64),
}

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("failed to calculate prayer times: {0}")]
    PrayerCalculation(String),
}

#[derive(Debug, Error)]
pub enum FormattingError {
    #[error("failed to serialize prayer times as JSON: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Service(#[from] ServiceError),
    #[error(transparent)]
    Formatting(#[from] FormattingError),
}
