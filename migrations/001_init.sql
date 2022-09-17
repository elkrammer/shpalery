CREATE TABLE IF NOT EXISTS reddit_wallpapers (
    id VARCHAR(10) PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    href VARCHAR (500) NOT NULL,
    subreddit VARCHAR(10) NOT NULL,
    hash VARCHAR(64) NOT NULL,
    download_date TEXT
);
