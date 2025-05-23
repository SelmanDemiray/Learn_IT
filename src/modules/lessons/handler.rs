use actix_web::{web, HttpResponse, Result};
use tera::Tera;
use std::fs;
use std::path::Path;
use crate::utils::markdown::convert_markdown_to_html;
use crate::modules::lessons::models::Lesson;

// Helper function to convert filename to readable title
fn filename_to_title(filename: &str) -> String {
    filename
        .trim_end_matches(".md")
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub async fn get_lessons(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "All Lessons");
    
    let levels = vec!["beginner", "intermediate", "advanced"];
    let mut all_levels_with_lessons = Vec::new();
    
    for level in &levels {
        let content_path = format!("content/{}", level);
        let mut lessons = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&content_path) {
            let mut lesson_files: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.file_name().to_str()
                        .map(|name| name.ends_with(".md"))
                        .unwrap_or(false)
                })
                .collect();
            
            // Sort lessons alphabetically
            lesson_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
            
            for entry in lesson_files {
                if let Some(file_name) = entry.file_name().to_str() {
                    let id = file_name.trim_end_matches(".md");
                    let title = filename_to_title(file_name);
                    
                    // Read content for preview
                    let file_path = format!("{}/{}", content_path, file_name);
                    let content = fs::read_to_string(&file_path).unwrap_or_default();
                    
                    // Create a better preview by taking first paragraph
                    let preview = content
                        .lines()
                        .skip_while(|line| line.trim().is_empty() || line.starts_with('#'))
                        .take_while(|line| !line.trim().is_empty())
                        .collect::<Vec<_>>()
                        .join(" ")
                        .chars()
                        .take(150)
                        .collect::<String>()
                        + if content.len() > 150 { "..." } else { "" };
                    
                    lessons.push(Lesson {
                        id: id.to_string(),
                        title: title.clone(),
                        level: level.to_string(),
                        content: content.clone(),
                        preview: preview
                    });
                }
            }
        }
        
        all_levels_with_lessons.push(serde_json::json!({
            "name": level,
            "lessons": lessons
        }));
    }
    
    context.insert("levels", &all_levels_with_lessons);
    
    let rendered = tmpl.render("lessons/index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn get_lessons_by_level(
    tmpl: web::Data<Tera>,
    path: web::Path<String>
) -> Result<HttpResponse> {
    let level = path.into_inner();
    
    // Validate level
    if !["beginner", "intermediate", "advanced"].contains(&level.as_str()) {
        return Ok(HttpResponse::NotFound().body("Level not found"));
    }
    
    let content_path = format!("content/{}", level);
    let mut lessons = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&content_path) {
        let mut lesson_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name().to_str()
                    .map(|name| name.ends_with(".md"))
                    .unwrap_or(false)
            })
            .collect();
        
        // Sort lessons alphabetically
        lesson_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        
        for entry in lesson_files {
            if let Some(file_name) = entry.file_name().to_str() {
                let id = file_name.trim_end_matches(".md");
                let title = filename_to_title(file_name);
                let file_path = format!("{}/{}", content_path, file_name);
                let content = fs::read_to_string(&file_path).unwrap_or_default();
                
                let preview = content
                    .lines()
                    .skip_while(|line| line.trim().is_empty() || line.starts_with('#'))
                    .take_while(|line| !line.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .chars()
                    .take(150)
                    .collect::<String>()
                    + if content.len() > 150 { "..." } else { "" };
                
                lessons.push(Lesson {
                    id: id.to_string(),
                    title: title.clone(),
                    level: level.to_string(),
                    content: content.clone(),
                    preview: preview
                });
            }
        }
    }
    
    let mut context = tera::Context::new();
    context.insert("title", &format!("{} Lessons", level.chars().next().unwrap().to_uppercase().to_string() + &level[1..]));
    context.insert("level", &level);
    context.insert("lessons", &lessons);
    
    let rendered = tmpl.render("lessons/level.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn get_lesson(
    tmpl: web::Data<Tera>,
    path: web::Path<(String, String)>
) -> Result<HttpResponse> {
    let (level, id) = path.into_inner();
    
    // Validate level
    if !["beginner", "intermediate", "advanced"].contains(&level.as_str()) {
        return Ok(HttpResponse::NotFound().body("Level not found"));
    }
    
    let file_path = format!("content/{}/{}.md", level, id);
    
    if !Path::new(&file_path).exists() {
        return Ok(HttpResponse::NotFound().body("Lesson not found"));
    }
    
    let content = fs::read_to_string(&file_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read lesson content"))?;
    
    // Convert markdown to HTML with syntax highlighting
    let html_content = convert_markdown_to_html(&content);
    
    // Get other lessons in the same level for navigation
    let content_path = format!("content/{}", level);
    let mut lessons = Vec::new();
    
    if let Ok(entries) = fs::read_dir(&content_path) {
        let mut lesson_files: Vec<_> = entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry.file_name().to_str()
                    .map(|name| name.ends_with(".md"))
                    .unwrap_or(false)
            })
            .collect();
        
        // Sort lessons alphabetically
        lesson_files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
        
        for entry in lesson_files {
            if let Some(file_name) = entry.file_name().to_str() {
                let lesson_id = file_name.trim_end_matches(".md");
                let title = filename_to_title(file_name);
                let file_path = format!("{}/{}", content_path, file_name);
                let content = fs::read_to_string(&file_path).unwrap_or_default();
                
                let preview = content
                    .lines()
                    .skip_while(|line| line.trim().is_empty() || line.starts_with('#'))
                    .take_while(|line| !line.trim().is_empty())
                    .collect::<Vec<_>>()
                    .join(" ")
                    .chars()
                    .take(150)
                    .collect::<String>()
                    + if content.len() > 150 { "..." } else { "" };
                
                lessons.push(Lesson {
                    id: lesson_id.to_string(),
                    title: title.clone(),
                    level: level.to_string(),
                    content: content.clone(),
                    preview: preview
                });
            }
        }
    }
    
    // Find current lesson index for navigation
    let current_index = lessons.iter().position(|l| l.id == id).unwrap_or(0);
    let prev_lesson = if current_index > 0 { Some(&lessons[current_index - 1]) } else { None };
    let next_lesson = if current_index < lessons.len() - 1 { Some(&lessons[current_index + 1]) } else { None };
    
    let title = filename_to_title(&format!("{}.md", id));
    
    let mut context = tera::Context::new();
    context.insert("title", &title);
    context.insert("level", &level);
    context.insert("content", &content);
    context.insert("html_content", &html_content);
    context.insert("prev_lesson", &prev_lesson);
    context.insert("next_lesson", &next_lesson);
    context.insert("all_lessons", &lessons);
    context.insert("course_id", &format!("{}_fundamentals", level)); // For progress tracking
    
    let rendered = tmpl.render("lessons/lesson.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}
