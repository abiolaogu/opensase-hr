//! Leave Management Models
//!
//! Data structures for leave types, balances, and requests.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Leave Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveType {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub code: String,
    pub default_days: i32,
    pub is_paid: bool,
    pub requires_approval: bool,
    pub requires_document: bool,
    pub document_threshold_days: i32,
    pub max_carry_over: i32,
    pub gender_restriction: Option<String>,  // "male", "female", or None
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Standard Nigerian Leave Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StandardLeaveType {
    Annual,
    Sick,
    Maternity,
    Paternity,
    Compassionate,
    Study,
    LeaveWithoutPay,
}

impl StandardLeaveType {
    pub fn default_days(&self) -> i32 {
        match self {
            Self::Annual => 21,
            Self::Sick => 12,
            Self::Maternity => 84,  // 12 weeks
            Self::Paternity => 10,
            Self::Compassionate => 5,
            Self::Study => 20,
            Self::LeaveWithoutPay => 30,
        }
    }

    pub fn is_paid(&self) -> bool {
        !matches!(self, Self::Study | Self::LeaveWithoutPay)
    }

    pub fn code(&self) -> &'static str {
        match self {
            Self::Annual => "annual",
            Self::Sick => "sick",
            Self::Maternity => "maternity",
            Self::Paternity => "paternity",
            Self::Compassionate => "compassionate",
            Self::Study => "study",
            Self::LeaveWithoutPay => "lwop",
        }
    }
}

/// Leave Balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub leave_type_name: String,
    pub year: i32,
    pub entitled_days: Decimal,
    pub used_days: Decimal,
    pub pending_days: Decimal,
    pub carried_over: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LeaveBalance {
    pub fn available_days(&self) -> Decimal {
        self.entitled_days + self.carried_over - self.used_days - self.pending_days
    }
}

/// Leave Request Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LeaveRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

impl Default for LeaveRequestStatus {
    fn default() -> Self {
        Self::Pending
    }
}

/// Leave Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub employee_name: Option<String>,
    pub leave_type_id: Uuid,
    pub leave_type_name: Option<String>,
    
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days_requested: Decimal,
    pub half_day: bool,
    
    pub reason: Option<String>,
    pub document_url: Option<String>,
    
    pub relief_officer_id: Option<Uuid>,
    pub relief_officer_name: Option<String>,
    pub handover_notes: Option<String>,
    
    pub status: LeaveRequestStatus,
    pub approved_by: Option<Uuid>,
    pub approver_name: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Public Holiday
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicHoliday {
    pub id: Uuid,
    pub tenant_id: Option<Uuid>,
    pub name: String,
    pub date: NaiveDate,
    pub is_recurring: bool,
    pub year: Option<i32>,
}

/// Request to create a leave request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeaveRequest {
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub half_day: bool,
    pub reason: Option<String>,
    pub relief_officer_id: Option<Uuid>,
    pub handover_notes: Option<String>,
}

/// Request to approve/reject leave
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveDecisionRequest {
    pub approved: bool,
    pub rejection_reason: Option<String>,
}

/// Leave calendar entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveCalendarEntry {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: LeaveRequestStatus,
}

/// Leave balance summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalanceSummary {
    pub employee_id: Uuid,
    pub year: i32,
    pub balances: Vec<LeaveBalance>,
    pub total_entitled: Decimal,
    pub total_used: Decimal,
    pub total_pending: Decimal,
    pub total_available: Decimal,
}
