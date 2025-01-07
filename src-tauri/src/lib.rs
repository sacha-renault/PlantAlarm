mod database;
use database::{
    tauri_db_connect, BackendError, DbConnection, Plant, PlantDto, PlantWithWaterings, Watering,
};
use tauri::{command, Manager, State};

#[command]
async fn add_plant(db: State<'_, DbConnection>, plant_dto: PlantDto) -> Result<i64, BackendError> {
    // Retrieve the pool from managed state
    let pool = db.lock().await;
    Plant::insert_plant(&pool, plant_dto).await
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
        .invoke_handler(tauri::generate_handler![add_plant])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
