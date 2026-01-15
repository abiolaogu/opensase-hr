//! OpenSASE HR - Self-hosted Human Resources Management

use anyhow::Result;
use axum::{extract::{Path, Query, State}, http::StatusCode, response::IntoResponse, routing::{get, post, put, delete}, Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub employee_number: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub job_title: Option<String>,
    pub employment_type: String,
    pub hire_date: NaiveDate,
    pub status: String,
    pub phone: Option<String>,
    pub address: Option<serde_json::Value>,
    pub emergency_contact: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Department {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i32,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub nats: Option<async_nats::Client>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = PgPoolOptions::new().max_connections(10)
        .connect(&std::env::var("DATABASE_URL")?).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let nats = std::env::var("NATS_URL").ok()
        .and_then(|url| futures::executor::block_on(async_nats::connect(&url)).ok());

    let state = AppState { db, nats };
    let app = Router::new()
        .route("/health", get(|| async { Json(serde_json::json!({"status": "healthy", "service": "opensase-hr"})) }))
        .route("/api/v1/employees", get(list_employees).post(create_employee))
        .route("/api/v1/employees/:id", get(get_employee).put(update_employee).delete(delete_employee))
        .route("/api/v1/departments", get(list_departments).post(create_department))
        .route("/api/v1/departments/:id", get(get_department))
        .route("/api/v1/leave", get(list_leave_requests).post(create_leave_request))
        .route("/api/v1/leave/:id/approve", post(approve_leave))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
        .with_state(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8082".to_string());
    tracing::info!("🚀 OpenSASE HR listening on 0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[derive(Debug, Deserialize)] pub struct ListParams { pub page: Option<u32>, pub per_page: Option<u32> }
#[derive(Debug, Serialize)] pub struct PaginatedResponse<T> { pub data: Vec<T>, pub total: i64, pub page: u32 }

async fn list_employees(State(state): State<AppState>, Query(p): Query<ListParams>) -> Result<Json<PaginatedResponse<Employee>>, (StatusCode, String)> {
    let page = p.page.unwrap_or(1).max(1);
    let per_page = p.per_page.unwrap_or(20).min(100);
    let employees = sqlx::query_as::<_, Employee>("SELECT * FROM employees ORDER BY created_at DESC LIMIT $1 OFFSET $2")
        .bind(per_page as i64).bind(((page - 1) * per_page) as i64)
        .fetch_all(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM employees").fetch_one(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(PaginatedResponse { data: employees, total: total.0, page }))
}

async fn get_employee(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Employee>, (StatusCode, String)> {
    sqlx::query_as::<_, Employee>("SELECT * FROM employees WHERE id = $1").bind(id)
        .fetch_optional(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map(Json).ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))
}

#[derive(Debug, Deserialize)]
pub struct CreateEmployeeRequest { pub email: String, pub first_name: String, pub last_name: String, pub department_id: Option<Uuid>, pub job_title: Option<String>, pub hire_date: NaiveDate }

async fn create_employee(State(state): State<AppState>, Json(req): Json<CreateEmployeeRequest>) -> Result<(StatusCode, Json<Employee>), (StatusCode, String)> {
    let id = Uuid::now_v7();
    let emp_num = format!("EMP-{:06}", rand::random::<u32>() % 1000000);
    let emp = sqlx::query_as::<_, Employee>(
        "INSERT INTO employees (id, employee_number, email, first_name, last_name, department_id, job_title, hire_date, employment_type, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'full_time', 'active', NOW(), NOW()) RETURNING *"
    ).bind(id).bind(&emp_num).bind(&req.email).bind(&req.first_name).bind(&req.last_name).bind(req.department_id).bind(&req.job_title).bind(req.hire_date)
    .fetch_one(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(emp)))
}

async fn update_employee(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<CreateEmployeeRequest>) -> Result<Json<Employee>, (StatusCode, String)> {
    let emp = sqlx::query_as::<_, Employee>("UPDATE employees SET email = $2, first_name = $3, last_name = $4, department_id = $5, job_title = $6, updated_at = NOW() WHERE id = $1 RETURNING *")
        .bind(id).bind(&req.email).bind(&req.first_name).bind(&req.last_name).bind(req.department_id).bind(&req.job_title)
        .fetch_optional(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))?;
    Ok(Json(emp))
}

async fn delete_employee(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM employees WHERE id = $1").bind(id).execute(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}

async fn list_departments(State(state): State<AppState>) -> Result<Json<Vec<Department>>, (StatusCode, String)> {
    let depts = sqlx::query_as::<_, Department>("SELECT * FROM departments ORDER BY name").fetch_all(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(depts))
}

async fn get_department(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<Department>, (StatusCode, String)> {
    sqlx::query_as::<_, Department>("SELECT * FROM departments WHERE id = $1").bind(id)
        .fetch_optional(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .map(Json).ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))
}

#[derive(Debug, Deserialize)] pub struct CreateDepartmentRequest { pub name: String, pub description: Option<String>, pub manager_id: Option<Uuid> }

async fn create_department(State(state): State<AppState>, Json(req): Json<CreateDepartmentRequest>) -> Result<(StatusCode, Json<Department>), (StatusCode, String)> {
    let dept = sqlx::query_as::<_, Department>("INSERT INTO departments (id, name, description, manager_id, created_at) VALUES ($1, $2, $3, $4, NOW()) RETURNING *")
        .bind(Uuid::now_v7()).bind(&req.name).bind(&req.description).bind(req.manager_id)
        .fetch_one(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(dept)))
}

async fn list_leave_requests(State(state): State<AppState>) -> Result<Json<Vec<LeaveRequest>>, (StatusCode, String)> {
    let leaves = sqlx::query_as::<_, LeaveRequest>("SELECT * FROM leave_requests ORDER BY created_at DESC").fetch_all(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(leaves))
}

#[derive(Debug, Deserialize)] pub struct CreateLeaveRequest { pub employee_id: Uuid, pub leave_type: String, pub start_date: NaiveDate, pub end_date: NaiveDate, pub reason: Option<String> }

async fn create_leave_request(State(state): State<AppState>, Json(req): Json<CreateLeaveRequest>) -> Result<(StatusCode, Json<LeaveRequest>), (StatusCode, String)> {
    let days = (req.end_date - req.start_date).num_days() as i32 + 1;
    let leave = sqlx::query_as::<_, LeaveRequest>("INSERT INTO leave_requests (id, employee_id, leave_type, start_date, end_date, days, reason, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'pending', NOW()) RETURNING *")
        .bind(Uuid::now_v7()).bind(req.employee_id).bind(&req.leave_type).bind(req.start_date).bind(req.end_date).bind(days).bind(&req.reason)
        .fetch_one(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(leave)))
}

async fn approve_leave(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<Json<LeaveRequest>, (StatusCode, String)> {
    let leave = sqlx::query_as::<_, LeaveRequest>("UPDATE leave_requests SET status = 'approved' WHERE id = $1 RETURNING *").bind(id)
        .fetch_optional(&state.db).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Not found".to_string()))?;
    Ok(Json(leave))
}
