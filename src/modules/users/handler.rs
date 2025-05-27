use actix_web::{web, HttpResponse, Result};
use actix_identity::Identity;
use sqlx::SqlitePool;
use tera::Tera;
use serde::{Deserialize, Serialize};
use chrono::Utc;

use super::model::{User, Progress, UserProfile, UserProgress, Achievement, LessonProgress};

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
    
    let course_id = &req.courseId;
    let lesson_id = &req.lessonId;
    let completed = req.completed;
    
    log::info!("Progress update for user {}: Course {}, Lesson {}, Completed: {}", 
        user_id, course_id, lesson_id, completed);
    
    // Get current user progress
    let mut user_progress = UserProgress::new(user_id.clone());
    
    // Simulate existing progress for demo
    user_progress.beginner_percentage = 50.0;
    user_progress.intermediate_percentage = 10.0;
    
    // Update lesson progress and calculate stage completion
    let stage_progress_change = if completed { 25.0 } else { -25.0 };
    
    // Determine which stage this lesson belongs to and update accordingly
    match course_id.as_str() {
        course if course.contains("beginner") => {
            user_progress.beginner_percentage = (user_progress.beginner_percentage + stage_progress_change).max(0.0).min(100.0);
            user_progress.check_stage_completion("beginner");
        },
        course if course.contains("intermediate") => {
            user_progress.intermediate_percentage = (user_progress.intermediate_percentage + stage_progress_change).max(0.0).min(100.0);
            user_progress.check_stage_completion("intermediate");
        },
        course if course.contains("advanced") => {
            user_progress.advanced_percentage = (user_progress.advanced_percentage + stage_progress_change).max(0.0).min(100.0);
            user_progress.check_stage_completion("advanced");
        },
        _ => {
            user_progress.expert_percentage = (user_progress.expert_percentage + stage_progress_change).max(0.0).min(100.0);
            user_progress.check_stage_completion("expert");
        }
    }
    
    user_progress.calculate_overall_progress();
    
    // Check for new achievements
    let mut new_achievements = Vec::new();
    if completed {
        // Award flower for lesson completion
        user_progress.award_flower("lesson_flower");
        
        // Check for "first lesson" achievement
        if user_progress.flowers_earned == 1 {
            new_achievements.push("First Bloom! 🌸".to_string());
        }
        
        // Check for stage completion achievements
        if user_progress.beginner_completed {
            new_achievements.push("Beginner Master! 🌷".to_string());
        }
        if user_progress.intermediate_completed {
            new_achievements.push("Growing Expert! 🌻".to_string());
        }
        if user_progress.advanced_completed {
            new_achievements.push("Advanced Gardener! 🌺".to_string());
        }
    }
    
    // Calculate course progress based on lessons completed
    let lessons_per_course = 4; // Assuming 4 lessons per course
    let course_progress = (user_progress.beginner_percentage / 100.0 * lessons_per_course as f64) / lessons_per_course as f64 * 100.0;
    
    // Return enhanced response with flower rewards
    Ok(HttpResponse::Ok().json(EnhancedProgressResponse {
        success: true,
        progress: course_progress.min(100.0),
        stage_progress: match course_id.as_str() {
            course if course.contains("beginner") => user_progress.beginner_percentage,
            course if course.contains("intermediate") => user_progress.intermediate_percentage,
            course if course.contains("advanced") => user_progress.advanced_percentage,
            _ => user_progress.expert_percentage,
        },
        flowers_earned: if completed { 1 } else { 0 },
        new_achievements,
        message: format!("Progress updated for lesson {} in course {}", lesson_id, course_id),
    }))
}

#[derive(Serialize)]
pub struct EnhancedProgressResponse {
    success: bool,
    progress: f64,
    stage_progress: f64,
    flowers_earned: i32,
    new_achievements: Vec<String>,
    message: String,
}

