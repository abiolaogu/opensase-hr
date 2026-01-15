//! Payroll Models
//!
//! Data structures for payroll processing.

use chrono::{DateTime, NaiveDate, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Payroll Run Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayrollRunStatus {
    Draft,
    Processing,
    PendingApproval,
    Approved,
    Paid,
    Cancelled,
}

impl Default for PayrollRunStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// Payroll Run - Represents a payroll period
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRun {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub run_date: Option<DateTime<Utc>>,
    pub status: PayrollRunStatus,
    
    // Totals
    pub total_employees: i32,
    pub total_gross: Decimal,
    pub total_deductions: Decimal,
    pub total_net: Decimal,
    pub total_employer_contributions: Decimal,
    
    // Approval
    pub processed_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PayrollRun {
    pub fn new(tenant_id: Uuid, name: String, period_start: NaiveDate, period_end: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            period_start,
            period_end,
            run_date: None,
            status: PayrollRunStatus::Draft,
            total_employees: 0,
            total_gross: Decimal::ZERO,
            total_deductions: Decimal::ZERO,
            total_net: Decimal::ZERO,
            total_employer_contributions: Decimal::ZERO,
            processed_by: None,
            processed_at: None,
            approved_by: None,
            approved_at: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_draft(&self) -> bool {
        self.status == PayrollRunStatus::Draft
    }

    pub fn can_be_processed(&self) -> bool {
        self.status == PayrollRunStatus::Draft
    }

    pub fn can_be_approved(&self) -> bool {
        self.status == PayrollRunStatus::PendingApproval
    }
}

/// Payroll Item - Individual employee payslip
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollItem {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub employee_id: Uuid,
    
    // Earnings
    pub basic_salary: Decimal,
    pub housing_allowance: Decimal,
    pub transport_allowance: Decimal,
    pub meal_allowance: Decimal,
    pub utility_allowance: Decimal,
    pub other_allowances: serde_json::Value,
    pub gross_pay: Decimal,
    
    // Nigerian Statutory Deductions
    pub paye_tax: Decimal,
    pub pension_employee: Decimal,
    pub pension_employer: Decimal,
    pub nhf_deduction: Decimal,
    
    // Other Deductions
    pub loan_repayment: Decimal,
    pub other_deductions: serde_json::Value,
    pub total_deductions: Decimal,
    
    // Net Pay
    pub net_pay: Decimal,
    
    // Bank Details (snapshot at time of payroll)
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    
    pub created_at: DateTime<Utc>,
}

impl PayrollItem {
    pub fn calculate_gross(&self) -> Decimal {
        self.basic_salary 
            + self.housing_allowance 
            + self.transport_allowance 
            + self.meal_allowance 
            + self.utility_allowance
    }

    pub fn calculate_total_deductions(&self) -> Decimal {
        self.paye_tax 
            + self.pension_employee 
            + self.nhf_deduction 
            + self.loan_repayment
    }
}

/// Employee salary details for payroll calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSalary {
    pub employee_id: Uuid,
    pub employee_name: String,
    pub employee_code: String,
    
    // Salary components
    pub basic_salary: Decimal,
    pub housing_allowance: Decimal,
    pub transport_allowance: Decimal,
    pub meal_allowance: Decimal,
    pub utility_allowance: Decimal,
    pub other_allowances: serde_json::Value,
    
    // Bank details
    pub bank_name: Option<String>,
    pub account_number: Option<String>,
    pub account_name: Option<String>,
    
    // Tax info
    pub tin: Option<String>,
    pub pension_pin: Option<String>,
    pub nhf_number: Option<String>,
    
    // Deductions
    pub loan_balance: Decimal,
    pub loan_monthly_repayment: Decimal,
}

/// Request to create a payroll run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePayrollRunRequest {
    pub name: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub notes: Option<String>,
}

/// Request to process payroll
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessPayrollRequest {
    /// Optional list of employee IDs to include (if empty, all active employees)
    pub employee_ids: Option<Vec<Uuid>>,
    /// Whether to recalculate if already processed
    pub force_recalculate: bool,
}

/// Payroll summary response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollSummary {
    pub payroll_run: PayrollRun,
    pub items: Vec<PayrollItem>,
    
    // Aggregated stats
    pub by_department: Vec<DepartmentPayrollSummary>,
    
    // Statutory totals
    pub total_paye: Decimal,
    pub total_pension_employee: Decimal,
    pub total_pension_employer: Decimal,
    pub total_nhf: Decimal,
}

/// Department-level payroll summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentPayrollSummary {
    pub department_id: Uuid,
    pub department_name: String,
    pub employee_count: i32,
    pub total_gross: Decimal,
    pub total_net: Decimal,
}

/// P9A Tax Return (Annual)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P9AReturn {
    pub year: i32,
    pub employee_id: Uuid,
    pub employee_name: String,
    pub tin: Option<String>,
    
    // Monthly breakdown
    pub monthly_earnings: Vec<MonthlyEarning>,
    
    // Annual totals
    pub annual_gross: Decimal,
    pub annual_tax_deducted: Decimal,
    pub annual_pension: Decimal,
}

/// Monthly earning for P9A
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyEarning {
    pub month: u32,  // 1-12
    pub gross: Decimal,
    pub tax_deducted: Decimal,
}

/// Pension schedule for PFA remittance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PensionSchedule {
    pub period: String,  // e.g., "January 2024"
    pub pfa_name: String,
    pub entries: Vec<PensionScheduleEntry>,
    pub total_employee: Decimal,
    pub total_employer: Decimal,
    pub grand_total: Decimal,
}

/// Individual entry in pension schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PensionScheduleEntry {
    pub employee_name: String,
    pub pension_pin: Option<String>,
    pub rsa_number: Option<String>,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total: Decimal,
}
