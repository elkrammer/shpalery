use crate::wallpaper::Wallpaper;
use serde_json::Value;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub async fn get_subreddit_wallpapers(
    subreddit: &str,
    limit: i32,
) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error>> {
    let mut wallpapers: Vec<Wallpaper> = Vec::new();
    let client = reqwest::Client::builder().build()?;
    let res = client
        .get(format!(
            "https://www.reddit.com/r/{subreddit}/hot/.json?limit={limit}"
        ))
        .send()
        .await?;
    let json: Value = serde_json::from_str(&res.text().await?)?;
    let items = json["data"]["children"]
        .as_array()
        .expect("Error parsing response");

    for item in items {
        let id = &item["data"]["id"];
        let title = &item["data"]["title"];
        let url = &item["data"]["url"];

        let wallpaper = Wallpaper {
            id: id.to_string().replace('"', ""),
            name: title.to_string().replace('"', ""),
            href: url.to_string().replace('"', ""),
            hash: id.to_string().replace('"', ""),
        };

        wallpapers.push(wallpaper);
    }

    Ok(wallpapers)
}

pub async fn download_wallpaper(
    url: &str,
    tmp_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");
        println!("Downloading {}", fname);

        let fname = tmp_dir.join(fname);
        File::create(fname)?
    };

    let content = response.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
