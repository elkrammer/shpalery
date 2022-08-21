mod database;
mod lib;
mod reddit;
mod wallpaper;

use crate::wallpaper::Wallpaper;
use std::path::Path;
use tempfile::Builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    // TODO: make this configurable
    // let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];
    let subreddits = vec!["wallpaper"];

    // TODO: make limit configurable
    for sr in subreddits.into_iter() {
        let posts: Vec<Wallpaper> = reddit::get_subreddit_wallpapers(&sr, 5).await?;
        wallpapers.push(posts)
    }

    let db = database::connect().await?;
    let tmp_dir = Builder::new().prefix("shpalery").rand_bytes(2).tempdir()?;
    let tmp_dir = tmp_dir.path();

    // TODO: make dst_dir configurable
    let dst_dir = Path::new("/home/elkrammer/tmp/wallpapers");
    // TODO: get all file hashes for dst_dir and add comparisson for downloaded tmp wall?

    println!("Temp dir is set to: {}", tmp_dir.display());

    let mut download_count: i32 = 0;
    for mut wall in wallpapers.into_iter().flatten() {
        // check if current item already exists in db - if so then skip it
        if let Ok(true) = database::find_reddit_entry_by_id(&db, &wall.id).await {
            continue;
        }

        // download wallpaper & get file hash
        let w_file = reddit::download_wallpaper(&wall.href, &tmp_dir).await?;
        wall.hash = lib::get_file_hash(&w_file)?;
        println!("Downloading {}", w_file.display());

        // copy file to final destination
        let filename = w_file.file_name().unwrap();
        std::fs::copy(&w_file, dst_dir.join(filename))?;

        // insert file to db
        if let Ok(..) = database::insert_reddit_entry(&db, &wall).await {
            download_count = download_count + 1;
        }
    }
    println!("Total Downloaded Wallpapers: {}", download_count);
    Ok(())
}
