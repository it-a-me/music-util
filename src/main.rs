use std::collections::HashMap;

use clap::Parser;
use cli::{Cli, Command};
use info::get_info;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::FilterFn, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod duplicates;
mod error;
mod info;
mod metadata;
mod stats;
mod walksongs;

pub use error::{Error, Result};

fn setup_tracing(max_level: tracing::Level) {
    let crate_filter = FilterFn::new(|s| {
        return !(s.target().starts_with("symphonia") && s.level() >= &tracing::Level::INFO);
    });

    tracing_subscriber::registry()
        .with(crate_filter)
        .with(LevelFilter::from_level(max_level))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

fn main() -> crate::Result<()> {
    let args = Cli::parse();
    setup_tracing(args.log_level);

    match args.command {
        Command::DetectDupe(detect_dupe) => {
            if !detect_dupe.metadata && !detect_dupe.filename && !detect_dupe.stream {
                Err(error::Error::Cli(
                    "Please supply one of either --metadata, --filename, or --stream".to_string(),
                ))?;
            }
            let duplicates = duplicates::detect_duplicates(
                detect_dupe.root,
                detect_dupe.metadata,
                detect_dupe.filename,
                detect_dupe.stream,
            )?;
            dbg!(duplicates);
        }
        Command::Info(i) => {
            let info = get_info(&i.song, i.nonstandard)?;
            print!("{}", toml::to_string_pretty(&info).unwrap());
        }
        Command::Stats(s) => {
            let songs = walksongs::get_songs(s.root)?;
            let stats = stats::get_stats(&songs)?;

            print!(
                "{}",
                toml::to_string_pretty(&HashMap::from([("Stats", stats.numbers())])).unwrap()
            );
            if s.tagged {
                print!(
                    "{}",
                    toml::to_string_pretty(&HashMap::from([("Tagged", &stats.tagged)])).unwrap()
                );
            }
            if s.untagged {
                print!(
                    "{}",
                    toml::to_string_pretty(&HashMap::from([("Untagged", &stats.untagged)]))
                        .unwrap()
                );
            }
        }
        Command::Hash { song } => {
            let hash = duplicates::hash_stream(&song)?.expect("hash_stream never exits with None");
            println!("{hash}");
        }
    }
    Ok(())
}
