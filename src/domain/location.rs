use salah::Coordinates;

use crate::errors::DomainError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Latitude(f64);

impl Latitude {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if (-90.0..=90.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidLatitude(value))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Longitude(f64);

impl Longitude {
    pub fn new(value: f64) -> Result<Self, DomainError> {
        if (-180.0..=180.0).contains(&value) {
            Ok(Self(value))
        } else {
            Err(DomainError::InvalidLongitude(value))
        }
    }

    pub fn value(self) -> f64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    latitude: Latitude,
    longitude: Longitude,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, DomainError> {
        Ok(Self {
            latitude: Latitude::new(latitude)?,
            longitude: Longitude::new(longitude)?,
        })
    }

    pub fn latitude(self) -> Latitude {
        self.latitude
    }

    pub fn longitude(self) -> Longitude {
        self.longitude
    }

    pub fn to_coordinates(self) -> Coordinates {
        Coordinates::new(self.latitude().value(), self.longitude().value())
    }
}
