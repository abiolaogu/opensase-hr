//! Recruitment Models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Job posting status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobPostingStatus {
    Draft,
    Published,
    Closed,
    Filled,
}

/// Job posting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub title: String,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub description: String,
    pub requirements: Vec<String>,
    pub responsibilities: Vec<String>,
    pub salary_min: Option<Decimal>,
    pub salary_max: Option<Decimal>,
    pub show_salary: bool,
    pub location: Option<String>,
    pub employment_type: String,
    pub experience_level: Option<String>,
    pub status: JobPostingStatus,
    pub posted_date: Option<DateTime<Utc>>,
    pub closing_date: Option<NaiveDate>,
    pub vacancies: i32,
    pub applications_count: i32,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Application stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicationStage {
    Received,
    Screening,
    Interview,
    Offer,
    Hired,
    Rejected,
}

/// AI recommendation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiRecommendation {
    StrongYes,
    Yes,
    Maybe,
    No,
}

/// Job application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: Uuid,
    pub job_posting_id: Uuid,
    pub applicant_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub cv_url: Option<String>,
    pub cover_letter: Option<String>,
    pub linkedin_url: Option<String>,
    pub ai_score: Option<Decimal>,
    pub ai_analysis: Option<CvAnalysis>,
    pub stage: ApplicationStage,
    pub stage_history: Vec<StageHistoryEntry>,
    pub interview_scheduled_at: Option<DateTime<Utc>>,
    pub interview_notes: Option<String>,
    pub interview_rating: Option<Decimal>,
    pub rejection_reason: Option<String>,
    pub offer_salary: Option<Decimal>,
    pub offer_sent_at: Option<DateTime<Utc>>,
    pub offer_accepted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Stage history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StageHistoryEntry {
    pub stage: ApplicationStage,
    pub entered_at: DateTime<Utc>,
    pub notes: Option<String>,
}

/// CV Analysis result from AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvAnalysis {
    pub score: Decimal,
    pub skills_matched: Vec<String>,
    pub skills_missing: Vec<String>,
    pub experience_years: Decimal,
    pub education_match: bool,
    pub summary: String,
    pub concerns: Vec<String>,
    pub recommendation: AiRecommendation,
}

/// Create job posting request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateJobPostingRequest {
    pub title: String,
    pub department_id: Option<Uuid>,
    pub description: String,
    pub requirements: Vec<String>,
    pub salary_min: Option<Decimal>,
    pub salary_max: Option<Decimal>,
    pub location: Option<String>,
    pub employment_type: String,
    pub closing_date: Option<NaiveDate>,
}

/// Submit application request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitApplicationRequest {
    pub applicant_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub cv_url: Option<String>,
    pub cover_letter: Option<String>,
    pub linkedin_url: Option<String>,
}

/// Move application stage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveStageRequest {
    pub new_stage: ApplicationStage,
    pub notes: Option<String>,
}
