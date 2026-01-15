//! OpenSASE HR & Payroll Platform
//!
//! Enterprise-grade HR platform with Nigerian market compliance.
//! Replaces BambooHR, Gusto, Workday, ADP.
//!
//! ## Modules
//!
//! - **domain**: Core DDD domain with aggregates and value objects
//! - **payroll**: Nigerian payroll with PAYE, PenCom, NHF
//! - **leave**: Leave management with Nigerian leave types
//! - **performance**: Performance reviews, goals, 360° feedback
//! - **recruitment**: Job postings and AI-powered CV scoring
//! - **benefits**: HMO enrollment and claims processing
//! - **compliance**: NDPR compliance and audit logging
//! - **auth**: JWT authentication and RBAC
//! - **sms**: SMS/USSD fallback channels for emerging markets
//!
//! ## Nigerian Compliance Features
//!
//! - PAYE Tax Calculator (2024 bands)
//! - PenCom Pension (8%/10% contributions)
//! - NHF (2.5% of basic salary)
//! - NSITF and ITF contributions
//! - NDPR Data Subject Requests
//! - Nigerian public holidays
//!
//! ## West Africa Coverage
//!
//! - Ghana (SSNIT, Tier 1/2/3)
//! - UEMOA Zone (CI, SN, ML, BF, NE, GW, BJ, TG)
//! - Mobile Money: OPay, MTN MoMo, Orange Money, Wave

// Core domain (from original)
pub mod domain;

// New enriched modules
pub mod payroll;
pub mod leave;
pub mod performance;
pub mod recruitment;
pub mod benefits;
pub mod compliance;
pub mod auth;
pub mod sms;
pub mod ops;

// Re-exports from domain
pub use domain::aggregates::{Employee, EmployeeError, PayrollRun, PayrollError};
pub use domain::value_objects::{EmployeeId, TaxId, PayRate, PayType, PayFrequency};
pub use domain::events::{DomainEvent, EmployeeEvent, PayrollEvent};

// Re-exports from new modules
pub use payroll::{PayrollService, NigerianTaxCalculator, PensionCalculator};
pub use payroll::{GhanaTaxCalculator, UemoaTaxCalculator, WestAfricaTaxRegistry, WestAfricaMobileMoneyRegistry};
pub use leave::LeaveService;
pub use performance::PerformanceService;
pub use recruitment::{RecruitmentService, AiCvScorer};
pub use auth::{Role, Permission, Claims, JwtService};
pub use sms::{UssdRegistry, SmsTemplateRegistry};

