use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Progress {
    pub id: Option<i64>,
    pub user_id: String,
    pub lesson_id: String,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub user: User,
    pub completed_lessons: i64,
    pub total_lessons: i64,
    pub completion_percentage: f64,
}

// New comprehensive progress tracking models
#[derive(Debug, Serialize, Deserialize)]
pub struct UserProgress {
    pub user_id: String,
    pub overall_percentage: f64,
    pub current_stage: String, // "beginner", "intermediate", "advanced", "expert"
    pub flowers_earned: i32,
    pub badges_earned: i32,
    pub streak_days: i32,
    
    // Stage-specific progress
    pub beginner_completed: bool,
    pub beginner_percentage: f64,
    pub intermediate_completed: bool,
    pub intermediate_percentage: f64,
    pub advanced_completed: bool,
    pub advanced_percentage: f64,
    pub expert_completed: bool,
    pub expert_percentage: f64,
    
    pub last_activity: DateTime<Utc>,
    pub total_study_time: i32, // in minutes
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub name: String,
    pub description: String,
    pub flower_emoji: String,
    pub category: String, // "progress", "streak", "completion", "special"
    pub requirement: i32, // numeric requirement to unlock
    pub earned: bool,
    pub earned_date: Option<DateTime<Utc>>,
    pub recently_earned: bool, // for highlighting new achievements
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LessonProgress {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub current: bool, // if this is the user's current lesson
    pub locked: bool,
    pub completion_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowerGarden {
    pub user_id: String,
    pub total_flowers: i32,
    pub flower_types: Vec<FlowerType>,
    pub garden_level: i32,
    pub last_watered: DateTime<Utc>, // for daily login streaks
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlowerType {
    pub name: String,
    pub emoji: String,
    pub count: i32,
    pub rarity: String, // "common", "uncommon", "rare", "legendary"
    pub unlock_condition: String,
}

impl UserProgress {
    pub fn new(user_id: String) -> Self {
        Self {
            user_id,
            overall_percentage: 0.0,
            current_stage: "beginner".to_string(),
            flowers_earned: 0,
            badges_earned: 0,
            streak_days: 0,
            beginner_completed: false,
            beginner_percentage: 0.0,
            intermediate_completed: false,
            intermediate_percentage: 0.0,
            advanced_completed: false,
            advanced_percentage: 0.0,
            expert_completed: false,
            expert_percentage: 0.0,
            last_activity: Utc::now(),
            total_study_time: 0,
        }
    }
    
    pub fn calculate_overall_progress(&mut self) {
        let total_progress = self.beginner_percentage + 
                           self.intermediate_percentage + 
                           self.advanced_percentage + 
                           self.expert_percentage;
        self.overall_percentage = total_progress / 4.0;
        
        // Update current stage based on progress
        if self.expert_percentage > 0.0 {
            self.current_stage = "expert".to_string();
        } else if self.advanced_percentage > 0.0 {
            self.current_stage = "advanced".to_string();
        } else if self.intermediate_percentage > 0.0 {
            self.current_stage = "intermediate".to_string();
        } else {
            self.current_stage = "beginner".to_string();
        }
    }
    
    pub fn award_flower(&mut self, _flower_type: &str) -> bool {
        self.flowers_earned += 1;
        true // Return true if successfully awarded
    }
    
    pub fn check_stage_completion(&mut self, stage: &str) {
        match stage {
            "beginner" if self.beginner_percentage >= 100.0 => {
                self.beginner_completed = true;
                self.award_flower("cherry_blossom");
            },
            "intermediate" if self.intermediate_percentage >= 100.0 => {
                self.intermediate_completed = true;
                self.award_flower("sunflower");
            },
            "advanced" if self.advanced_percentage >= 100.0 => {
                self.advanced_completed = true;
                self.award_flower("hibiscus");
            },
            "expert" if self.expert_percentage >= 100.0 => {
                self.expert_completed = true;
                self.award_flower("crown");
            },
            _ => {}
        }
    }
}

// Predefined achievements
impl Achievement {
    pub fn get_default_achievements() -> Vec<Achievement> {
        vec![
            Achievement {
                id: "first_lesson".to_string(),
                name: "First Bloom".to_string(),
                description: "Complete your first lesson".to_string(),
                flower_emoji: "🌸".to_string(),
                category: "progress".to_string(),
                requirement: 1,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "week_streak".to_string(),
                name: "Dedicated Gardener".to_string(),
                description: "Study for 7 days in a row".to_string(),
                flower_emoji: "🌻".to_string(),
                category: "streak".to_string(),
                requirement: 7,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "beginner_master".to_string(),
                name: "Seedling Graduate".to_string(),
                description: "Complete all beginner lessons".to_string(),
                flower_emoji: "🌷".to_string(),
                category: "completion".to_string(),
                requirement: 100,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "speed_learner".to_string(),
                name: "Quick Bloomer".to_string(),
                description: "Complete 5 lessons in one day".to_string(),
                flower_emoji: "🌺".to_string(),
                category: "special".to_string(),
                requirement: 5,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "night_owl".to_string(),
                name: "Midnight Gardener".to_string(),
                description: "Study after 10 PM".to_string(),
                flower_emoji: "🌙".to_string(),
                category: "special".to_string(),
                requirement: 1,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "perfectionist".to_string(),
                name: "Perfect Garden".to_string(),
                description: "Complete all lessons with 100%".to_string(),
                flower_emoji: "💎".to_string(),
                category: "completion".to_string(),
                requirement: 100,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
            Achievement {
                id: "master_graduate".to_string(),
                name: "IT Master".to_string(),
                description: "Complete the entire learning path".to_string(),
                flower_emoji: "👑".to_string(),
                category: "completion".to_string(),
                requirement: 100,
                earned: false,
                earned_date: None,
                recently_earned: false,
            },
        ]
    }
}
