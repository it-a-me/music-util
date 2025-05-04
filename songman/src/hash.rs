use music_manager::duplicates::hash_stream;

use crate::cli;

pub fn show_hash(args: cli::Hash, json: bool) -> anyhow::Result<()> {
    let hash = hash_stream(&args.song)?.expect("hash_stream never exits with None");
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&hash.to_string()).unwrap()
        );
    } else {
        println!("{hash}");
    }
    Ok(())
}
