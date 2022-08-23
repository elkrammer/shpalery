mod database;
mod lib;
mod reddit;
mod wallpaper;

use crate::wallpaper::Wallpaper;
use rand::seq::SliceRandom;
use std::io::Write;
use std::path::Path;
use std::process;
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: make this configurable
    // let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];
    let subreddits = vec!["wallpaper", "wallpapers"];
    let amount: i32 = 10;
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    for sr in subreddits.into_iter() {
        // TODO: make fetch_type configurable
        //  one of hot, top(hour, day, week, month, year, all)
        let posts: Vec<Wallpaper> = reddit::get_subreddit_wallpapers(&sr, "year", amount).await?;
        wallpapers.push(posts)
    }

    // shuffle collected wallpapers
    let mut wallpapers: Vec<Wallpaper> = wallpapers.into_iter().flatten().collect();
    wallpapers.shuffle(&mut rand::thread_rng());

    if wallpapers.is_empty() {
        println!("Sorry, there's no more wallpapers to download");
        process::exit(1);
    }

    let db = database::connect().await?;
    let tmp_dir = Builder::new().prefix("shpalery").rand_bytes(2).tempdir()?;
    let tmp_dir = tmp_dir.path();

    // TODO: make dst_dir configurable
    let dst_dir = Path::new("/home/elkrammer/tmp/wallpapers");

    // TODO: get all file hashes for dst_dir and add comparisson for downloaded tmp wall?
    let mut download_count: i32 = 0;
    for mut wall in wallpapers.into_iter() {
        print!("\rDownloading [{}/{}]", download_count, amount);
        std::io::stdout().flush()?;

        // if desired amount of wallpapers is met we can break out of this loop
        if download_count >= amount {
            break;
        }

        // check if current item's id already exists in db - if so then skip it
        if let Ok(true) = database::find_reddit_entry_by_id(&db, &wall.id).await {
            continue;
        }

        // download wallpaper file
        let w_file = reddit::download_wallpaper(&wall.href, &tmp_dir).await?;

        // hash check
        // check if downloaded wallpaper is already present in our inventory
        // the wallpaper we download might be a cross-post to another subreddit - in this case
        // it will have a different id, thus this check
        wall.hash = lib::get_file_hash(&w_file)?;
        if let Ok(true) = database::find_reddit_entry_by_hash(&db, &wall.hash).await {
            println!("Hash {} already present in db", &wall.hash);
            continue;
        }

        // copy file to final destination
        let filename = w_file.file_name().unwrap();
        std::fs::copy(&w_file, dst_dir.join(filename))?;

        // insert file to db
        if let Ok(..) = database::insert_reddit_entry(&db, &wall).await {
            download_count += 1;
        }
    }
    Ok(())
}
