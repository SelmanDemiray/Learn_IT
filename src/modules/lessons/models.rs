use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Lesson {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub preview: String,
    pub level: String,
}
