mod database;
mod reddit;
mod wallpaper;

use crate::wallpaper::Wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    // TODO: make this configurable
    let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];

    for sr in subreddits.into_iter() {
        let posts: Vec<Wallpaper> = reddit::get_posts(&sr, 5).await?;
        wallpapers.push(posts)
    }

    let db = database::connect().await?;
    for wall in wallpapers.into_iter().flatten() {
        // TODO:
        // - download wallpaper file
        // get file hash
        // - if hash already in DB get another wallpaper?
        // staging area:
        // - create tmp directory
        // - allow preview
        let _insert = database::insert_reddit_entry(&db, &wall).await?;
    }
    Ok(())
}
