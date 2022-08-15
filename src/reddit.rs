use crate::wallpaper::Wallpaper;
use serde_json::Value;

pub async fn get_posts(
    subreddit: &str,
    limit: i32,
) -> Result<Vec<Wallpaper>, Box<dyn std::error::Error>> {
    let mut wall: Vec<Wallpaper> = Vec::new();
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
        let title = &item["data"]["title"];
        let url = &item["data"]["url"];

        let wallpaper = Wallpaper {
            name: title.to_string(),
            href: url.to_string(),
        };

        wall.push(wallpaper);
    }

    Ok(wall)
}
