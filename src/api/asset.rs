use super::author::Author;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub url: String,
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub label: Option<String>,
    pub uploader: Author,
    pub content_type: String,
    pub state: String,
    pub size: u64,
    pub download_count: i64,
    pub created_at: String,
    pub updated_at: String,
    pub browser_download_url: String,
}
