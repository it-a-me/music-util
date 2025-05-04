#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::correctness)]

use clap::Parser;
use cli::{Cli, Command};
use duplicates::show_duplicates;
use hash::show_hash;
use info::show_info;
use stats::show_stats;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::FilterFn, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod duplicates;
mod hash;
mod info;
mod sort;
mod stats;

fn setup_tracing(max_level: tracing::Level) {
    let crate_filter = FilterFn::new(|s| {
        !(s.target().starts_with("symphonia") && s.level() >= &tracing::Level::INFO)
    });

    tracing_subscriber::registry()
        .with(crate_filter)
        .with(LevelFilter::from_level(max_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    setup_tracing(args.log_level);

    match args.command {
        Command::Sort(s) => sort::sort(s, args.json)?,
        Command::DetectDupe(d) => show_duplicates(d, args.json)?,
        Command::Info(i) => show_info(i, args.json)?,
        Command::Stats(s) => show_stats(s, args.json)?,
        Command::Hash(h) => show_hash(h, args.json)?,
    }
    Ok(())
}
