use music_manager::{get_songs, stats::get_stats};

use crate::cli;

pub fn show_stats(s: cli::Stats, json: bool) -> anyhow::Result<()> {
    let songs = get_songs(s.root.clone())?;
    let mut stats = get_stats(&s.root, &songs)?;

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
