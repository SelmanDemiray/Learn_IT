use actix_web::{web, HttpResponse, Result};
use std::fs;
use std::path::Path;
use tera::Tera;
use serde_json::from_str;
use crate::utils::markdown::convert_markdown_to_html;

use super::model::Course;

pub async fn get_courses(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let courses_path = "content/courses.json";
    let courses_json = fs::read_to_string(courses_path)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to read courses"))?;
        
    let courses: Vec<Course> = from_str(&courses_json)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Failed to parse courses"))?;
    
    // Group courses by level for better display
    let mut beginner_courses = Vec::new();
    let mut intermediate_courses = Vec::new();
    let mut advanced_courses = Vec::new();
    
    for course in &courses {
        match course.level.as_str() {
            "beginner" => beginner_courses.push(course),
            "intermediate" => intermediate_courses.push(course),
            "advanced" => advanced_courses.push(course),
            _ => {}
        }
    }
    
    let mut context = tera::Context::new();
    context.insert("title", "All Courses");
    context.insert("courses", &courses);
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
        
    let courses: Vec<Course> = from_str(&courses_json)
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
