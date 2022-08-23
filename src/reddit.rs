use crate::database;
use crate::lib::string_ends_with_any;
use crate::wallpaper::Wallpaper;
use serde_json::Value;
use std::fs::File;
use std::path::{Path, PathBuf};

pub async fn get_subreddit_wallpapers(
    subreddit: &str,
    fetch_type: &str,
    amount: i32,
) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Wallpaper> = Vec::new();
    let client = reqwest::Client::builder().build()?;
    let db = database::connect().await?;
    let buffer_limit = amount + 30; // get extra wallpapers in case we need to skip items

    let sub_url: String = if fetch_type == "hot" {
        format!("https://www.reddit.com/r/{subreddit}/{fetch_type}/.json?limit={buffer_limit}")
    } else {
        format!(
            "https://www.reddit.com/r/{subreddit}/top/.json?t={fetch_type}&limit={buffer_limit}"
        )
    };

    let res = client.get(sub_url).send().await?;
    let json: Value = serde_json::from_str(&res.text().await?)?;
    let items = json["data"]["children"]
        .as_array()
        .expect("Error parsing response");

    for item in items {
        let id: &str = &item["data"]["id"].to_string().replace('"', "");
        let name: &str = &item["data"]["title"].to_string().replace('"', "");
        let url: &str = &item["data"]["url"].to_string().replace('"', "");

        let valid_image_suffixes = vec!["jpg", "png"];

        // skip item if already in db
        if let Ok(true) = database::find_reddit_entry_by_id(&db, id).await {
            continue;
        }

        // skip item if it's not an image file
        if !(string_ends_with_any(url.to_string(), valid_image_suffixes)) {
            continue;
        }

        let wallpaper = Wallpaper {
            id: id.to_string(),
            name: name.to_string(),
            href: url.to_string(),
            subreddit: subreddit.to_string(),
            hash: "".to_string(),
        };

        wallpapers.push(wallpaper);
    }

    Ok(wallpapers)
}

pub async fn download_wallpaper(
    url: &str,
    tmp_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let wallfile;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        wallfile = tmp_dir.join(fname);
        File::create(wallfile.clone())?
    };
    let mut content = std::io::Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut dest)?;
    Ok(wallfile)
}
