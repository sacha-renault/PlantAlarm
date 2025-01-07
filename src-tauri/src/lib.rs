mod database;
use database::{tauri_db_connect, DbConnection};
use tauri::{Manager, State};

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
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
