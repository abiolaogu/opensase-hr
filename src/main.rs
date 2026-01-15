//! OpenSASE HR API Server
//!
//! Nigerian-compliant HR platform with:
//! - PAYE tax calculation
//! - PenCom pension
//! - Leave management
//! - Performance reviews
//! - Recruitment with AI scoring
//! - Benefits administration
//! - NDPR compliance

use axum::{
    routing::{get, post},
    Router,
    Json,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import modules from library
use sase_hr::{
    payroll::{self, PayrollService},
    leave::{self, LeaveService},
    auth::{Role, JwtService},
};

/// Health check response
#[derive(serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    modules: Vec<&'static str>,
}

/// Health check endpoint
async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
        modules: vec![
            "payroll",
            "leave",
            "performance",
            "recruitment",
            "benefits",
            "compliance",
            "auth",
        ],
    })
}

/// API info endpoint
async fn api_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "OpenSASE HR API",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "Enterprise HR platform with Nigerian compliance",
        "features": {
            "payroll": {
                "paye_tax": true,
                "pension_pencom": true,
                "nhf_deduction": true,
                "nsitf": true,
                "itf": true
            },
            "leave": {
                "nigerian_types": true,
                "public_holidays": true,
                "balance_tracking": true
            },
            "performance": {
                "goals": true,
                "360_feedback": true,
                "rating_categories": true
            },
            "recruitment": {
                "ai_cv_scoring": true,
                "pipeline_stages": true
            },
            "compliance": {
                "ndpr": true,
                "audit_logging": true
            }
        }
    }))
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "info".into()))
        .init();

    tracing::info!("Starting OpenSASE HR API Server v{}", env!("CARGO_PKG_VERSION"));

    // Initialize services
    let _payroll_service = PayrollService::new();
    let _leave_service = LeaveService::new();
    let _jwt_service = JwtService::new("your-secret-key".to_string());

    // Build router
    let app = Router::new()
        // Health & Info
        .route("/health", get(health_check))
        .route("/api/info", get(api_info))
        
        // API v1 routes (stubs - implement with database later)
        .route("/api/v1/payroll/tax/calculate", post(calculate_tax_preview))
        
        // CORS
        .layer(CorsLayer::permissive());

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on http://{}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Tax calculation preview endpoint
async fn calculate_tax_preview(
    Json(request): Json<TaxCalculateRequest>,
) -> Json<serde_json::Value> {
    let service = PayrollService::new();
    let preview = service.calculate_tax_preview(request.monthly_gross);
    
    Json(serde_json::json!({
        "success": true,
        "data": preview
    }))
}

#[derive(serde::Deserialize)]
struct TaxCalculateRequest {
    monthly_gross: rust_decimal::Decimal,
}
