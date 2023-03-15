use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

const DATABASE_URL: &str = "postgres://postgres:toujours_skateboarding@localhost:5432";
const DB_NAME: &str = "toujours_skateboarding_db";
pub async fn db_connection() -> Result<DatabaseConnection, DbErr> {
    let url: String = format!("{}/{}", DATABASE_URL, DB_NAME);
    let mut opt = ConnectOptions::new(url.to_owned());
    opt.acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(true);

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Not connected to the database.");

    dbg!(Ok(db))
    // Ok(db)
}
