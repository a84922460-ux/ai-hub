mod adapters;
mod commands;
mod learning;
mod router;
mod storage;
mod upgrade;
mod utils;

use commands::chat::DbState;
use std::sync::Mutex;
use tauri::Manager;

pub fn run() {
    let app_dir = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("ai-hub");
    std::fs::create_dir_all(&app_dir).expect("无法创建数据目录");
    let db_path = app_dir.join("ai-hub.db");
    let conn = storage::db::initialize(&db_path).expect("数据库初始化失败");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .manage(DbState(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![
            commands::chat::get_conversations,
            commands::chat::get_messages,
            commands::chat::new_conversation,
            commands::chat::send_message,
        ])
        .setup(|_app| Ok(()))
        .run(tauri::generate_context!())
        .expect("启动失败");
}