use std::time::Duration;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub async fn db_connection() -> Result<DatabaseConnection, DbErr> {
    let mut opt =
        ConnectOptions::new("postgres://postgres:toujours_skateboarding@localhost:5432".to_owned());
    opt.acquire_timeout(Duration::from_secs(10))
        .sqlx_logging(true);

    let db: DatabaseConnection = Database::connect(opt)
        .await
        .expect("Not connected to the database.");

    dbg!(Ok(db))
    // Ok(db)
}
