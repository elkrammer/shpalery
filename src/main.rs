use crate::wallpaper::Wallpaper;
use rand::seq::SliceRandom;
use std::process;
use wallpaper::process_wallpaper_batch;

use crate::config::Config;

mod config;
mod database;
mod lib;
mod reddit;
mod wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();
    let config = Config::load();

    Config::print_config();

    for sr in config.subreddits.into_iter() {
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
    let _batch_result = process_wallpaper_batch(wallpapers).await;

    Ok(())
}
