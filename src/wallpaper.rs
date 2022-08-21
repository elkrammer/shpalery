use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper {
    pub name: String,
    pub subreddit: String,
    pub href: String,
    pub id: String,
    pub hash: String,
}
