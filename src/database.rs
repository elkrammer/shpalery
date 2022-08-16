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

pub async fn insert_reddit_entry(
    pool: &sqlx::SqlitePool,
    wallpaper: &Wallpaper,
) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO reddit_wallpapers (id, name, href, hash) values (?, ?, ?, ?)")
        .bind(&wallpaper.id)
        .bind(&wallpaper.name)
        .bind(&wallpaper.href)
        .bind(&wallpaper.hash)
        .execute(pool)
        .await
}
