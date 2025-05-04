use music_manager::info::get_info;

use crate::cli;

pub fn show_info(args: cli::Info, json: bool) -> anyhow::Result<()> {
    let info = get_info(&args.song, args.nonstandard)?;
    if json {
        println!("{}", serde_json::to_string_pretty(&info).unwrap());
    } else {
        println!("{}", toml::to_string_pretty(&info).unwrap());
    }
    Ok(())
}
