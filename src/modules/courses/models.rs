use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: String,
    pub lessons: Vec<String>, // IDs of lessons in this course
}

impl Course {
    pub fn new(id: String, title: String, description: String, level: String) -> Self {
        Course {
            id,
            title,
            description,
            level,
            lessons: Vec::new(),
        }
    }
}
