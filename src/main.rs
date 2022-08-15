mod reddit;
mod wallpaper;

// use crate::wallpaper::Wallpaper;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // let mut wall: Vec<Wallpaper> = Vec::new();

    let posts = reddit::get_posts().await?;


    // println!("Name is: {:?}", posts[0].name);
    println!("{:?}", posts);

//     let client = reqwest::Client::builder().build()?;
//     let res = client
//         .get("https://www.reddit.com/r/wallpaper/hot/.json?limit=5")
//         .send()
//         .await?;

//     let json: Value = serde_json::from_str(&res.text().await?)?;
//     let items = json["data"]["children"].as_array().expect("Error parsing response");

//     for item in items {
//         let title = &item["data"]["title"];
//         let url = &item["data"]["url"];

//         let wallpaper = Wallpaper {
//             name: title.to_string(),
//             href: url.to_string(),
//         };

//         wall.push(wallpaper);
//     }


    // let json = res.json::<Listing>().await.unwrap();

    // match json {
    //     Ok(value) => {
    //         println!("{:?}", value.data.children[0].data.url)
    //         Ok(format!("Gucci"))
    //     },
    //     Err(err) => { panic!("Uh oh! Something unexpected happened: {:?}", other) }
    // };

    //     match res.status() {
    //         reqwest::StatusCode::OK => {
    //             println!("All looking gucci thus far {:?}", res);
    //             match res.json::<APIResponse>().await {
    //                 Ok(parsed) => println!("All gucci! {:?}", parsed),
    //                 Err(_) => println!("Invalid response")
    //             };
    //         }

    //         other => {
    //             panic!("Uh oh! Something unexpected happened: {:?}", other);
    //         }
    //     }

    // let data = res.json::<HashMap<String, String>>().await?;

    // println!("{:?}", data);

    Ok(())
}
