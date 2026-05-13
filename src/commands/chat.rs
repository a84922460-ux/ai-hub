use crate::storage::db;
use crate::storage::models::{Conversation, Message, MessageRole};
use tauri::State;
use std::sync::Mutex;
use chrono::Utc;

pub struct DbState(pub Mutex<rusqlite::Connection>);

#[tauri::command]
pub async fn get_conversations(state: State<'_, DbState>) -> Result<Vec<Conversation>, String> {
    let conn = state.0.lock().unwrap();
    db::get_conversations(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_messages(state: State<'_, DbState>, conv_id: i64) -> Result<Vec<Message>, String> {
    let conn = state.0.lock().unwrap();
    db::get_messages(&conn, conv_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn new_conversation(state: State<'_, DbState>, title: String, model_id: String) -> Result<Conversation, String> {
    let conn = state.0.lock().unwrap();
    let conv = Conversation {
        id: 0,
        title,
        model_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        is_starred: false,
        tags: vec![],
        system_prompt: None,
    };
    db::insert_conversation(&conn, &conv).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_message(state: State<'_, DbState>, conv_id: i64, content: String) -> Result<Message, String> {
    // 保存用户消息
    let user_msg = Message {
        id: 0,
        conversation_id: conv_id,
        role: MessageRole::User,
        content: content.clone(),
        model_id: None,
        latency_ms: None,
        token_usage: None,
        created_at: Utc::now(),
        is_liked: None,
    };
    let conn = state.0.lock().unwrap();
    db::insert_message(&conn, &user_msg).map_err(|e| e.to_string())?;

    // 模拟 AI 回复（避免网络和复杂依赖）
    let reply = format!("你发送了: {}\n（这是模拟回复，正式版会接入真实模型）", content);
    let assistant_msg = Message {
        id: 0,
        conversation_id: conv_id,
        role: MessageRole::Assistant,
        content: reply,
        model_id: Some("模拟模型".to_string()),
        latency_ms: Some(0),
        token_usage: None,
        created_at: Utc::now(),
        is_liked: None,
    };
    db::insert_message(&conn, &assistant_msg).map_err(|e| e.to_string())
}
