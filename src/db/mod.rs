use sqlx::PgPool;
use std::env;

pub async fn connect_db() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    println!("✅ Database connected");

    pool
}