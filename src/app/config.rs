use salah::{Configuration, Madhab, Method, Parameters};

use crate::domain::location::Location;

#[derive(Debug, Clone, Copy)]
pub struct AppConfig {
    location: Location,
    madhab: Madhab,
    calculation_method: Method,
}

impl AppConfig {
    pub fn new(location: Location, madhab: Madhab, calculation_method: Method) -> Self {
        Self {
            location,
            madhab,
            calculation_method,
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn madhab(&self) -> Madhab {
        self.madhab
    }

    pub fn calculation_method(&self) -> Method {
        self.calculation_method
    }

    pub fn salah_parameters(&self) -> Parameters {
        Configuration::with(self.calculation_method(), self.madhab())
    }
}
