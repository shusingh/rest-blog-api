use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub timestamp: i64,
}

impl Post {
    pub fn new(title: String, content: String) -> Self {
        Post {
            id: Uuid::new_v4(),
            title,
            content,
            timestamp: Utc::now().timestamp(),
        }
    }
}
