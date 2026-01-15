//! Recruitment Module with AI CV Scoring
//!
//! Job postings, applications pipeline, and AI-powered candidate ranking.

pub mod models;
pub mod service;
pub mod ai_scorer;

pub use models::*;
pub use service::RecruitmentService;
pub use ai_scorer::AiCvScorer;
