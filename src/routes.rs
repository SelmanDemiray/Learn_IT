use actix_web::{web, HttpResponse, Result};
use tera::Tera;

// Import from crate::modules instead of declaring local modules
use crate::modules::auth;
use crate::modules::courses;
use crate::modules::lessons;
use crate::modules::users;

// Main index function
async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Home");
    
    let rendered = tmpl.render("index.html", &context)
        .map_err(|e| {
            log::error!("Template error in index: {}", e);
            // Return a simple HTML response if template fails
            actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
        })?;
        
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// About page
async fn about(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "About");
    
    let rendered = tmpl.render("about.html", &context)
        .map_err(|e| {
            log::error!("Template error in about: {}", e);
            actix_web::error::ErrorInternalServerError(format!("Template error: {}", e))
        })?;
        
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Add roadmap route
async fn roadmap(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let mut context = tera::Context::new();
    context.insert("title", "Learning Roadmap");
    
    // Roadmap data structure
    let roadmap_data = serde_json::json!({
        "paths": [
            {
                "name": "Beginner Path",
                "description": "Start your IT journey with fundamental concepts",
                "color": "beginner",
                "duration": "3-6 months",
                "steps": [
                    {
                        "title": "Computer Fundamentals",
                        "description": "Learn about hardware, software, and basic computer operations",
                        "lessons": ["intro-to-computers", "hardware-basics", "software-basics"],
                        "duration": "2 weeks"
                    },
                    {
                        "title": "Operating Systems",
                        "description": "Understanding Windows, Linux, and macOS basics",
                        "lessons": ["os-intro", "windows-basics", "linux-basics"],
                        "duration": "3 weeks"
                    },
                    {
                        "title": "Networking Basics",
                        "description": "Learn how computers communicate",
                        "lessons": ["network-intro", "tcp-ip-basics", "internet-basics"],
                        "duration": "3 weeks"
                    },
                    {
                        "title": "Internet & Web",
                        "description": "Understanding the web and basic web technologies",
                        "lessons": ["web-intro", "html-basics", "css-basics"],
                        "duration": "4 weeks"
                    }
                ]
            },
            {
                "name": "Intermediate Path", 
                "description": "Build upon your foundation with more advanced topics",
                "color": "intermediate",
                "duration": "6-12 months",
                "steps": [
                    {
                        "title": "Advanced Networking",
                        "description": "Deep dive into network protocols and administration",
                        "lessons": ["advanced-tcp-ip", "routing-switching", "network-security"],
                        "duration": "4 weeks"
                    },
                    {
                        "title": "Database Systems",
                        "description": "Learn to store, manage, and query data",
                        "lessons": ["database-intro", "sql-basics", "database-design"],
                        "duration": "6 weeks"
                    },
                    {
                        "title": "Programming Fundamentals",
                        "description": "Start coding with popular programming languages",
                        "lessons": ["programming-intro", "python-basics", "web-development"],
                        "duration": "8 weeks"
                    },
                    {
                        "title": "System Administration",
                        "description": "Manage servers and IT infrastructure",
                        "lessons": ["server-basics", "user-management", "backup-recovery"],
                        "duration": "6 weeks"
                    }
                ]
            },
            {
                "name": "Advanced Path",
                "description": "Specialize in cutting-edge technologies and expert practices",
                "color": "advanced", 
                "duration": "12+ months",
                "steps": [
                    {
                        "title": "Cloud Computing",
                        "description": "Master cloud platforms and services",
                        "lessons": ["cloud-intro", "aws-basics", "azure-basics", "cloud-architecture"],
                        "duration": "8 weeks"
                    },
                    {
                        "title": "DevOps & Automation",
                        "description": "Streamline development and operations",
                        "lessons": ["devops-intro", "ci-cd", "infrastructure-code", "monitoring"],
                        "duration": "10 weeks"
                    },
                    {
                        "title": "Cybersecurity",
                        "description": "Protect systems and data from threats",
                        "lessons": ["security-fundamentals", "ethical-hacking", "incident-response"],
                        "duration": "12 weeks"
                    },
                    {
                        "title": "Emerging Technologies",
                        "description": "AI, ML, IoT, and future tech trends",
                        "lessons": ["ai-intro", "machine-learning", "iot-basics", "blockchain"],
                        "duration": "16 weeks"
                    }
                ]
            }
        ]
    });
    
    context.insert("roadmap", &roadmap_data);
    
    let rendered = tmpl.render("roadmap.html", &context).map_err(|e| {
        log::error!("Template rendering error: {}", e);
        actix_web::error::ErrorInternalServerError("Template rendering failed")
    })?;
        
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

// Emergency fallback route for debugging
async fn debug_info(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let template_names: Vec<String> = tmpl.get_template_names().map(|s| s.to_string()).collect();
    
    let debug_html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Debug Info - IT Learning Platform</title>
    <style>
        body {{ font-family: Arial, sans-serif; max-width: 800px; margin: 0 auto; padding: 20px; }}
        pre {{ background: #f4f4f4; padding: 15px; overflow: auto; }}
        .templates {{ background: #e8f5e8; padding: 10px; }}
    </style>
</head>
<body>
    <h1>IT Learning Platform - Debug Info</h1>
    <h2>Available Templates</h2>
    <div class="templates">
        <ul>
            {}
        </ul>
    </div>
    <h2>Quick Links</h2>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/lessons">All Lessons</a></li>
        <li><a href="/courses">Courses</a></li>
        <li><a href="/about">About</a></li>
    </ul>
</body>
</html>
    "#, template_names.iter().map(|t| format!("<li>{}</li>", t)).collect::<Vec<_>>().join(""));
    
    Ok(HttpResponse::Ok().content_type("text/html").body(debug_html))
}

// Configure all routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").to(index))
       .service(web::resource("/about").to(about))
       .service(web::resource("/roadmap").to(roadmap))
       .service(web::resource("/debug").to(debug_info))
       .configure(auth::routes::configure)
       .configure(courses::routes::configure)
       .configure(lessons::routes::configure)
       .configure(users::routes::configure);
}
