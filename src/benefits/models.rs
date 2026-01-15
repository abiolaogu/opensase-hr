//! Benefits Models

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Benefit plan type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BenefitPlanType {
    Hmo,
    LifeInsurance,
    PensionAvc,  // Additional Voluntary Contribution
    Allowance,
}

/// Benefit plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitPlan {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub plan_type: BenefitPlanType,
    pub provider: Option<String>,
    pub coverage_details: serde_json::Value,
    pub cost_employee: Decimal,
    pub cost_employer: Decimal,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enrollment status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentStatus {
    Active,
    Cancelled,
    Expired,
}

/// Employee benefit enrollment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBenefit {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub benefit_plan_id: Uuid,
    pub plan_name: Option<String>,
    pub enrolled_date: NaiveDate,
    pub status: EnrollmentStatus,
    pub dependents: Vec<Dependent>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Dependent (for HMO coverage)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependent {
    pub name: String,
    pub relationship: String,  // spouse, child, parent
    pub date_of_birth: Option<NaiveDate>,
    pub is_covered: bool,
}

/// Claim status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimStatus {
    Pending,
    Approved,
    Rejected,
    Paid,
}

/// Benefit claim
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitClaim {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub benefit_plan_id: Uuid,
    pub claim_type: String,
    pub amount: Decimal,
    pub description: Option<String>,
    pub receipt_url: Option<String>,
    pub status: ClaimStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Enroll in benefit request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollBenefitRequest {
    pub benefit_plan_id: Uuid,
    pub dependents: Vec<Dependent>,
}

/// Submit claim request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitClaimRequest {
    pub benefit_plan_id: Uuid,
    pub claim_type: String,
    pub amount: Decimal,
    pub description: Option<String>,
    pub receipt_url: Option<String>,
}

/// Nigerian HMO providers
pub const NIGERIAN_HMO_PROVIDERS: &[&str] = &[
    "Hygeia HMO",
    "Total Health Trust",
    "Leadway Health",
    "Redcare HMO",
    "Clearline HMO",
    "United Healthcare",
    "Reliance HMO",
    "Avon HMO",
    "Axa Mansard Health",
    "Liberty Health",
];
