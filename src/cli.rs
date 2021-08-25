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
    #[clap(long)]
    pub debug: Option<LevelFilter>,
}
