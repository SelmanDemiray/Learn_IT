use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub level: String,
}
