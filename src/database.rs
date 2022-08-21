use crate::wallpaper::Wallpaper;

pub async fn connect() -> Result<sqlx::SqlitePool, sqlx::Error> {
    let db_path = &"file:///tmp/shpalery.db";
    let pool = sqlx::pool::PoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true),
        )
        .await?;

    migrate(&pool).await?;
    Ok(pool)
}

pub async fn migrate(pool: &sqlx::SqlitePool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

pub async fn find_reddit_entry_by_id(
    pool: &sqlx::SqlitePool,
    id: &str,
) -> Result<bool, sqlx::Error> {
    let (row,): (bool,) =
        sqlx::query_as("SELECT EXISTS(SELECT 1 FROM reddit_wallpapers WHERE id = ?)")
            .bind(&id)
            .fetch_one(pool)
            .await?;
    Ok(row)
}

pub async fn insert_reddit_entry(
    pool: &sqlx::SqlitePool,
    wallpaper: &Wallpaper,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let insert = sqlx::query(
        "INSERT INTO reddit_wallpapers (id, name, href, subreddit, hash) values (?, ?, ?, ?, ?)",
    )
    .bind(&wallpaper.id)
    .bind(&wallpaper.name)
    .bind(&wallpaper.href)
    .bind(&wallpaper.hash)
    .bind(&wallpaper.subreddit)
    .execute(pool)
    .await?;
    Ok(insert)
}
