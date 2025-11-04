use std::fmt;

use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Format {
    #[default]
    Json,
    CSV,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let format_name = match self {
            Format::CSV => "csv",
            Format::Json => "json",
        };

        write!(f, "{}", format_name)
    }
}
