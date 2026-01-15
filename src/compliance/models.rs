//! Compliance Models

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use uuid::Uuid;

/// Audit action type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    Create,
    Update,
    Delete,
    View,
    Export,
    Login,
    Logout,
}

/// Actor type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorType {
    User,
    System,
    Api,
}

/// Immutable audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub action: AuditAction,
    pub actor_id: Option<Uuid>,
    pub actor_type: ActorType,
    pub changes: Option<AuditChanges>,
    pub metadata: serde_json::Value,
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Audit changes (before/after)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditChanges {
    pub before: serde_json::Value,
    pub after: serde_json::Value,
}

impl AuditLog {
    pub fn new(
        tenant_id: Uuid,
        entity_type: impl Into<String>,
        entity_id: Uuid,
        action: AuditAction,
        actor_id: Option<Uuid>,
        actor_type: ActorType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            entity_type: entity_type.into(),
            entity_id,
            action,
            actor_id,
            actor_type,
            changes: None,
            metadata: serde_json::json!({}),
            ip_address: None,
            user_agent: None,
            created_at: Utc::now(),
        }
    }

    pub fn with_changes(mut self, before: serde_json::Value, after: serde_json::Value) -> Self {
        self.changes = Some(AuditChanges { before, after });
        self
    }

    pub fn with_ip(mut self, ip: IpAddr) -> Self {
        self.ip_address = Some(ip);
        self
    }
}

/// Data Subject Request type (NDPR)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DsrType {
    Access,        // Right to access personal data
    Rectification, // Right to correct data
    Erasure,       // Right to be forgotten
    Portability,   // Right to data portability
}

/// DSR Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DsrStatus {
    Pending,
    Processing,
    Completed,
    Rejected,
}

/// Data Subject Request (NDPR compliance)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSubjectRequest {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub request_type: DsrType,
    pub subject_email: String,
    pub subject_name: Option<String>,
    pub description: Option<String>,
    pub status: DsrStatus,
    pub processed_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub response: Option<String>,
    pub due_date: NaiveDate,  // 30 days per NDPR
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DataSubjectRequest {
    pub fn new(
        tenant_id: Uuid,
        request_type: DsrType,
        subject_email: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        let due = now.date_naive() + chrono::Duration::days(30);
        
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            request_type,
            subject_email,
            subject_name: None,
            description,
            status: DsrStatus::Pending,
            processed_by: None,
            processed_at: None,
            response: None,
            due_date: due,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_overdue(&self) -> bool {
        self.status == DsrStatus::Pending 
            && Utc::now().date_naive() > self.due_date
    }
}

/// Compliance checklist item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceChecklistItem {
    pub id: String,
    pub category: String,
    pub requirement: String,
    pub is_completed: bool,
    pub completed_by: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub evidence_url: Option<String>,
    pub notes: Option<String>,
}

/// Nigerian compliance categories
pub const COMPLIANCE_CATEGORIES: &[&str] = &[
    "NDPR",      // Nigeria Data Protection Regulation
    "PAYE",      // Tax compliance
    "PenCom",    // Pension compliance
    "NSITF",     // National Social Insurance
    "ITF",       // Industrial Training Fund
    "LabourAct", // Nigerian Labour Act
];
