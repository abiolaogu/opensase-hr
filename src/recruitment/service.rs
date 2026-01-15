//! Recruitment Service

use chrono::Utc;
use uuid::Uuid;

use super::models::*;

/// Recruitment service errors
#[derive(Debug, thiserror::Error)]
pub enum RecruitmentError {
    #[error("Job posting not found: {0}")]
    JobNotFound(Uuid),
    #[error("Application not found: {0}")]
    ApplicationNotFound(Uuid),
    #[error("Job is not published")]
    JobNotPublished,
    #[error("Job is closed")]
    JobClosed,
    #[error("Invalid stage transition")]
    InvalidStageTransition,
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Recruitment Service
#[derive(Debug, Clone, Default)]
pub struct RecruitmentService;

impl RecruitmentService {
    pub fn new() -> Self {
        Self
    }

    /// Create job posting
    pub fn create_job_posting(
        &self,
        tenant_id: Uuid,
        request: CreateJobPostingRequest,
        created_by: Uuid,
    ) -> JobPosting {
        let now = Utc::now();
        JobPosting {
            id: Uuid::new_v4(),
            tenant_id,
            title: request.title,
            department_id: request.department_id,
            position_id: None,
            description: request.description,
            requirements: request.requirements,
            responsibilities: vec![],
            salary_min: request.salary_min,
            salary_max: request.salary_max,
            show_salary: false,
            location: request.location,
            employment_type: request.employment_type,
            experience_level: None,
            status: JobPostingStatus::Draft,
            posted_date: None,
            closing_date: request.closing_date,
            vacancies: 1,
            applications_count: 0,
            created_by: Some(created_by),
            created_at: now,
            updated_at: now,
        }
    }

    /// Publish job posting
    pub fn publish_job(&self, job: &mut JobPosting) -> Result<(), RecruitmentError> {
        if job.status != JobPostingStatus::Draft {
            return Err(RecruitmentError::Validation(
                "Only draft jobs can be published".to_string()
            ));
        }
        job.status = JobPostingStatus::Published;
        job.posted_date = Some(Utc::now());
        job.updated_at = Utc::now();
        Ok(())
    }

    /// Submit application
    pub fn submit_application(
        &self,
        job: &JobPosting,
        request: SubmitApplicationRequest,
    ) -> Result<JobApplication, RecruitmentError> {
        if job.status != JobPostingStatus::Published {
            return Err(RecruitmentError::JobNotPublished);
        }

        let now = Utc::now();
        Ok(JobApplication {
            id: Uuid::new_v4(),
            job_posting_id: job.id,
            applicant_name: request.applicant_name,
            email: request.email,
            phone: request.phone,
            cv_url: request.cv_url,
            cover_letter: request.cover_letter,
            linkedin_url: request.linkedin_url,
            ai_score: None,
            ai_analysis: None,
            stage: ApplicationStage::Received,
            stage_history: vec![StageHistoryEntry {
                stage: ApplicationStage::Received,
                entered_at: now,
                notes: None,
            }],
            interview_scheduled_at: None,
            interview_notes: None,
            interview_rating: None,
            rejection_reason: None,
            offer_salary: None,
            offer_sent_at: None,
            offer_accepted_at: None,
            created_at: now,
            updated_at: now,
        })
    }

    /// Move application to new stage
    pub fn move_to_stage(
        &self,
        application: &mut JobApplication,
        request: MoveStageRequest,
    ) -> Result<(), RecruitmentError> {
        // Validate stage transition
        let valid = match (&application.stage, &request.new_stage) {
            (ApplicationStage::Received, ApplicationStage::Screening) => true,
            (ApplicationStage::Received, ApplicationStage::Rejected) => true,
            (ApplicationStage::Screening, ApplicationStage::Interview) => true,
            (ApplicationStage::Screening, ApplicationStage::Rejected) => true,
            (ApplicationStage::Interview, ApplicationStage::Offer) => true,
            (ApplicationStage::Interview, ApplicationStage::Rejected) => true,
            (ApplicationStage::Offer, ApplicationStage::Hired) => true,
            (ApplicationStage::Offer, ApplicationStage::Rejected) => true,
            _ => false,
        };

        if !valid {
            return Err(RecruitmentError::InvalidStageTransition);
        }

        application.stage = request.new_stage;
        application.stage_history.push(StageHistoryEntry {
            stage: request.new_stage,
            entered_at: Utc::now(),
            notes: request.notes,
        });
        application.updated_at = Utc::now();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_publish_job() {
        let service = RecruitmentService::new();
        let tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        let request = CreateJobPostingRequest {
            title: "Software Engineer".to_string(),
            department_id: None,
            description: "Looking for a skilled engineer".to_string(),
            requirements: vec!["Rust".to_string(), "PostgreSQL".to_string()],
            salary_min: None,
            salary_max: None,
            location: Some("Lagos, Nigeria".to_string()),
            employment_type: "full_time".to_string(),
            closing_date: None,
        };

        let mut job = service.create_job_posting(tenant_id, request, user_id);
        assert_eq!(job.status, JobPostingStatus::Draft);

        service.publish_job(&mut job).unwrap();
        assert_eq!(job.status, JobPostingStatus::Published);
        assert!(job.posted_date.is_some());
    }

    #[test]
    fn test_application_pipeline() {
        let service = RecruitmentService::new();
        let tenant_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();

        // Create and publish job
        let mut job = service.create_job_posting(
            tenant_id,
            CreateJobPostingRequest {
                title: "Developer".to_string(),
                department_id: None,
                description: "Dev role".to_string(),
                requirements: vec![],
                salary_min: None,
                salary_max: None,
                location: None,
                employment_type: "full_time".to_string(),
                closing_date: None,
            },
            user_id,
        );
        service.publish_job(&mut job).unwrap();

        // Submit application
        let mut application = service.submit_application(
            &job,
            SubmitApplicationRequest {
                applicant_name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                phone: None,
                cv_url: Some("https://example.com/cv.pdf".to_string()),
                cover_letter: None,
                linkedin_url: None,
            },
        ).unwrap();

        assert_eq!(application.stage, ApplicationStage::Received);

        // Move through pipeline
        service.move_to_stage(&mut application, MoveStageRequest {
            new_stage: ApplicationStage::Screening,
            notes: Some("Initial screen".to_string()),
        }).unwrap();

        service.move_to_stage(&mut application, MoveStageRequest {
            new_stage: ApplicationStage::Interview,
            notes: None,
        }).unwrap();

        assert_eq!(application.stage, ApplicationStage::Interview);
        assert_eq!(application.stage_history.len(), 3);
    }
}
