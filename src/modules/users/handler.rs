use actix_web::{web, HttpResponse, Result};
use actix_identity::{Identity, error::GetIdentityError};
use sqlx::SqlitePool;
use tera::Tera;
use serde::{Deserialize, Serialize};

use super::model::{User, Progress, UserProfile};

pub async fn get_profile(
    tmpl: web::Data<Tera>,
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Check if user is authenticated
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    // In a real app, get user from database
    let user = User {
        id: user_id,
        username: "demo_user".to_string(),
        email: "user@example.com".to_string(),
        password_hash: "".to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    // Get user progress
    let completed_lessons = 5; // Would be from DB in real app
    let total_lessons = 20;   // Would be counted from files
    
    let profile = UserProfile {
        user,
        completed_lessons,
        total_lessons,
        completion_percentage: if total_lessons > 0 {
            (completed_lessons as f64 / total_lessons as f64) * 100.0
        } else {
            0.0
        },
    };
    
    let mut context = tera::Context::new();
    context.insert("title", "Your Profile");
    context.insert("profile", &profile);
    
    let rendered = tmpl.render("users/profile.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

pub async fn get_progress(
    tmpl: web::Data<Tera>,
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Check if user is authenticated
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    // Placeholder progress data
    let progress_data = vec![
        Progress {
            id: Some(1),
            user_id: user_id.clone(),
            lesson_id: "introduction_to_it".to_string(),
            completed: true,
            completed_at: Some(chrono::Utc::now()),
        },
        Progress {
            id: Some(2),
            user_id,
            lesson_id: "computer_basics".to_string(),
            completed: true,
            completed_at: Some(chrono::Utc::now()),
        },
    ];
    
    let mut context = tera::Context::new();
    context.insert("title", "Your Learning Progress");
    context.insert("progress", &progress_data);
    
    let rendered = tmpl.render("users/progress.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Add new structs for progress update API
#[derive(Deserialize)]
pub struct ProgressUpdateRequest {
    courseId: String,
    lessonId: String,
    completed: bool,
}

#[derive(Serialize)]
pub struct ProgressUpdateResponse {
    success: bool,
    progress: f64,
    message: String,
}

// New API endpoint to update progress
pub async fn update_progress(
    req: web::Json<ProgressUpdateRequest>,
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Check if user is authenticated
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    // In a real app, save progress to database
    // Here we'll just simulate the update
    let course_id = &req.courseId;
    let lesson_id = &req.lessonId;
    let completed = req.completed;
    
    log::info!("Progress update for user {}: Course {}, Lesson {}, Completed: {}", 
        user_id, course_id, lesson_id, completed);
        
    // Calculate simulated progress based on lesson completion
    // In a real app, this would query the database for actual completion percentage
    let progress = if completed { 20.0 } else { 0.0 };
    
    // Return the updated progress information
    Ok(HttpResponse::Ok().json(ProgressUpdateResponse {
        success: true,
        progress,
        message: format!("Progress updated for lesson {} in course {}", lesson_id, course_id),
    }))
}
