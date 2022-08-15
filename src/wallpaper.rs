use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper {
    pub name: String,
    pub href: String,
}
