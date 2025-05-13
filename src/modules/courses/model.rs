use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub lessons: Vec<String>,
    pub level: String,
}
