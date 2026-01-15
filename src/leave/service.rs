//! Leave Management Service
//!
//! Business logic for leave requests, balances, and approvals.

use std::collections::HashSet;
use chrono::{Datelike, NaiveDate, Utc};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

use super::models::*;

/// Leave service errors
#[derive(Debug, thiserror::Error)]
pub enum LeaveError {
    #[error("Leave request not found: {0}")]
    NotFound(Uuid),
    
    #[error("Leave type not found: {0}")]
    LeaveTypeNotFound(Uuid),
    
    #[error("Insufficient leave balance: available {available}, requested {requested}")]
    InsufficientBalance { available: Decimal, requested: Decimal },
    
    #[error("Leave request already {0}")]
    InvalidStatus(String),
    
    #[error("Start date must be before or equal to end date")]
    InvalidDateRange,
    
    #[error("Leave request overlaps with existing request")]
    OverlappingRequest,
    
    #[error("Document required for leave > {0} days")]
    DocumentRequired(i32),
    
    #[error("Relief officer required for leave > 3 days")]
    ReliefOfficerRequired,
    
    #[error("This leave type is restricted to {0} employees")]
    GenderRestricted(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Leave Management Service
#[derive(Debug, Clone, Default)]
pub struct LeaveService {
    // In real implementation, would have database pool
}

impl LeaveService {
    pub fn new() -> Self {
        Self {}
    }

    /// Calculate working days between two dates, excluding weekends and public holidays
    pub fn calculate_working_days(
        &self,
        start_date: NaiveDate,
        end_date: NaiveDate,
        public_holidays: &[PublicHoliday],
        half_day: bool,
    ) -> Decimal {
        if end_date < start_date {
            return Decimal::ZERO;
        }

        let holiday_dates: HashSet<NaiveDate> = public_holidays
            .iter()
            .map(|h| h.date)
            .collect();

        let mut working_days = 0;
        let mut current = start_date;

        while current <= end_date {
            // Check if it's a weekday (Mon-Fri)
            let weekday = current.weekday().num_days_from_monday();
            if weekday < 5 && !holiday_dates.contains(&current) {
                working_days += 1;
            }
            current = current.succ_opt().unwrap_or(current);
        }

        let days = Decimal::from(working_days);
        if half_day && working_days > 0 {
            days - dec!(0.5)
        } else {
            days
        }
    }

    /// Validate leave request
    pub fn validate_leave_request(
        &self,
        request: &CreateLeaveRequest,
        leave_type: &LeaveType,
        balance: &LeaveBalance,
        employee_gender: Option<&str>,
        public_holidays: &[PublicHoliday],
    ) -> Result<Decimal, LeaveError> {
        // Validate date range
        if request.end_date < request.start_date {
            return Err(LeaveError::InvalidDateRange);
        }

        // Calculate days
        let days = self.calculate_working_days(
            request.start_date,
            request.end_date,
            public_holidays,
            request.half_day,
        );

        // Check gender restriction
        if let Some(restriction) = &leave_type.gender_restriction {
            if let Some(gender) = employee_gender {
                if restriction != gender {
                    return Err(LeaveError::GenderRestricted(restriction.clone()));
                }
            }
        }

        // Check balance
        let available = balance.available_days();
        if days > available {
            return Err(LeaveError::InsufficientBalance {
                available,
                requested: days,
            });
        }

        // Check document requirement
        if leave_type.requires_document && days > Decimal::from(leave_type.document_threshold_days) {
            if request.reason.is_none() {
                return Err(LeaveError::DocumentRequired(leave_type.document_threshold_days));
            }
        }

        // Check relief officer requirement (for leave > 3 days)
        if days > dec!(3) && request.relief_officer_id.is_none() {
            return Err(LeaveError::ReliefOfficerRequired);
        }

        Ok(days)
    }

    /// Create a leave request
    pub fn create_leave_request(
        &self,
        employee_id: Uuid,
        request: CreateLeaveRequest,
        leave_type: &LeaveType,
        balance: &LeaveBalance,
        employee_gender: Option<&str>,
        public_holidays: &[PublicHoliday],
    ) -> Result<LeaveRequest, LeaveError> {
        // Validate and calculate days
        let days = self.validate_leave_request(
            &request,
            leave_type,
            balance,
            employee_gender,
            public_holidays,
        )?;

        let now = Utc::now();
        
        Ok(LeaveRequest {
            id: Uuid::new_v4(),
            employee_id,
            employee_name: None,
            leave_type_id: request.leave_type_id,
            leave_type_name: Some(leave_type.name.clone()),
            
            start_date: request.start_date,
            end_date: request.end_date,
            days_requested: days,
            half_day: request.half_day,
            
            reason: request.reason,
            document_url: None,
            
            relief_officer_id: request.relief_officer_id,
            relief_officer_name: None,
            handover_notes: request.handover_notes,
            
            status: LeaveRequestStatus::Pending,
            approved_by: None,
            approver_name: None,
            approved_at: None,
            rejection_reason: None,
            
            created_at: now,
            updated_at: now,
        })
    }

    /// Approve a leave request
    pub fn approve_leave(
        &self,
        request: &mut LeaveRequest,
        balance: &mut LeaveBalance,
        approver_id: Uuid,
    ) -> Result<(), LeaveError> {
        if request.status != LeaveRequestStatus::Pending {
            return Err(LeaveError::InvalidStatus(format!("{:?}", request.status)));
        }

        // Update balance
        balance.pending_days -= request.days_requested;
        balance.used_days += request.days_requested;
        balance.updated_at = Utc::now();

        // Update request
        request.status = LeaveRequestStatus::Approved;
        request.approved_by = Some(approver_id);
        request.approved_at = Some(Utc::now());
        request.updated_at = Utc::now();

        Ok(())
    }

    /// Reject a leave request
    pub fn reject_leave(
        &self,
        request: &mut LeaveRequest,
        balance: &mut LeaveBalance,
        approver_id: Uuid,
        reason: Option<String>,
    ) -> Result<(), LeaveError> {
        if request.status != LeaveRequestStatus::Pending {
            return Err(LeaveError::InvalidStatus(format!("{:?}", request.status)));
        }

        // Restore pending days to available
        balance.pending_days -= request.days_requested;
        balance.updated_at = Utc::now();

        // Update request
        request.status = LeaveRequestStatus::Rejected;
        request.approved_by = Some(approver_id);
        request.approved_at = Some(Utc::now());
        request.rejection_reason = reason;
        request.updated_at = Utc::now();

        Ok(())
    }

    /// Cancel a leave request
    pub fn cancel_leave(
        &self,
        request: &mut LeaveRequest,
        balance: &mut LeaveBalance,
    ) -> Result<(), LeaveError> {
        match request.status {
            LeaveRequestStatus::Pending => {
                // Restore pending days
                balance.pending_days -= request.days_requested;
            }
            LeaveRequestStatus::Approved => {
                // Restore used days (only if leave hasn't started)
                let today = Utc::now().date_naive();
                if request.start_date > today {
                    balance.used_days -= request.days_requested;
                } else {
                    return Err(LeaveError::Validation(
                        "Cannot cancel leave that has already started".to_string()
                    ));
                }
            }
            _ => {
                return Err(LeaveError::InvalidStatus(format!("{:?}", request.status)));
            }
        }

        request.status = LeaveRequestStatus::Cancelled;
        request.updated_at = Utc::now();
        balance.updated_at = Utc::now();

        Ok(())
    }

    /// Initialize leave balances for a new year
    pub fn initialize_annual_balances(
        &self,
        employee_id: Uuid,
        year: i32,
        leave_types: &[LeaveType],
        previous_balances: Option<&[LeaveBalance]>,
    ) -> Vec<LeaveBalance> {
        let now = Utc::now();
        
        leave_types.iter().map(|lt| {
            // Calculate carry over from previous year
            let carried_over = if let Some(prev) = previous_balances {
                prev.iter()
                    .find(|b| b.leave_type_id == lt.id)
                    .map(|b| {
                        let remaining = b.entitled_days - b.used_days;
                        remaining.min(Decimal::from(lt.max_carry_over))
                    })
                    .unwrap_or(Decimal::ZERO)
            } else {
                Decimal::ZERO
            };

            LeaveBalance {
                id: Uuid::new_v4(),
                employee_id,
                leave_type_id: lt.id,
                leave_type_name: lt.name.clone(),
                year,
                entitled_days: Decimal::from(lt.default_days),
                used_days: Decimal::ZERO,
                pending_days: Decimal::ZERO,
                carried_over,
                created_at: now,
                updated_at: now,
            }
        }).collect()
    }

    /// Get leave balance summary
    pub fn get_balance_summary(
        &self,
        employee_id: Uuid,
        year: i32,
        balances: Vec<LeaveBalance>,
    ) -> LeaveBalanceSummary {
        let total_entitled: Decimal = balances.iter().map(|b| b.entitled_days + b.carried_over).sum();
        let total_used: Decimal = balances.iter().map(|b| b.used_days).sum();
        let total_pending: Decimal = balances.iter().map(|b| b.pending_days).sum();
        let total_available: Decimal = balances.iter().map(|b| b.available_days()).sum();

        LeaveBalanceSummary {
            employee_id,
            year,
            balances,
            total_entitled,
            total_used,
            total_pending,
            total_available,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_leave_type() -> LeaveType {
        LeaveType {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Annual Leave".to_string(),
            code: "annual".to_string(),
            default_days: 21,
            is_paid: true,
            requires_approval: true,
            requires_document: false,
            document_threshold_days: 0,
            max_carry_over: 5,
            gender_restriction: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn create_test_balance(leave_type_id: Uuid, employee_id: Uuid) -> LeaveBalance {
        LeaveBalance {
            id: Uuid::new_v4(),
            employee_id,
            leave_type_id,
            leave_type_name: "Annual Leave".to_string(),
            year: 2024,
            entitled_days: dec!(21),
            used_days: dec!(5),
            pending_days: dec!(0),
            carried_over: dec!(3),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[test]
    fn test_calculate_working_days() {
        let service = LeaveService::new();
        
        // Monday to Friday (5 days)
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();  // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 12).unwrap();   // Friday
        
        let days = service.calculate_working_days(start, end, &[], false);
        assert_eq!(days, dec!(5));
    }

    #[test]
    fn test_calculate_working_days_with_weekend() {
        let service = LeaveService::new();
        
        // Monday to next Monday (8 days, but only 6 working days)
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();  // Monday
        let end = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();   // Monday
        
        let days = service.calculate_working_days(start, end, &[], false);
        assert_eq!(days, dec!(6)); // Mon-Fri + Mon = 6
    }

    #[test]
    fn test_calculate_working_days_half_day() {
        let service = LeaveService::new();
        
        let start = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 1, 10).unwrap();
        
        let days = service.calculate_working_days(start, end, &[], true);
        assert_eq!(days, dec!(2.5)); // 3 days - 0.5 = 2.5
    }

    #[test]
    fn test_create_leave_request() {
        let service = LeaveService::new();
        let leave_type = create_test_leave_type();
        let employee_id = Uuid::new_v4();
        let balance = create_test_balance(leave_type.id, employee_id);

        let request = CreateLeaveRequest {
            leave_type_id: leave_type.id,
            start_date: NaiveDate::from_ymd_opt(2024, 6, 3).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 6, 5).unwrap(),
            half_day: false,
            reason: Some("Vacation".to_string()),
            relief_officer_id: None,  // < 3 days, not required
            handover_notes: None,
        };

        let result = service.create_leave_request(
            employee_id,
            request,
            &leave_type,
            &balance,
            None,
            &[],
        );

        assert!(result.is_ok());
        let leave_request = result.unwrap();
        assert_eq!(leave_request.status, LeaveRequestStatus::Pending);
        assert_eq!(leave_request.days_requested, dec!(3));
    }

    #[test]
    fn test_insufficient_balance() {
        let service = LeaveService::new();
        let leave_type = create_test_leave_type();
        let employee_id = Uuid::new_v4();
        let mut balance = create_test_balance(leave_type.id, employee_id);
        balance.used_days = dec!(20); // Only 4 days available (21 + 3 - 20)

        let request = CreateLeaveRequest {
            leave_type_id: leave_type.id,
            start_date: NaiveDate::from_ymd_opt(2024, 6, 3).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 6, 14).unwrap(), // 10 days
            half_day: false,
            reason: Some("Long vacation".to_string()),
            relief_officer_id: Some(Uuid::new_v4()),
            handover_notes: Some("Handover notes".to_string()),
        };

        let result = service.create_leave_request(
            employee_id,
            request,
            &leave_type,
            &balance,
            None,
            &[],
        );

        assert!(matches!(result, Err(LeaveError::InsufficientBalance { .. })));
    }

    #[test]
    fn test_approve_leave() {
        let service = LeaveService::new();
        let leave_type = create_test_leave_type();
        let employee_id = Uuid::new_v4();
        let mut balance = create_test_balance(leave_type.id, employee_id);
        balance.pending_days = dec!(3); // Simulate pending request

        let mut request = LeaveRequest {
            id: Uuid::new_v4(),
            employee_id,
            employee_name: None,
            leave_type_id: leave_type.id,
            leave_type_name: Some("Annual Leave".to_string()),
            start_date: NaiveDate::from_ymd_opt(2024, 6, 3).unwrap(),
            end_date: NaiveDate::from_ymd_opt(2024, 6, 5).unwrap(),
            days_requested: dec!(3),
            half_day: false,
            reason: None,
            document_url: None,
            relief_officer_id: None,
            relief_officer_name: None,
            handover_notes: None,
            status: LeaveRequestStatus::Pending,
            approved_by: None,
            approver_name: None,
            approved_at: None,
            rejection_reason: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let approver_id = Uuid::new_v4();
        let result = service.approve_leave(&mut request, &mut balance, approver_id);

        assert!(result.is_ok());
        assert_eq!(request.status, LeaveRequestStatus::Approved);
        assert_eq!(balance.pending_days, dec!(0));
        assert_eq!(balance.used_days, dec!(8)); // 5 + 3
    }
}
