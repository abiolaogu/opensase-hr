//! Performance Service

use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

use super::models::*;

/// Performance service errors
#[derive(Debug, thiserror::Error)]
pub enum PerformanceError {
    #[error("Review not found: {0}")]
    NotFound(Uuid),
    #[error("Cycle not found: {0}")]
    CycleNotFound(Uuid),
    #[error("Review already submitted")]
    AlreadySubmitted,
    #[error("Cycle is not active")]
    CycleNotActive,
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Performance Service
#[derive(Debug, Clone, Default)]
pub struct PerformanceService;

impl PerformanceService {
    pub fn new() -> Self {
        Self
    }

    /// Calculate final rating from goals and competencies
    pub fn calculate_final_rating(
        &self,
        goals_rating: Decimal,
        competencies_rating: Decimal,
        goals_weight: Decimal,
        competencies_weight: Decimal,
    ) -> Decimal {
        (goals_rating * goals_weight) + (competencies_rating * competencies_weight)
    }

    /// Calculate goals rating from individual goal ratings
    pub fn calculate_goals_rating(&self, goals: &[Goal]) -> Decimal {
        if goals.is_empty() {
            return Decimal::ZERO;
        }

        let total_weight: Decimal = goals.iter().map(|g| g.weight).sum();
        if total_weight == Decimal::ZERO {
            return Decimal::ZERO;
        }

        let weighted_sum: Decimal = goals
            .iter()
            .filter_map(|g| g.rating.map(|r| r * g.weight))
            .sum();

        weighted_sum / total_weight
    }

    /// Calculate competencies rating
    pub fn calculate_competencies_rating(&self, competencies: &[CompetencyRating]) -> Decimal {
        if competencies.is_empty() {
            return Decimal::ZERO;
        }

        let ratings: Vec<Decimal> = competencies
            .iter()
            .filter_map(|c| c.manager_rating.or(c.self_rating))
            .collect();

        if ratings.is_empty() {
            return Decimal::ZERO;
        }

        let sum: Decimal = ratings.iter().sum();
        sum / Decimal::from(ratings.len())
    }

    /// Submit self review
    pub fn submit_self_review(
        &self,
        review: &mut PerformanceReview,
        request: SubmitSelfReviewRequest,
    ) -> Result<(), PerformanceError> {
        if review.status != ReviewStatus::Pending {
            return Err(PerformanceError::AlreadySubmitted);
        }

        // Apply self ratings to goals
        for goal_rating in &request.goals_self_ratings {
            if let Some(goal) = review.goals.iter_mut().find(|g| g.id == goal_rating.goal_id) {
                goal.rating = Some(goal_rating.rating);
            }
        }

        // Apply competency ratings
        review.competencies = request.competencies_ratings;
        
        // Calculate self rating
        let goals_rating = self.calculate_goals_rating(&review.goals);
        review.self_rating = Some(goals_rating);
        
        review.self_comments = request.comments;
        review.self_review_submitted_at = Some(Utc::now());
        review.status = ReviewStatus::SelfSubmitted;
        review.updated_at = Utc::now();

        Ok(())
    }

    /// Complete review (manager)
    pub fn complete_review(
        &self,
        review: &mut PerformanceReview,
        cycle: &PerformanceCycle,
        manager_rating: Decimal,
        comments: Option<String>,
    ) -> Result<(), PerformanceError> {
        if review.status != ReviewStatus::SelfSubmitted && review.status != ReviewStatus::Pending {
            return Err(PerformanceError::Validation(
                "Review must be in self-submitted or pending state".to_string()
            ));
        }

        review.manager_rating = Some(manager_rating);
        review.manager_comments = comments;
        review.manager_review_submitted_at = Some(Utc::now());

        // Calculate final rating
        let competencies_rating = self.calculate_competencies_rating(&review.competencies);
        review.final_rating = Some(self.calculate_final_rating(
            manager_rating,
            competencies_rating,
            cycle.goals_weight,
            cycle.competencies_weight,
        ));

        review.status = ReviewStatus::Completed;
        review.updated_at = Utc::now();

        Ok(())
    }

    /// Get rating category
    pub fn get_rating_category(&self, score: Decimal) -> RatingCategory {
        RatingCategory::from_score(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_final_rating() {
        let service = PerformanceService::new();
        
        let final_rating = service.calculate_final_rating(
            dec!(4.0),  // goals
            dec!(3.5),  // competencies
            dec!(0.70), // goals weight
            dec!(0.30), // competencies weight
        );

        // 4.0 * 0.70 + 3.5 * 0.30 = 2.8 + 1.05 = 3.85
        assert_eq!(final_rating, dec!(3.85));
    }

    #[test]
    fn test_rating_category() {
        assert_eq!(RatingCategory::from_score(dec!(1.5)), RatingCategory::NeedsImprovement);
        assert_eq!(RatingCategory::from_score(dec!(2.5)), RatingCategory::MeetsSomeExpectations);
        assert_eq!(RatingCategory::from_score(dec!(3.5)), RatingCategory::MeetsExpectations);
        assert_eq!(RatingCategory::from_score(dec!(4.3)), RatingCategory::ExceedsExpectations);
        assert_eq!(RatingCategory::from_score(dec!(4.8)), RatingCategory::Outstanding);
    }
}
