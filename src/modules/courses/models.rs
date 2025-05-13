use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub level: String,
    pub lessons: Vec<Uuid>,
}
