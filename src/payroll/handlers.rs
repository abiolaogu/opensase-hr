//! Payroll API Handlers
//!
//! REST API endpoints for payroll operations.

use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

use super::{
    models::*,
    service::{PayrollService, TaxPreviewResponse},
};

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub payroll_service: PayrollService,
    // In real app: database pool, auth service, etc.
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            payroll_service: PayrollService::new(),
        }
    }
}

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

/// List payroll runs query parameters
#[derive(Debug, Deserialize)]
pub struct ListPayrollRunsQuery {
    pub status: Option<String>,
    pub year: Option<i32>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

/// Create payroll run handler
/// 
/// POST /api/v1/payroll/runs
pub async fn create_payroll_run(
    State(state): State<AppState>,
    Json(request): Json<CreatePayrollRunRequest>,
) -> impl IntoResponse {
    // In real implementation, get tenant_id from auth context
    let tenant_id = Uuid::new_v4();
    
    match state.payroll_service.create_payroll_run(tenant_id, request) {
        Ok(run) => (StatusCode::CREATED, Json(ApiResponse::success(run))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<PayrollRun>::error(e.to_string()))),
    }
}

/// Get payroll run by ID
/// 
/// GET /api/v1/payroll/runs/:id
pub async fn get_payroll_run(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // In real implementation, fetch from database
    Json(ApiResponse::<PayrollRun>::error(format!("Payroll run {} not found (stub)", id)))
}

/// List payroll runs
/// 
/// GET /api/v1/payroll/runs
pub async fn list_payroll_runs(
    State(_state): State<AppState>,
    Query(_query): Query<ListPayrollRunsQuery>,
) -> impl IntoResponse {
    // In real implementation, fetch from database with filters
    let runs: Vec<PayrollRun> = vec![];
    Json(ApiResponse::success(runs))
}

/// Process payroll handler
/// 
/// POST /api/v1/payroll/runs/:id/process
pub async fn process_payroll_run(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_request): Json<ProcessPayrollRequest>,
) -> impl IntoResponse {
    // In real implementation:
    // 1. Fetch payroll run from DB
    // 2. Fetch active employees with salary configs
    // 3. Process payroll
    // 4. Save payroll items to DB
    // 5. Update payroll run status
    
    Json(ApiResponse::<PayrollRun>::error(format!("Processing payroll {} (stub)", id)))
}

/// Approve payroll handler
/// 
/// POST /api/v1/payroll/runs/:id/approve
pub async fn approve_payroll_run(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    // In real implementation, get approver from auth context
    Json(ApiResponse::<PayrollRun>::error(format!("Approving payroll {} (stub)", id)))
}

/// Get payroll items (payslips) for a run
/// 
/// GET /api/v1/payroll/runs/:id/items
pub async fn get_payroll_items(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    let items: Vec<PayrollItem> = vec![];
    Json(ApiResponse::success(items))
}

/// Get employee payroll history
/// 
/// GET /api/v1/payroll/employees/:employee_id/history
pub async fn get_employee_payroll_history(
    State(_state): State<AppState>,
    Path(employee_id): Path<Uuid>,
) -> impl IntoResponse {
    let items: Vec<PayrollItem> = vec![];
    Json(ApiResponse::success(items))
}

/// Tax calculation preview request
#[derive(Debug, Deserialize)]
pub struct TaxCalculateRequest {
    pub monthly_gross: Decimal,
}

/// Calculate tax preview
/// 
/// POST /api/v1/payroll/tax/calculate
pub async fn calculate_tax_preview(
    State(state): State<AppState>,
    Json(request): Json<TaxCalculateRequest>,
) -> impl IntoResponse {
    let preview = state.payroll_service.calculate_tax_preview(request.monthly_gross);
    Json(ApiResponse::success(preview))
}

/// Generate P9A tax return
/// 
/// GET /api/v1/payroll/reports/p9/:year/:employee_id
pub async fn generate_p9a(
    State(_state): State<AppState>,
    Path((year, employee_id)): Path<(i32, Uuid)>,
) -> impl IntoResponse {
    // In real implementation, aggregate all payroll items for the year
    let p9a = P9AReturn {
        year,
        employee_id,
        employee_name: "Employee Name".to_string(),
        tin: Some("12345678-0001".to_string()),
        monthly_earnings: vec![],
        annual_gross: Decimal::ZERO,
        annual_tax_deducted: Decimal::ZERO,
        annual_pension: Decimal::ZERO,
    };
    
    Json(ApiResponse::success(p9a))
}

/// Generate pension schedule
/// 
/// GET /api/v1/payroll/reports/pension/:payroll_run_id
pub async fn generate_pension_schedule(
    State(_state): State<AppState>,
    Path(payroll_run_id): Path<Uuid>,
) -> impl IntoResponse {
    let schedules: Vec<PensionSchedule> = vec![];
    Json(ApiResponse::success(schedules))
}

/// Create payroll routes
pub fn payroll_routes() -> axum::Router<AppState> {
    use axum::routing::{get, post};
    
    axum::Router::new()
        // Payroll Runs
        .route("/runs", post(create_payroll_run))
        .route("/runs", get(list_payroll_runs))
        .route("/runs/:id", get(get_payroll_run))
        .route("/runs/:id/process", post(process_payroll_run))
        .route("/runs/:id/approve", post(approve_payroll_run))
        .route("/runs/:id/items", get(get_payroll_items))
        
        // Employee History
        .route("/employees/:employee_id/history", get(get_employee_payroll_history))
        
        // Tax Preview
        .route("/tax/calculate", post(calculate_tax_preview))
        
        // Reports
        .route("/reports/p9/:year/:employee_id", get(generate_p9a))
        .route("/reports/pension/:payroll_run_id", get(generate_pension_schedule))
}