// New roadmap handler
pub async fn get_roadmap(
    tmpl: web::Data<Tera>,
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Check if user is authenticated
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    // Get user progress (in a real app, this would come from the database)
    let mut user_progress = UserProgress::new(user_id.clone());
    
    // Simulate some progress for demo
    user_progress.beginner_percentage = 75.0;
    user_progress.intermediate_percentage = 30.0;
    user_progress.advanced_percentage = 0.0;
    user_progress.expert_percentage = 0.0;
    user_progress.flowers_earned = 8;
    user_progress.badges_earned = 3;
    user_progress.streak_days = 5;
    user_progress.calculate_overall_progress();
    
    // Get lesson progress for each stage
    let beginner_lessons = get_stage_lessons("beginner", &user_progress);
    let intermediate_lessons = get_stage_lessons("intermediate", &user_progress);
    let advanced_lessons = get_stage_lessons("advanced", &user_progress);
    
    // Get user achievements
    let mut user_achievements = Achievement::get_default_achievements();
    
    // Mark some achievements as earned for demo
    for achievement in &mut user_achievements {
        match achievement.id.as_str() {
            "first_lesson" | "week_streak" | "beginner_master" => {
                achievement.earned = true;
                achievement.earned_date = Some(Utc::now());
            },
            _ => {}
        }
    }
    
    let mut context = tera::Context::new();
    context.insert("title", "Learning Roadmap");
    context.insert("user_progress", &user_progress);
    context.insert("beginner_lessons", &beginner_lessons);
    context.insert("intermediate_lessons", &intermediate_lessons);
    context.insert("advanced_lessons", &advanced_lessons);
    context.insert("user_achievements", &user_achievements);
    
    let rendered = tmpl.render("progression/roadmap.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Helper function to get lessons for a stage
fn get_stage_lessons(stage: &str, user_progress: &UserProgress) -> Vec<LessonProgress> {
    match stage {
        "beginner" => vec![
            LessonProgress {
                id: "intro_to_it".to_string(),
                title: "Introduction to IT".to_string(),
                completed: true,
                current: false,
                locked: false,
                completion_date: Some(Utc::now()),
            },
            LessonProgress {
                id: "computer_basics".to_string(),
                title: "Computer Basics".to_string(),
                completed: true,
                current: false,
                locked: false,
                completion_date: Some(Utc::now()),
            },
            LessonProgress {
                id: "operating_systems".to_string(),
                title: "Operating Systems".to_string(),
                completed: false,
                current: true,
                locked: false,
                completion_date: None,
            },
            LessonProgress {
                id: "networking_basics".to_string(),
                title: "Networking Basics".to_string(),
                completed: false,
                current: false,
                locked: false,
                completion_date: None,
            },
        ],
        "intermediate" => vec![
            LessonProgress {
                id: "advanced_networking".to_string(),
                title: "Advanced Networking".to_string(),
                completed: false,
                current: false,
                locked: !user_progress.beginner_completed,
                completion_date: None,
            },
            LessonProgress {
                id: "databases".to_string(),
                title: "Database Management".to_string(),
                completed: false,
                current: false,
                locked: !user_progress.beginner_completed,
                completion_date: None,
            },
        ],
        "advanced" => vec![
            LessonProgress {
                id: "cloud_computing".to_string(),
                title: "Cloud Computing".to_string(),
                completed: false,
                current: false,
                locked: !user_progress.intermediate_completed,
                completion_date: None,
            },
            LessonProgress {
                id: "cybersecurity".to_string(),
                title: "Cybersecurity".to_string(),
                completed: false,
                current: false,
                locked: !user_progress.intermediate_completed,
                completion_date: None,
            },
        ],
        _ => vec![],
    }
}

// New endpoint to get user's flower garden
pub async fn get_flower_garden(
    tmpl: web::Data<Tera>,
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().finish()),
    };
    
    // Get user's flower collection
    let flower_collection = vec![
        ("🌸", "Cherry Blossom", 3, "common"),
        ("🌷", "Tulip", 2, "uncommon"), 
        ("🌻", "Sunflower", 1, "rare"),
        ("🌺", "Hibiscus", 1, "legendary"),
    ];
    
    let mut context = tera::Context::new();
    context.insert("title", "Your Flower Garden");
    context.insert("flower_collection", &flower_collection);
    
    let rendered = tmpl.render("progression/garden.html", &context)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
        
    Ok(HttpResponse::Ok().body(rendered))
}

// Add missing API endpoint for user progress
pub async fn get_user_progress_api(
    _db: web::Data<SqlitePool>,
    id: Identity,
) -> Result<HttpResponse> {
    // Check if user is authenticated
    let user_id = match id.id() {
        Ok(id) => id,
        Err(_) => return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Not authenticated"
        }))),
    };
    
    // Return demo progress data
    let courses_progress = vec![
        serde_json::json!({
            "id": "1",
            "progress": 75
        }),
        serde_json::json!({
            "id": "2", 
            "progress": 30
        }),
        serde_json::json!({
            "id": "3",
            "progress": 0
        })
    ];
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "courses": courses_progress
    })))
}
