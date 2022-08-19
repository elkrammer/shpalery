mod database;
mod reddit;
mod wallpaper;

use tempfile::Builder;

use crate::wallpaper::Wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    // TODO: make this configurable
    let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];

    for sr in subreddits.into_iter() {
        let posts: Vec<Wallpaper> = reddit::get_subreddit_wallpapers(&sr, 5).await?;
        wallpapers.push(posts)
    }

    let db = database::connect().await?;
    let mut download_count: i32 = 0;
    let tmp_dir = Builder::new().prefix("shpalery").rand_bytes(2).tempdir()?;
    let tmp_dir = tmp_dir.path();
    println!("Temp dir is set to: {}", tmp_dir.display());

    for wall in wallpapers.into_iter().flatten() {
        // TODO:
        // - download wallpaper file
        // get file hash
        // - if hash already in DB get another wallpaper?
        // staging area:
        // - create tmp directory
        // - allow preview

        let _download = reddit::download_wallpaper(&wall.href, &tmp_dir).await?;
        let insert_to_db = database::insert_reddit_entry(&db, &wall).await?;
        if insert_to_db {
            download_count = download_count + 1;
        }
    }
    println!("Total Downloaded Wallpapers: {}", download_count);

    Ok(())
}
