//! Role-Based Access Control

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// User roles in the HR system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    SuperAdmin,    // Platform admin (multi-tenant)
    TenantAdmin,   // Company admin
    HrManager,     // Full HR access
    HrStaff,       // Limited HR access
    DepartmentHead,// Department-level access
    TeamLead,      // Team-level access
    Employee,      // Self-service only
}

impl Role {
    /// Get all permissions for this role
    pub fn permissions(&self) -> HashSet<Permission> {
        match self {
            Role::SuperAdmin => Permission::all(),
            Role::TenantAdmin => {
                let mut perms = Permission::all();
                perms.remove(&Permission::SystemAdmin);
                perms
            }
            Role::HrManager => {
                let mut perms = HashSet::new();
                perms.extend([
                    Permission::EmployeeView, Permission::EmployeeCreate,
                    Permission::EmployeeUpdate, Permission::EmployeeDelete,
                    Permission::PayrollView, Permission::PayrollProcess,
                    Permission::PayrollApprove,
                    Permission::LeaveRequest, Permission::LeaveApprove,
                    Permission::LeaveAdmin,
                    Permission::PerformanceView, Permission::PerformanceReview,
                    Permission::PerformanceAdmin,
                    Permission::RecruitmentView, Permission::RecruitmentManage,
                    Permission::BenefitsEnroll, Permission::BenefitsAdmin,
                    Permission::ComplianceView, Permission::ComplianceAdmin,
                    Permission::ReportsView, Permission::ReportsExport,
                ]);
                perms
            }
            Role::HrStaff => {
                let mut perms = HashSet::new();
                perms.extend([
                    Permission::EmployeeView, Permission::EmployeeCreate,
                    Permission::EmployeeUpdate,
                    Permission::PayrollView,
                    Permission::LeaveRequest, Permission::LeaveApprove,
                    Permission::PerformanceView,
                    Permission::RecruitmentView,
                    Permission::BenefitsEnroll,
                    Permission::ReportsView,
                ]);
                perms
            }
            Role::DepartmentHead => {
                let mut perms = HashSet::new();
                perms.extend([
                    Permission::EmployeeView,
                    Permission::PayrollView,
                    Permission::LeaveRequest, Permission::LeaveApprove,
                    Permission::PerformanceView, Permission::PerformanceReview,
                    Permission::ReportsView,
                ]);
                perms
            }
            Role::TeamLead => {
                let mut perms = HashSet::new();
                perms.extend([
                    Permission::EmployeeView,
                    Permission::LeaveRequest, Permission::LeaveApprove,
                    Permission::PerformanceView, Permission::PerformanceReview,
                ]);
                perms
            }
            Role::Employee => {
                let mut perms = HashSet::new();
                perms.extend([
                    Permission::EmployeeView,  // Own profile only
                    Permission::PayrollView,   // Own payslips only
                    Permission::LeaveRequest,
                    Permission::PerformanceView,
                    Permission::BenefitsEnroll,
                ]);
                perms
            }
        }
    }
}

/// Granular permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Permission {
    // Employees
    EmployeeView,
    EmployeeCreate,
    EmployeeUpdate,
    EmployeeDelete,
    
    // Payroll
    PayrollView,
    PayrollProcess,
    PayrollApprove,
    
    // Leave
    LeaveRequest,
    LeaveApprove,
    LeaveAdmin,
    
    // Performance
    PerformanceView,
    PerformanceReview,
    PerformanceAdmin,
    
    // Recruitment
    RecruitmentView,
    RecruitmentManage,
    
    // Benefits
    BenefitsEnroll,
    BenefitsAdmin,
    
    // Compliance
    ComplianceView,
    ComplianceAdmin,
    
    // System
    SystemAdmin,
    ReportsView,
    ReportsExport,
}

impl Permission {
    /// Get all permissions
    pub fn all() -> HashSet<Permission> {
        let mut perms = HashSet::new();
        perms.extend([
            Self::EmployeeView, Self::EmployeeCreate,
            Self::EmployeeUpdate, Self::EmployeeDelete,
            Self::PayrollView, Self::PayrollProcess, Self::PayrollApprove,
            Self::LeaveRequest, Self::LeaveApprove, Self::LeaveAdmin,
            Self::PerformanceView, Self::PerformanceReview, Self::PerformanceAdmin,
            Self::RecruitmentView, Self::RecruitmentManage,
            Self::BenefitsEnroll, Self::BenefitsAdmin,
            Self::ComplianceView, Self::ComplianceAdmin,
            Self::SystemAdmin, Self::ReportsView, Self::ReportsExport,
        ]);
        perms
    }
}

/// Check if a role has a specific permission
pub fn has_permission(role: Role, permission: Permission) -> bool {
    role.permissions().contains(&permission)
}

/// Authorization context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub user_id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub employee_id: Option<uuid::Uuid>,
    pub role: Role,
    pub permissions: HashSet<Permission>,
    pub department_id: Option<uuid::Uuid>,
}

impl AuthContext {
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.contains(&permission)
    }

    pub fn can_access_employee(&self, employee_id: uuid::Uuid) -> bool {
        // SuperAdmin and TenantAdmin can access all
        if matches!(self.role, Role::SuperAdmin | Role::TenantAdmin | Role::HrManager | Role::HrStaff) {
            return true;
        }
        // Others can only access themselves
        self.employee_id == Some(employee_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_permissions() {
        let hr_manager = Role::HrManager;
        assert!(has_permission(hr_manager, Permission::PayrollProcess));
        assert!(has_permission(hr_manager, Permission::LeaveApprove));
        assert!(!has_permission(hr_manager, Permission::SystemAdmin));

        let employee = Role::Employee;
        assert!(has_permission(employee, Permission::LeaveRequest));
        assert!(!has_permission(employee, Permission::PayrollProcess));
    }

    #[test]
    fn test_auth_context() {
        let ctx = AuthContext {
            user_id: uuid::Uuid::new_v4(),
            tenant_id: uuid::Uuid::new_v4(),
            employee_id: Some(uuid::Uuid::new_v4()),
            role: Role::Employee,
            permissions: Role::Employee.permissions(),
            department_id: None,
        };

        assert!(ctx.has_permission(Permission::LeaveRequest));
        assert!(!ctx.has_permission(Permission::PayrollApprove));
        assert!(ctx.can_access_employee(ctx.employee_id.unwrap()));
    }
}
