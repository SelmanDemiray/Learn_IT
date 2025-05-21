use actix_web::{web, HttpResponse, Result};
use actix_identity::Identity;
use sqlx::SqlitePool;
use tera::Tera;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::utils::markdown::convert_markdown_to_html;
use crate::modules::users::model::Progress;

#[derive(Serialize, Deserialize, Debug)]
pub struct Course {
    id: String,
    title: String,
    description: String,
    lessons: Vec<String>,
    level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    progress: Option<f64>,
}

pub async fn get_courses(
    tmpl: web::Data<Tera>,
    db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Read courses from JSON file
    let courses_json = fs::read_to_string("content/courses.json")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read courses data"))?;
    
    // Parse courses
    let mut courses: Vec<Course> = serde_json::from_str(&courses_json)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to parse courses data"))?;
    
    // Get user ID if logged in
    let user_id = id.id().ok();
    
    // If user is logged in, fetch progress data
    if let Some(user_id) = &user_id {
        // In a real app, fetch actual progress from database
        // For now, we'll use dummy data
        let progress_data = get_dummy_progress(user_id);
        
        // Update course objects with progress data
        for course in &mut courses {
            if let Some(progress) = progress_data.iter().find(|p| p.lesson_id.starts_with(&course.id)) {
                // In a real app, calculate actual percentage based on completed lessons
                course.progress = Some(if progress.completed { 50.0 } else { 25.0 });
            }
        }
    }
    
    // Separate courses by level
    let beginner_courses: Vec<&Course> = courses.iter().filter(|c| c.level == "beginner").collect();
    let intermediate_courses: Vec<&Course> = courses.iter().filter(|c| c.level == "intermediate").collect();
    let advanced_courses: Vec<&Course> = courses.iter().filter(|c| c.level == "advanced").collect();
    
    let mut context = tera::Context::new();
    context.insert("beginner_courses", &beginner_courses);
    context.insert("intermediate_courses", &intermediate_courses);
    context.insert("advanced_courses", &advanced_courses);
    
    let rendered = tmpl.render("courses/index.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn get_course(
    tmpl: web::Data<Tera>,
    path: web::Path<String>
) -> Result<HttpResponse> {
    let id = path.into_inner();
    let courses_path = "content/courses.json";
    
    let courses_json = fs::read_to_string(courses_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read courses"))?;
        
    let courses: Vec<Course> = serde_json::from_str(&courses_json)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to parse courses"))?;
    
    let course = courses.iter().find(|c| c.id == id);
    
    match course {
        Some(course) => {
            // Get lesson details for this course
            let mut lessons = Vec::new();
            for lesson_id in &course.lessons {
                let file_path = format!("content/{}/{}.md", course.level, lesson_id);
                if Path::new(&file_path).exists() {
                    let content = fs::read_to_string(&file_path).unwrap_or_default();
                    let title = lesson_id.replace("_", " ");
                    let preview = if content.len() > 150 { 
                        format!("{}...", &content[0..150]) 
                    } else { 
                        content.clone() 
                    };
                    
                    lessons.push(serde_json::json!({
                        "id": lesson_id,
                        "title": title,
                        "preview": preview,
                        "html_preview": convert_markdown_to_html(&preview)
                    }));
                }
            }
            
            let mut context = tera::Context::new();
            context.insert("title", &course.title);
            context.insert("course", &course);
            context.insert("lessons", &lessons);
            
            let rendered = tmpl.render("courses/course.html", &context)
                .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
                
            Ok(HttpResponse::Ok().body(rendered))
        },
        None => Ok(HttpResponse::NotFound().body("Course not found"))
    }
}

// Dummy function to simulate progress data - in a real app, this would query the database
fn get_dummy_progress(user_id: &str) -> Vec<Progress> {
    vec![
        Progress {
            id: Some(1),
            user_id: user_id.to_string(),
            lesson_id: "intro_to_it".to_string(),
            completed: true,
            completed_at: Some(chrono::Utc::now()),
        },
        Progress {
            id: Some(2),
            user_id: user_id.to_string(),
            lesson_id: "programming_basics".to_string(),
            completed: false,
            completed_at: None,
        },
    ]
}
