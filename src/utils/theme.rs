use std::sync::RwLock;
use std::collections::HashMap;
use once_cell::sync::Lazy;

// Define available themes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

// Global theme storage with user preferences
static USER_THEMES: Lazy<RwLock<HashMap<String, Theme>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

pub fn get_user_theme(user_id: &str) -> Theme {
    let themes = USER_THEMES.read().unwrap();
    themes.get(user_id).copied().unwrap_or(Theme::System)
}

pub fn set_user_theme(user_id: &str, theme: Theme) {
    let mut themes = USER_THEMES.write().unwrap();
    themes.insert(user_id.to_string(), theme);
}

// CSS class helpers for theming elements
pub fn get_theme_class(theme: Theme) -> String {
    match theme {
        Theme::Light => "theme-light".to_string(),
        Theme::Dark => "theme-dark".to_string(),
        Theme::System => "theme-system".to_string(),
    }
}
