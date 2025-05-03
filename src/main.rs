#![warn(clippy::nursery)]
#![allow(clippy::option_if_let_else)]

use clap::Parser;
use cli::{Cli, Command};
use info::get_info;
use sort::Transaction;
use std::{collections::HashMap, path::PathBuf};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::FilterFn, layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod duplicates;
mod error;
mod info;
mod metadata;
mod sort;
mod stats;
mod walksongs;

pub use error::{Error, Result};

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

fn main() -> crate::Result<()> {
    let args = Cli::parse();
    setup_tracing(args.log_level);

    match args.command {
        Command::Sort { root, apply } => sort(root, apply, args.json)?,
        Command::DetectDupe(detect_dupe) => {
            if !detect_dupe.metadata && !detect_dupe.filename && !detect_dupe.stream {
                Err(error::Error::Cli(
                    "Please supply one of either --metadata, --filename, or --stream".to_string(),
                ))?;
            }
            duplicates::detect_duplicates(
                detect_dupe.root,
                detect_dupe.metadata,
                detect_dupe.filename,
                detect_dupe.stream,
                args.json,
            )?;
        }
        Command::Info(i) => {
            let info = get_info(&i.song, i.nonstandard)?;
            if args.json {
                println!("{}", serde_json::to_string_pretty(&info).unwrap());
            } else {
                println!("{}", toml::to_string_pretty(&info).unwrap());
            }
        }
        Command::Stats(s) => display_stats(s, args.json)?,
        Command::Hash { song } => {
            let hash = duplicates::hash_stream(&song)?.expect("hash_stream never exits with None");
            if args.json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&hash.to_string()).unwrap()
                );
            } else {
                println!("{hash}");
            }
        }
    }
    Ok(())
}

fn display_stats(s: cli::Stats, json: bool) -> crate::Result<()> {
    let songs = walksongs::get_songs(s.root.clone())?;
    let mut stats = stats::get_stats(&s.root, &songs)?;

    if !s.all {
        stats.total.clear();
    }

    if !s.tagged {
        stats.tagged.clear();
    }

    if !s.untagged {
        stats.untagged.clear();
    }

    if !s.sorted {
        stats.sorted.clear();
    }

    if !s.unsorted {
        stats.unsorted.clear();
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&stats).unwrap());
    } else {
        println!("{}", toml::to_string_pretty(&stats).unwrap());
    }
    Ok(())
}

fn sort(root: PathBuf, apply: bool, json: bool) -> Result<()> {
    let songs = walksongs::get_songs(root.clone())?;
    let mut transactions: Vec<Transaction> = songs
        .iter()
        .map(|s| sort::sort_song(&root, s))
        .try_fold(Vec::new(), |mut a, b| -> Result<Vec<Transaction>> {
            let mut b = match b {
                Ok(b) => b,
                Err(Error::MissingMetadata) => return Ok(a),
                Err(b) => return Err(b),
            };
            a.append(&mut b);
            Ok(a)
        })?;
    transactions.sort_unstable();
    if apply {
        for transaction in transactions {
            transaction.apply()?
        }
    } else if json {
        println!("{}", serde_json::to_string_pretty(&transactions).unwrap());
    } else {
        let transactions = transactions
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>();
        println!(
            "{}",
            toml::to_string_pretty(&HashMap::from([("Transactions", transactions)])).unwrap()
        );
    }
    Ok(())
}
