//! AI CV Scorer (Mock Implementation)
//!
//! Provides mock AI scoring for CVs. In production, this would integrate with
//! OpenAI, Anthropic, or other LLM providers.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

use super::models::{CvAnalysis, AiRecommendation, JobPosting};

/// AI CV Scoring Service
#[derive(Debug, Clone, Default)]
pub struct AiCvScorer {
    // In production: LLM client configuration, API keys, etc.
}

impl AiCvScorer {
    pub fn new() -> Self {
        Self {}
    }

    /// Analyze CV against job requirements (mock implementation)
    /// 
    /// In production, this would:
    /// 1. Parse CV content (PDF/DOCX)
    /// 2. Call LLM API with job requirements + CV
    /// 3. Parse structured response
    pub async fn analyze_cv(
        &self,
        cv_content: &str,
        job_posting: &JobPosting,
    ) -> CvAnalysis {
        // Mock analysis based on simple keyword matching
        let cv_lower = cv_content.to_lowercase();
        
        let mut skills_matched = Vec::new();
        let mut skills_missing = Vec::new();
        
        for req in &job_posting.requirements {
            let req_lower = req.to_lowercase();
            let keywords: Vec<&str> = req_lower.split_whitespace().collect();
            
            let matched = keywords.iter().any(|k| cv_lower.contains(k));
            if matched {
                skills_matched.push(req.clone());
            } else {
                skills_missing.push(req.clone());
            }
        }
        
        // Calculate score
        let total_requirements = job_posting.requirements.len() as f32;
        let matched_count = skills_matched.len() as f32;
        let score = if total_requirements > 0.0 {
            (matched_count / total_requirements) * 100.0
        } else {
            50.0
        };
        let score = Decimal::from_f32_retain(score).unwrap_or(dec!(50));

        // Mock experience extraction
        let experience_years = if cv_lower.contains("10 years") || cv_lower.contains("10+ years") {
            dec!(10)
        } else if cv_lower.contains("5 years") || cv_lower.contains("5+ years") {
            dec!(5)
        } else if cv_lower.contains("3 years") || cv_lower.contains("3+ years") {
            dec!(3)
        } else {
            dec!(1)
        };

        // Mock education check
        let education_match = cv_lower.contains("bachelor") 
            || cv_lower.contains("master") 
            || cv_lower.contains("degree")
            || cv_lower.contains("b.sc")
            || cv_lower.contains("m.sc");

        // Determine recommendation
        let recommendation = if score >= dec!(80) && education_match {
            AiRecommendation::StrongYes
        } else if score >= dec!(60) {
            AiRecommendation::Yes
        } else if score >= dec!(40) {
            AiRecommendation::Maybe
        } else {
            AiRecommendation::No
        };

        // Build concerns
        let mut concerns = Vec::new();
        if skills_missing.len() > skills_matched.len() {
            concerns.push("Missing majority of required skills".to_string());
        }
        if !education_match {
            concerns.push("Education requirements may not be met".to_string());
        }
        if experience_years < dec!(3) {
            concerns.push("Limited experience indicated".to_string());
        }

        let summary = format!(
            "Candidate matched {}/{} requirements. Experience: ~{} years. {}",
            skills_matched.len(),
            job_posting.requirements.len(),
            experience_years,
            if education_match { "Education requirements appear met." } else { "" }
        );

        CvAnalysis {
            score,
            skills_matched,
            skills_missing,
            experience_years,
            education_match,
            summary,
            concerns,
            recommendation,
        }
    }

    /// Rank candidates by AI score
    pub fn rank_candidates(
        &self,
        analyses: &mut [(uuid::Uuid, CvAnalysis)],
    ) {
        analyses.sort_by(|a, b| b.1.score.cmp(&a.1.score));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_posting() -> JobPosting {
        JobPosting {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            title: "Senior Rust Developer".to_string(),
            department_id: None,
            position_id: None,
            description: "We are looking for a Rust developer".to_string(),
            requirements: vec![
                "Rust programming".to_string(),
                "Web development".to_string(),
                "PostgreSQL".to_string(),
                "Docker".to_string(),
            ],
            responsibilities: vec![],
            salary_min: None,
            salary_max: None,
            show_salary: false,
            location: None,
            employment_type: "full_time".to_string(),
            experience_level: Some("senior".to_string()),
            status: super::super::models::JobPostingStatus::Published,
            posted_date: Some(Utc::now()),
            closing_date: None,
            vacancies: 1,
            applications_count: 0,
            created_by: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_cv_analysis_good_match() {
        let scorer = AiCvScorer::new();
        let posting = create_test_posting();
        
        let cv = "
            Senior Software Engineer with 5+ years experience.
            Skills: Rust programming, web development, PostgreSQL, Docker, Kubernetes
            Education: B.Sc Computer Science
        ";

        let analysis = scorer.analyze_cv(cv, &posting).await;
        
        assert!(analysis.score >= dec!(75));
        assert_eq!(analysis.skills_matched.len(), 4);
        assert!(analysis.education_match);
        assert!(matches!(analysis.recommendation, AiRecommendation::StrongYes | AiRecommendation::Yes));
    }

    #[tokio::test]
    async fn test_cv_analysis_poor_match() {
        let scorer = AiCvScorer::new();
        let posting = create_test_posting();
        
        let cv = "
            Junior Developer, 1 year experience
            Skills: JavaScript, React
        ";

        let analysis = scorer.analyze_cv(cv, &posting).await;
        
        assert!(analysis.score < dec!(50));
        assert!(!analysis.concerns.is_empty());
        assert!(matches!(analysis.recommendation, AiRecommendation::Maybe | AiRecommendation::No));
    }
}
