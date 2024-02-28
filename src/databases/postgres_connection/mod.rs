use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn postgres_connection() -> Pool<Postgres> {
  let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

  let pool = PgPoolOptions::new() 
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Error connecting to database");

    let check_migrate = sqlx::migrate!("src/databases/postgres_connection/migrations")
    .run(&pool)
    .await;

  match check_migrate {
    Ok(_) => println!("Database migrated successfully"),
    Err(e) => println!("Error migrating database: {:?}", e),
  }

  pool
}