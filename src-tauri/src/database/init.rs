use super::constant::DB_FILE_NAME;
use sqlx::sqlite::SqlitePool;
use sqlx::{Error, Executor};
use std::path::Path;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Manager};

pub type DbConnection = Arc<Mutex<SqlitePool>>;

pub async fn create_tables(pool: &SqlitePool) -> Result<(), Error> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS plant (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        day_interval INTEGER NOT NULL,
        water_quantity INTEGER NOT NULL,
        image TEXT
    );

    CREATE TABLE IF NOT EXISTS watering (
        id INTEGER PRIMARY KEY,
        plant_id INTEGER NOT NULL,
        date_watered DATETIME NOT NULL,
        FOREIGN KEY (plant_id) REFERENCES plant(id) ON DELETE CASCADE
    );
    "#;

    // Execute the query to create the tables
    pool.execute(query).await?;

    Ok(())
}

pub async fn init_db(db_directory_path: &str) -> Result<SqlitePool, Error> {
    // build the db path
    let db_path = Path::new(db_directory_path).join(DB_FILE_NAME);

    // Build the pool
    let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display())).await?;

    // Create table if they don't exist
    create_tables(&pool).await?;

    Ok(pool)
}

pub async fn tauri_db_connect(app: AppHandle) -> Result<DbConnection, Error> {
    // Get the app's data directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Couldn't resolve the resource path.");

    // Ensure the directory exists
    std::fs::create_dir_all(&app_data_dir)?;

    // Connect to the db
    // This will create a file if it doesn't exist
    let pool = init_db(app_data_dir).await?;

    Ok(Arc::new(Mutex::new(pool)))
}