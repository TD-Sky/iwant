use clap::Parser;
use clap::ValueEnum;
use smol_str::SmolStr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// The manifest of applications
    pub manifest: PathBuf,

    /// The specified categories [delimiter: ,] [default: all]
    #[arg(long, short = 'C', value_delimiter = ',')]
    pub categories: Vec<SmolStr>,

    /// The excluded categories [delimiter: ,]
    #[arg(long, short = 'E', value_delimiter = ',')]
    pub exclude: Vec<SmolStr>,

    /// Don't display manifest
    #[arg(long, short, conflicts_with = "preview")]
    pub silent: bool,

    /// View the manifest without downloading
    #[arg(long, short, conflicts_with = "silent")]
    pub preview: bool,

    /// Enable extra managers [delimiter: ,]
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
