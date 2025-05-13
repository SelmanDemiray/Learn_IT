use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub app_name: String,
    pub version: String,
    pub description: String,
    pub contact_email: String,
    pub features: Features,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Features {
    pub enable_auth: bool,
    pub enable_progress_tracking: bool,
    pub enable_comments: bool,
    pub enable_dark_mode: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "IT Learning Platform".to_string(),
            version: "1.0.0".to_string(),
            description: "Learn IT from beginner to expert level".to_string(),
            contact_email: "contact@itlearning.example".to_string(),
            features: Features {
                enable_auth: true,
                enable_progress_tracking: true,
                enable_comments: true,
                enable_dark_mode: true,
            },
        }
    }
}
