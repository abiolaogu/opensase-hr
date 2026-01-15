//! Compliance & Audit Module
//!
//! Global compliance framework: GDPR, CCPA, LGPD, PDPA, POPIA, APPI, PIPL.
//! Data residency rules for 180+ countries.

pub mod models;
pub mod global_compliance;

pub use models::*;
pub use global_compliance::{
    PolicyEngine, GdprEvaluator, DataResidencyEngine, DataClassifier,
    ComplianceFramework, DataCategory, LegalBasis, ResidencyRequirement,
    TransferMechanism, DsrType, ComplianceRegistry,
};
