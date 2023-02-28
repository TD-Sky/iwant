use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The manifest (.toml) of applications
    pub manifest: PathBuf,

    /// The specified categories [default: all]
    #[arg(long, short = 'C', value_delimiter = ',')]
    pub categories: Vec<String>,

    /// The excluded categories
    #[arg(long, short = 'E', value_delimiter = ',')]
    pub exclude: Vec<String>,

    /// Don't display manifest
    #[arg(long, short, conflicts_with = "preview")]
    pub silent: bool,

    /// View the manifest without downloading
    #[arg(long, short, conflicts_with = "silent")]
    pub preview: bool,

    /// Enable extra managers
    #[arg(long, short = 'm', value_delimiter = ',', value_name = "MANAGERS")]
    pub extra_managers: Vec<ExtraManager>,
}

#[derive(Clone, ValueEnum, PartialEq, Eq)]
pub enum ExtraManager {
    Paru,
    Flatpak,
    Npm,
    Cargo,
}
