use crate::storage::db;
use crate::storage::models::{Conversation, Message};
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

pub struct DbState(pub Mutex<Connection>);

#[tauri::command]
pub async fn get_conversations(db: State<'_, DbState>) -> Result<Vec<Conversation>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, model_id, created_at, updated_at, is_starred, tags FROM conversations ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                model_id: row.get(2)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(3)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
                is_starred: row.get::<_, i32>(5)? != 0,
                tags: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            })
        })
        .map_err(|e| e.to_string())?;
    let mut conversations = Vec::new();
    for row in rows {
        conversations.push(row.map_err(|e| e.to_string())?);
    }
    Ok(conversations)
}

#[tauri::command]
pub async fn get_messages(
    db: State<'_, DbState>,
    conv_id: i64,
) -> Result<Vec<Message>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, conversation_id, role, content, model_id, latency_ms, token_usage, created_at FROM messages WHERE conversation_id = ?1 ORDER BY created_at ASC")
        .map_err(|e| e.to_string())?;
    let rows = stmt
        .query_map([conv_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                model_id: row.get(4)?,
                latency_ms: row.get(5)?,
                token_usage: row.get(6)?,
                created_at: chrono::DateTime::parse_from_rfc3339(&row.get::<_, String>(7)?)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
        })
        .map_err(|e| e.to_string())?;
    let mut messages = Vec::new();
    for row in rows {
        messages.push(row.map_err(|e| e.to_string())?);
    }
    Ok(messages)
}

#[tauri::command]
pub async fn new_conversation(
    db: State<'_, DbState>,
    title: String,
    model_id: String,
) -> Result<Conversation, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO conversations (title, model_id) VALUES (?1, ?2)",
        rusqlite::params![title, model_id],
    )
    .map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(Conversation {
        id,
        title,
        model_id,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        is_starred: false,
        tags: vec![],
    })
}

#[tauri::command]
pub async fn send_message(
    db: State<'_, DbState>,
    conv_id: i64,
    content: String,
) -> Result<Message, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO messages (conversation_id, role, content, created_at) VALUES (?1, 'user', ?2, ?3)",
        rusqlite::params![conv_id, content, now],
    )
    .map_err(|e| e.to_string())?;
    let msg_id = conn.last_insert_rowid();
    Ok(Message {
        id: msg_id,
        conversation_id: conv_id,
        role: "user".into(),
        content,
        model_id: None,
        latency_ms: None,
        token_usage: None,
        created_at: chrono::Utc::now(),
    })
}

// 后续可在升级版本中加入真正的AI回复插入逻辑
