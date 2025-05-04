use music_manager::get_songs;

pub fn show_duplicates(args: crate::cli::DetectDupe, json: bool) -> anyhow::Result<()> {
    if !args.metadata && !args.filename && !args.stream {
        anyhow::bail!(
            "Please supply one of either --metadata, --filename, or --stream".to_string()
        );
    }
    let songs = get_songs(args.root)?;
    let duplicates = music_manager::duplicates::detect_duplicates(
        &songs,
        args.metadata,
        args.filename,
        args.stream,
    )?;
    if json {
        println!("{}", serde_json::to_string_pretty(&duplicates).unwrap());
    } else {
        println!("{}", toml::to_string_pretty(&duplicates).unwrap());
    }
    Ok(())
}
