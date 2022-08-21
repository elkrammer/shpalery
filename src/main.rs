mod database;
mod lib;
mod reddit;
mod wallpaper;

use tempfile::Builder;

use crate::wallpaper::Wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    // TODO: make this configurable
    // let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];
    let subreddits = vec!["wallpaper"];

    for sr in subreddits.into_iter() {
        let posts: Vec<Wallpaper> = reddit::get_subreddit_wallpapers(&sr, 5).await?;
        wallpapers.push(posts)
    }

    let db = database::connect().await?;
    let tmp_dir = Builder::new().prefix("shpalery").rand_bytes(2).tempdir()?;
    let tmp_dir = tmp_dir.path();
    println!("Temp dir is set to: {}", tmp_dir.display());

    let mut download_count: i32 = 0;
    for mut wall in wallpapers.into_iter().flatten() {
        // check if current item already exists in db - if so then skip it
        let exists = database::find_reddit_entry_by_id(&db, &wall.id).await;
        if exists? {
            continue;
        }

        let w_file = reddit::download_wallpaper(&wall.href, &tmp_dir).await?;
        wall.hash = lib::get_file_hash(&w_file)?;

        if let Ok(_insert) = database::insert_reddit_entry(&db, &wall).await {
            println!("Downloading {}", w_file.display());
            download_count = download_count + 1;
        }
    }
    println!("Total Downloaded Wallpapers: {}", download_count);
    Ok(())
}
