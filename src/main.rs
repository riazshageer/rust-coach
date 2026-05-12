mod app;
mod domain;
mod errors;
mod formatting;
mod services;

use app::{config::AppConfig, prayer_app::PrayerApp};
use domain::location::Location;
use errors::AppError;
use formatting::{json_formatter::JsonFormatter, terminal_formatter::TerminalFormatter};
use salah::{Madhab, Method};

enum OutputFormat {
    Terminal,
    Json,
}

impl OutputFormat {
    fn from_args() -> Self {
        match std::env::args().nth(1).as_deref() {
            Some("--json") => Self::Json,
            _ => Self::Terminal,
        }
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Application error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let config = AppConfig::new(
        Location::new(10.525469, -61.3529187)?,
        Madhab::Shafi,
        Method::MuslimWorldLeague,
    );
    let app = PrayerApp::new(config);

    let output = match OutputFormat::from_args() {
        OutputFormat::Terminal => app.render_today(&TerminalFormatter::new())?,
        OutputFormat::Json => app.render_today(&JsonFormatter::new())?,
    };

    println!("{output}");

    Ok(())
}
