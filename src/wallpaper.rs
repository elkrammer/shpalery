use serde::{Deserialize, Serialize};
use std::io::Write;
use tempfile::Builder;

use crate::config::Config;
use crate::database;
use crate::lib;
use crate::reddit;

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper {
    pub name: String,
    pub subreddit: String,
    pub href: String,
    pub id: String,
    pub hash: String,
}

pub async fn process_wallpaper_batch(mut wallpapers: Vec<Wallpaper>) -> Vec<Wallpaper> {
    let config = Config::load();
    let db = database::connect()
        .await
        .expect("Error connecting to the database");
    let tmp_dir = Builder::new()
        .prefix("shpalery")
        .rand_bytes(2)
        .tempdir()
        .expect("Error getting tmp_dir");
    let tmp_dir = tmp_dir.path();
    let dst_dir = config.download_dir;
    let mut download_count: i32 = 0;

    for mut wall in wallpapers.iter_mut() {
        print!("\rDownloading [{}/{}]", download_count, config.amount);
        std::io::stdout().flush().expect("Error flushing stdout");

        // if desired amount of wallpapers is met we can break out of this loop
        if download_count >= config.amount {
            break;
        }

        // check if current item's id already exists in db - if so then skip it
        if let Ok(true) = database::find_reddit_entry_by_id(&db, &wall.id).await {
            continue;
        }

        // download wallpaper file
        let w_file = reddit::download_wallpaper(&wall.href, &tmp_dir)
            .await
            .expect("Error downloading wallpaper");

        // hash check
        // check if downloaded wallpaper is already present in our inventory
        // the wallpaper we download might be a cross-post to another subreddit - in this case
        // it will have a different id, thus this check
        wall.hash = lib::get_file_hash(&w_file).expect("Error getting hash for file");
        if let Ok(true) = database::find_reddit_entry_by_hash(&db, &wall.hash).await {
            println!("Hash {} already present in db", &wall.hash);
            continue;
        }

        // copy file to final destination
        let filename = w_file.file_name().unwrap();
        std::fs::copy(&w_file, dst_dir.join(filename))
            .expect("Error copying file to wallpapers data dir");

        // insert file to db
        if let Ok(..) = database::insert_reddit_entry(&db, &wall).await {
            download_count += 1;
        }
    }
    return wallpapers;
}
