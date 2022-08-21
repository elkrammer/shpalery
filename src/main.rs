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
    let subreddits = vec!["wallpaper", "EarthPorn"];
    let amount: i32 = 5;

    for sr in subreddits.into_iter() {
        // TODO: allow use of other fetch_types. one of (hour, day, week, month, year, all)
        let posts: Vec<Wallpaper> = reddit::get_subreddit_wallpapers(&sr, "hot", amount).await?;
        wallpapers.push(posts)
    }

    // TODO: shuffle collected wallpapers

    let db = database::connect().await?;
    let tmp_dir = Builder::new().prefix("shpalery").rand_bytes(2).tempdir()?;
    let tmp_dir = tmp_dir.path();

    // TODO: make dst_dir configurable
    let dst_dir = Path::new("/home/elkrammer/tmp/wallpapers");
    // TODO: get all file hashes for dst_dir and add comparisson for downloaded tmp wall?

    println!("Temp dir is set to: {}", tmp_dir.display());

    let mut download_count: i32 = 0;
    for mut wall in wallpapers.into_iter().flatten() {
        println!("Downloading wallpaper {}/{}", download_count, amount);
        // if desired amount of wallpapers is met we can break out of this loop
        if download_count >= amount {
            break;
        }

        // check if current item already exists in db - if so then skip it
        if let Ok(true) = database::find_reddit_entry_by_id(&db, &wall.id).await {
            continue;
        }

        // download wallpaper & get file hash
        let w_file = reddit::download_wallpaper(&wall.href, &tmp_dir).await?;
        wall.hash = lib::get_file_hash(&w_file)?;
        println!("Downloading {} from /r/{}", &wall.name, &wall.subreddit);

        // println!("Wallpaper: {:?}", wall);

        // copy file to final destination
        let filename = w_file.file_name().unwrap();
        std::fs::copy(&w_file, dst_dir.join(filename))?;

        // insert file to db
        if let Ok(..) = database::insert_reddit_entry(&db, &wall).await {
            download_count += 1;
        }
    }
    println!("Total Downloaded Wallpapers: {}", download_count);
    Ok(())
}
