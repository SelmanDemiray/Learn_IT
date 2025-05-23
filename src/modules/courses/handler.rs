use actix_web::{web, HttpResponse, Result};
use tera::Tera;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::utils::markdown::convert_markdown_to_html;

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
) -> Result<HttpResponse> {
    // Read courses from JSON file or create default courses if file doesn't exist
    let courses = if Path::new("content/courses.json").exists() {
        let courses_json = fs::read_to_string("content/courses.json")
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read courses data"))?;
        
        serde_json::from_str::<Vec<Course>>(&courses_json)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to parse courses data"))?
    } else {
        // Create default courses based on available lessons
        create_default_courses()
    };
    
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
    
    let courses = if Path::new("content/courses.json").exists() {
        let courses_json = fs::read_to_string("content/courses.json")
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read courses"))?;
            
        serde_json::from_str::<Vec<Course>>(&courses_json)
            .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to parse courses"))?
    } else {
        create_default_courses()
    };
    
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

// Helper function to create default courses based on available lesson files
fn create_default_courses() -> Vec<Course> {
    let mut courses = Vec::new();
    let levels = vec!["beginner", "intermediate", "advanced"];
    
    for level in &levels {
        let content_path = format!("content/{}", level);
        let mut lessons = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&content_path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(".md") {
                            let lesson_id = file_name.trim_end_matches(".md");
                            lessons.push(lesson_id.to_string());
                        }
                    }
                }
            }
        }
        
        if !lessons.is_empty() {
            courses.push(Course {
                id: format!("{}_fundamentals", level),
                title: format!("{} IT Fundamentals", level.to_uppercase()),
                description: match *level {
                    "beginner" => "Learn the basic concepts and principles of Information Technology".to_string(),
                    "intermediate" => "Build upon your foundational knowledge with more complex IT topics".to_string(),
                    "advanced" => "Master advanced IT concepts and specialized techniques".to_string(),
                    _ => "IT Learning Course".to_string(),
                },
                lessons,
                level: level.to_string(),
                progress: None,
            });
        }
    }
    
    courses
}
