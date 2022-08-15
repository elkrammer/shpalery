mod reddit;
mod wallpaper;

use crate::wallpaper::Wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subreddits = vec!["wallpaper", "wallpapers", "EarthPorn", "SkyPorn"];
    let mut wallpapers: Vec<Vec<Wallpaper>> = Vec::new();

    for sr in subreddits.into_iter() {
        let posts: Vec<Wallpaper> = reddit::get_posts(&sr, 5).await?;
        wallpapers.push(posts)
    }

    // for v in wallpapers {
    //     println!("{:?}", v);
    // }
    Ok(())
}
