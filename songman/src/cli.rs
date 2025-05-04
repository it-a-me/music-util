use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
pub struct Cli {
    /// Log level { Trace, Debug, Info, Warn, Error }
    #[arg(short, long, default_value_t = tracing::Level::INFO)]
    pub log_level: tracing::Level,

    /// Output json
    #[arg(short, long)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    DetectDupe(DetectDupe),
    Info(Info),
    Stats(Stats),
    Sort(Sort),
    Hash(Hash),
}

#[derive(Parser, Debug, Clone)]
/// Detect duplicate music files
pub struct DetectDupe {
    /// Detect duplicate title metadata
    #[arg(short, long)]
    pub metadata: bool,

    /// Detect duplicate file names
    #[arg(short, long)]
    pub filename: bool,

    /// Detect duplicate music streams
    #[arg(short, long)]
    pub stream: bool,

    /// Root music directory
    #[arg()]
    pub root: PathBuf,
}

#[derive(Parser, Debug, Clone)]
/// Show info about a song
pub struct Info {
    /// Show nonstandard tags
    #[arg(short, long)]
    pub nonstandard: bool,

    /// Path to a song
    #[arg()]
    pub song: PathBuf,
}

#[derive(Parser, Debug, Clone)]
/// Show stats about a library
pub struct Stats {
    /// Show all songs
    #[arg(short, long)]
    pub all: bool,

    /// Show tagged songs
    #[arg(short, long)]
    pub tagged: bool,

    /// Show untagged songs
    #[arg(short = 'T', long)]
    pub untagged: bool,

    /// Show sorted songs
    #[arg(short, long)]
    pub sorted: bool,

    /// Show unsorted songs
    #[arg(short = 'S', long)]
    pub unsorted: bool,

    /// Root music directory
    #[arg()]
    pub root: PathBuf,
}

#[derive(Parser, Debug, Clone)]
/// Sort the music directory
pub struct Sort {
    /// Apply the sort POTENTIAL LOSS OF DATA MAY OCCUR
    #[arg(long)]
    pub apply: bool,

    /// Root music directory
    #[arg()]
    pub root: PathBuf,
}

#[derive(Parser, Debug, Clone)]
/// Show the hash of a song's streams
pub struct Hash {
    /// Path to a song
    #[arg()]
    pub song: PathBuf,
}
