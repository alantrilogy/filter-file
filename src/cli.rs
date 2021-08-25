//! Defines CLI options

use clap::{AppSettings, Clap};
use log::LevelFilter;

#[derive(Clap)]
#[clap(version = "1.0", author = "@savish")]
#[clap(setting = AppSettings::ColoredHelp)]
/// CLI Options
pub struct Opts {
    /// File to clean
    pub input: String,
    /// Debug level
    #[clap(long, default_value = "info")]
    pub debug: LevelFilter,
    /// Start of range to clean
    #[clap(long)]
    pub from: Option<usize>,
    /// End of range to clean
    #[clap(long)]
    pub to: Option<usize>,
}
