use scraper::{Html, Selector};
use std::error::Error;

/// 简单的网页抓取与文本提取（遵守 robots.txt 由调用方控制）
pub async fn fetch_text(url: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .user_agent("ai-hub/0.1")
        .build()?;
    let body = client.get(url).send().await?.text().await?;
    let doc = Html::parse_document(&body);
    let sel = Selector::parse("p, h1, h2, h3, li").unwrap();
    let text: Vec<_> = doc.select(&sel).map(|el| el.text().collect::<String>()).collect();
    Ok(text.join("\n"))
}
