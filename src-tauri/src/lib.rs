mod database;
mod parsing;

use chrono::NaiveDateTime;
use database::{
    tauri_db_connect, BackendError, DbConnection, Plant, PlantDto, PlantWithWaterings, Watering,
};
use parsing::date::{parse_date_str, parse_option_date_string};
use tauri::{command, Manager, State};

#[command]
async fn add_plant(
    db: State<'_, DbConnection>,
    plant_dto: PlantDto,
) -> Result<Plant, BackendError> {
    // Retrieve the pool from managed state
    let pool = db.lock().await;
    Plant::insert_plant(&pool, plant_dto).await
}

#[command]
async fn get_all_plants(db: State<'_, DbConnection>) -> Result<Vec<Plant>, BackendError> {
    // Retrieve the pool from managed state
    let pool = db.lock().await;
    Plant::get_plants(&pool).await
}

#[command]
async fn add_watering(
    db: State<'_, DbConnection>,
    plant_id: i64,
    date: String,
) -> Result<(), BackendError> {
    // Parse the date
    let parsed_date = parse_date_str(date)
        .map_err(|e| BackendError::DateError(format!("Failed to parse date: {}", e)))?;

    // Retrieve the pool from managed state
    let pool = db.lock().await;
    Watering::insert_watering(&pool, plant_id, parsed_date).await
}

#[command]
async fn get_all_plant_with_watering(
    db: State<'_, DbConnection>,
    date: Option<String>,
    offset: Option<u64>,
) -> Result<Vec<PlantWithWaterings>, BackendError> {
    // parse date
    let parsed_date = parse_option_date_string(date)
        .map_err(|e| BackendError::DateError(format!("Failed to parse date: {}", e)))?;

    // Retrieve the pool from managed state
    let pool = db.lock().await;
    PlantWithWaterings::get_plants_with_recent_waterings(&pool, Some(parsed_date), offset).await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let db = rt.block_on(tauri_db_connect(app.handle().clone()))?;
            app.manage(db);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_plant,
            get_all_plants,
            add_watering,
            get_all_plant_with_watering
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
