//! Performance Management Models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Cycle type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CycleType {
    Annual,
    Quarterly,
    Probation,
}

/// Cycle status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CycleStatus {
    Draft,
    Active,
    Closed,
}

/// Performance Cycle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCycle {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub cycle_type: CycleType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub goals_weight: Decimal,         // e.g., 0.70 = 70%
    pub competencies_weight: Decimal,  // e.g., 0.30 = 30%
    pub status: CycleStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Review status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Pending,
    SelfSubmitted,
    ManagerSubmitted,
    Completed,
}

/// Performance Review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReview {
    pub id: Uuid,
    pub cycle_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Option<Uuid>,
    pub self_rating: Option<Decimal>,
    pub manager_rating: Option<Decimal>,
    pub final_rating: Option<Decimal>,
    pub goals: Vec<Goal>,
    pub competencies: Vec<CompetencyRating>,
    pub self_review_submitted_at: Option<DateTime<Utc>>,
    pub manager_review_submitted_at: Option<DateTime<Utc>>,
    pub status: ReviewStatus,
    pub self_comments: Option<String>,
    pub manager_comments: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Goal category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalCategory {
    Individual,
    Team,
    Company,
}

/// Goal status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    Active,
    Completed,
    Cancelled,
}

/// Goal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub category: GoalCategory,
    // SMART criteria
    pub specific: Option<String>,
    pub measurable: Option<String>,
    pub achievable: Option<String>,
    pub relevant: Option<String>,
    pub time_bound: Option<NaiveDate>,
    pub weight: Decimal,           // % of total goals score
    pub target_value: Option<Decimal>,
    pub current_value: Decimal,
    pub progress_percentage: i32,
    pub status: GoalStatus,
    pub rating: Option<Decimal>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Competency Rating
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyRating {
    pub competency_name: String,
    pub competency_type: String, // core, role_specific, leadership
    pub self_rating: Option<Decimal>,
    pub manager_rating: Option<Decimal>,
    pub comments: Option<String>,
}

/// Rating category based on final score
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RatingCategory {
    NeedsImprovement,      // < 2.0
    MeetsSomeExpectations, // 2.0-2.9
    MeetsExpectations,     // 3.0-3.9
    ExceedsExpectations,   // 4.0-4.5
    Outstanding,           // 4.5-5.0
}

impl RatingCategory {
    pub fn from_score(score: Decimal) -> Self {
        use rust_decimal_macros::dec;
        if score < dec!(2.0) {
            Self::NeedsImprovement
        } else if score < dec!(3.0) {
            Self::MeetsSomeExpectations
        } else if score < dec!(4.0) {
            Self::MeetsExpectations
        } else if score < dec!(4.5) {
            Self::ExceedsExpectations
        } else {
            Self::Outstanding
        }
    }
}

/// Create goal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    pub description: Option<String>,
    pub category: GoalCategory,
    pub weight: Decimal,
    pub target_value: Option<Decimal>,
    pub time_bound: Option<NaiveDate>,
}

/// Update goal progress request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGoalProgressRequest {
    pub current_value: Decimal,
    pub progress_percentage: i32,
}

/// Submit self review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSelfReviewRequest {
    pub goals_self_ratings: Vec<GoalSelfRating>,
    pub competencies_ratings: Vec<CompetencyRating>,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalSelfRating {
    pub goal_id: Uuid,
    pub rating: Decimal,
}
