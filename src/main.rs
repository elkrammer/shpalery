use crate::wallpaper::Wallpaper;
use clap::Parser;
use rand::seq::SliceRandom;
use std::process;
use wallpaper::process_wallpaper_batch;

use crate::args::ShpaleryArgs;
use crate::config::Config;

mod args;
mod config;
mod database;
mod lib;
mod reddit;
mod wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();
    let mut config = Config::load();
    let args = ShpaleryArgs::parse();

    if let Some(amount) = args.amount {
        config.amount = amount;
    }

    if let Some(fetch_type) = args.fetch_type {
        config.fetch_type = fetch_type;
    }

    if args.verbose {
        Config::print_config(&config);
    }

    for sr in config.subreddits.iter() {
        let posts: Vec<Wallpaper> =
            reddit::get_subreddit_wallpapers(&sr, &config.fetch_type, config.amount).await?;
        wallpapers.push(posts)
    }

    if wallpapers.is_empty() {
        println!("Sorry, there's no more wallpapers to download");
        process::exit(1);
    }

    // shuffle collected wallpapers
    let mut wallpapers: Vec<Wallpaper> = wallpapers.into_iter().flatten().collect();
    wallpapers.shuffle(&mut rand::thread_rng());

    // process wallpaper batch
    let _batch_result = process_wallpaper_batch(wallpapers, &config).await;
    println!("");

    Ok(())
}
