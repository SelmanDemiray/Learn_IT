use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub level: String,
    pub content: String,
    pub preview: String,
}

impl Lesson {
    pub fn new(id: String, title: String, level: String, content: String) -> Self {
        // Create a short preview from the content
        let preview = if content.len() > 150 {
            format!("{}...", &content[0..150])
        } else {
            content.clone()
        };
        
        Lesson {
            id,
            title,
            level,
            content,
            preview,
        }
    }
}
