//! Leave Management API Handlers
//!
//! REST API endpoints for leave operations.

use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::models::*;
use super::service::LeaveService;

/// API Response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message.into()),
        }
    }
}

/// Shared leave state
#[derive(Clone)]
pub struct LeaveAppState {
    pub leave_service: LeaveService,
}

impl Default for LeaveAppState {
    fn default() -> Self {
        Self {
            leave_service: LeaveService::new(),
        }
    }
}

/// Query params for leave requests
#[derive(Debug, Deserialize)]
pub struct LeaveRequestsQuery {
    pub status: Option<String>,
    pub year: Option<i32>,
    pub employee_id: Option<Uuid>,
}

/// Get leave types
/// 
/// GET /api/v1/leave/types
pub async fn list_leave_types(
    State(_state): State<LeaveAppState>,
) -> impl IntoResponse {
    // In real implementation, fetch from database
    let types: Vec<LeaveType> = vec![];
    Json(ApiResponse::success(types))
}

/// Create leave type (admin)
/// 
/// POST /api/v1/leave/types
pub async fn create_leave_type(
    State(_state): State<LeaveAppState>,
    Json(_leave_type): Json<LeaveType>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(ApiResponse::<LeaveType>::error("Stub")))
}

/// Get my leave balances
/// 
/// GET /api/v1/leave/balances
pub async fn get_my_balances(
    State(_state): State<LeaveAppState>,
) -> impl IntoResponse {
    // In real implementation, get employee_id from auth context
    let summary = LeaveBalanceSummary {
        employee_id: Uuid::new_v4(),
        year: 2024,
        balances: vec![],
        total_entitled: rust_decimal_macros::dec!(0),
        total_used: rust_decimal_macros::dec!(0),
        total_pending: rust_decimal_macros::dec!(0),
        total_available: rust_decimal_macros::dec!(0),
    };
    Json(ApiResponse::success(summary))
}

/// Get employee balances (manager only)
/// 
/// GET /api/v1/leave/balances/:employee_id
pub async fn get_employee_balances(
    State(_state): State<LeaveAppState>,
    Path(employee_id): Path<Uuid>,
) -> impl IntoResponse {
    let summary = LeaveBalanceSummary {
        employee_id,
        year: 2024,
        balances: vec![],
        total_entitled: rust_decimal_macros::dec!(0),
        total_used: rust_decimal_macros::dec!(0),
        total_pending: rust_decimal_macros::dec!(0),
        total_available: rust_decimal_macros::dec!(0),
    };
    Json(ApiResponse::success(summary))
}

/// Create leave request
/// 
/// POST /api/v1/leave/requests
pub async fn create_leave_request(
    State(_state): State<LeaveAppState>,
    Json(_request): Json<CreateLeaveRequest>,
) -> impl IntoResponse {
    // In real implementation:
    // 1. Get employee_id from auth
    // 2. Fetch leave type
    // 3. Fetch current balance
    // 4. Fetch public holidays
    // 5. Create request
    // 6. Update pending balance
    // 7. Send notification to manager
    (StatusCode::CREATED, Json(ApiResponse::<LeaveRequest>::error("Stub")))
}

/// Get my leave requests
/// 
/// GET /api/v1/leave/requests
pub async fn get_my_requests(
    State(_state): State<LeaveAppState>,
    Query(_query): Query<LeaveRequestsQuery>,
) -> impl IntoResponse {
    let requests: Vec<LeaveRequest> = vec![];
    Json(ApiResponse::success(requests))
}

/// Get pending approvals (manager)
/// 
/// GET /api/v1/leave/requests/pending
pub async fn get_pending_approvals(
    State(_state): State<LeaveAppState>,
) -> impl IntoResponse {
    let requests: Vec<LeaveRequest> = vec![];
    Json(ApiResponse::success(requests))
}

/// Get leave request details
/// 
/// GET /api/v1/leave/requests/:id
pub async fn get_request(
    State(_state): State<LeaveAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    Json(ApiResponse::<LeaveRequest>::error(format!("Request {} not found", id)))
}

/// Approve leave request
/// 
/// PUT /api/v1/leave/requests/:id/approve
pub async fn approve_request(
    State(_state): State<LeaveAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // In real implementation:
    // 1. Get approver_id from auth
    // 2. Fetch request
    // 3. Approve using service
    // 4. Update database
    // 5. Send notification
    Json(ApiResponse::<LeaveRequest>::error(format!("Approving {} (stub)", id)))
}

/// Reject leave request
/// 
/// PUT /api/v1/leave/requests/:id/reject
pub async fn reject_request(
    State(_state): State<LeaveAppState>,
    Path(id): Path<Uuid>,
    Json(decision): Json<LeaveDecisionRequest>,
) -> impl IntoResponse {
    Json(ApiResponse::<LeaveRequest>::error(format!("Rejecting {} (stub): {:?}", id, decision.rejection_reason)))
}

/// Cancel leave request
/// 
/// PUT /api/v1/leave/requests/:id/cancel
pub async fn cancel_request(
    State(_state): State<LeaveAppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    Json(ApiResponse::<LeaveRequest>::error(format!("Cancelling {} (stub)", id)))
}

/// Team leave calendar query
#[derive(Debug, Deserialize)]
pub struct CalendarQuery {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub department_id: Option<Uuid>,
}

/// Get team leave calendar
/// 
/// GET /api/v1/leave/calendar
pub async fn get_calendar(
    State(_state): State<LeaveAppState>,
    Query(_query): Query<CalendarQuery>,
) -> impl IntoResponse {
    let entries: Vec<LeaveCalendarEntry> = vec![];
    Json(ApiResponse::success(entries))
}

/// Get public holidays
/// 
/// GET /api/v1/leave/holidays
pub async fn get_holidays(
    State(_state): State<LeaveAppState>,
    Query(year): Query<Option<i32>>,
) -> impl IntoResponse {
    let _year = year.unwrap_or(2024);
    let holidays: Vec<PublicHoliday> = vec![];
    Json(ApiResponse::success(holidays))
}

/// Create leave routes
pub fn leave_routes() -> axum::Router<LeaveAppState> {
    use axum::routing::{get, post, put};
    
    axum::Router::new()
        // Leave Types
        .route("/types", get(list_leave_types))
        .route("/types", post(create_leave_type))
        
        // Balances
        .route("/balances", get(get_my_balances))
        .route("/balances/:employee_id", get(get_employee_balances))
        
        // Requests
        .route("/requests", post(create_leave_request))
        .route("/requests", get(get_my_requests))
        .route("/requests/pending", get(get_pending_approvals))
        .route("/requests/:id", get(get_request))
        .route("/requests/:id/approve", put(approve_request))
        .route("/requests/:id/reject", put(reject_request))
        .route("/requests/:id/cancel", put(cancel_request))
        
        // Calendar & Holidays
        .route("/calendar", get(get_calendar))
        .route("/holidays", get(get_holidays))
}
