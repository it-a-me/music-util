use music_manager::{
    get_songs,
    sort::{Transaction, sort_songs_transactions},
};

use crate::cli;

pub fn sort(args: cli::Sort, json: bool) -> anyhow::Result<()> {
    let songs = get_songs(args.root.clone())?;
    let transactions = sort_songs_transactions(&args.root, &songs)?;
    for transaction in transactions {
        println!("{}", fmt_transaction(&transaction, json)?);
        if args.apply {
            transaction.apply()?;
            if !json {
                println!("Success");
            }
        }
    }
    Ok(())
}

fn fmt_transaction(transaction: &Transaction, json: bool) -> Result<String, serde_json::Error> {
    if json {
        serde_json::to_string(transaction)
    } else {
        Ok(transaction.to_string())
    }
}
