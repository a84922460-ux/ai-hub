use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeItem {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub source_url: Option<String>,
    pub quality_score: f32,
}

pub fn insert(conn: &Connection, title: &str, content: &str, url: Option<&str>) -> Result<i64, rusqlite::Error> {
    conn.execute(
        "INSERT INTO knowledge_items (title, content, source_url) VALUES (?1, ?2, ?3)",
        rusqlite::params![title, content, url],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn search(conn: &Connection, query: &str) -> Result<Vec<KnowledgeItem>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, source_url, quality_score FROM knowledge_items WHERE content LIKE ?1 LIMIT 20"
    )?;
    let rows = stmt.query_map([format!("%{}%", query)], |row| {
        Ok(KnowledgeItem {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            source_url: row.get(3)?,
            quality_score: row.get(4)?,
        })
    })?;
    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }
    Ok(items)
}
