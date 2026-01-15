//! Payroll Service
//!
//! Business logic for payroll processing with Nigerian compliance.

use std::collections::HashMap;
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use uuid::Uuid;

use super::{
    models::*,
    tax_calculator::NigerianTaxCalculator,
    pension::PensionCalculator,
};

/// Payroll processing errors
#[derive(Debug, thiserror::Error)]
pub enum PayrollError {
    #[error("Payroll run not found: {0}")]
    NotFound(Uuid),
    
    #[error("Payroll run is not in draft status")]
    NotDraft,
    
    #[error("Payroll run cannot be approved in current status")]
    CannotApprove,
    
    #[error("No employees found for payroll processing")]
    NoEmployees,
    
    #[error("Employee {0} has no salary configuration")]
    NoSalaryConfig(Uuid),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

/// Payroll Service
#[derive(Debug, Clone)]
pub struct PayrollService {
    tax_calculator: NigerianTaxCalculator,
    pension_calculator: PensionCalculator,
}

impl Default for PayrollService {
    fn default() -> Self {
        Self::new()
    }
}

impl PayrollService {
    pub fn new() -> Self {
        Self {
            tax_calculator: NigerianTaxCalculator::new(),
            pension_calculator: PensionCalculator::new(),
        }
    }

    /// Create a new payroll run
    pub fn create_payroll_run(
        &self,
        tenant_id: Uuid,
        request: CreatePayrollRunRequest,
    ) -> Result<PayrollRun, PayrollError> {
        // Validate dates
        if request.period_end < request.period_start {
            return Err(PayrollError::Validation(
                "Period end must be after period start".to_string()
            ));
        }

        let mut run = PayrollRun::new(
            tenant_id,
            request.name,
            request.period_start,
            request.period_end,
        );
        run.notes = request.notes;

        Ok(run)
    }

    /// Process payroll for all employees
    /// 
    /// This calculates gross pay, all deductions, and net pay for each employee.
    pub fn process_payroll(
        &self,
        payroll_run: &mut PayrollRun,
        employees: Vec<EmployeeSalary>,
        processor_id: Uuid,
    ) -> Result<Vec<PayrollItem>, PayrollError> {
        if !payroll_run.can_be_processed() {
            return Err(PayrollError::NotDraft);
        }

        if employees.is_empty() {
            return Err(PayrollError::NoEmployees);
        }

        let mut items = Vec::with_capacity(employees.len());
        let mut total_gross = Decimal::ZERO;
        let mut total_deductions = Decimal::ZERO;
        let mut total_net = Decimal::ZERO;
        let mut total_employer_contributions = Decimal::ZERO;

        for employee in employees {
            let item = self.calculate_payslip(payroll_run.id, &employee)?;
            
            total_gross += item.gross_pay;
            total_deductions += item.total_deductions;
            total_net += item.net_pay;
            total_employer_contributions += item.pension_employer;
            
            items.push(item);
        }

        // Update payroll run totals
        payroll_run.total_employees = items.len() as i32;
        payroll_run.total_gross = total_gross;
        payroll_run.total_deductions = total_deductions;
        payroll_run.total_net = total_net;
        payroll_run.total_employer_contributions = total_employer_contributions;
        payroll_run.status = PayrollRunStatus::PendingApproval;
        payroll_run.processed_by = Some(processor_id);
        payroll_run.processed_at = Some(Utc::now());
        payroll_run.run_date = Some(Utc::now());
        payroll_run.updated_at = Utc::now();

        Ok(items)
    }

    /// Calculate individual payslip
    fn calculate_payslip(
        &self,
        payroll_run_id: Uuid,
        employee: &EmployeeSalary,
    ) -> Result<PayrollItem, PayrollError> {
        // Calculate gross pay
        let gross_pay = employee.basic_salary
            + employee.housing_allowance
            + employee.transport_allowance
            + employee.meal_allowance
            + employee.utility_allowance;

        // Calculate pension (based on Basic + Housing + Transport)
        let pension_calc = self.pension_calculator.calculate(
            employee.basic_salary,
            employee.housing_allowance,
            employee.transport_allowance,
        );

        // Calculate PAYE tax (monthly)
        let tax_calc = self.tax_calculator.calculate_monthly_paye(
            gross_pay,
            pension_calc.employee_contribution,
            pension_calc.nhf_contribution,
        );

        // Calculate total deductions
        let total_deductions = tax_calc.monthly_tax
            + pension_calc.employee_contribution
            + pension_calc.nhf_contribution
            + employee.loan_monthly_repayment;

        // Calculate net pay
        let net_pay = gross_pay - total_deductions;

        Ok(PayrollItem {
            id: Uuid::new_v4(),
            payroll_run_id,
            employee_id: employee.employee_id,
            
            basic_salary: employee.basic_salary,
            housing_allowance: employee.housing_allowance,
            transport_allowance: employee.transport_allowance,
            meal_allowance: employee.meal_allowance,
            utility_allowance: employee.utility_allowance,
            other_allowances: employee.other_allowances.clone(),
            gross_pay,
            
            paye_tax: tax_calc.monthly_tax,
            pension_employee: pension_calc.employee_contribution,
            pension_employer: pension_calc.employer_contribution,
            nhf_deduction: pension_calc.nhf_contribution,
            
            loan_repayment: employee.loan_monthly_repayment,
            other_deductions: serde_json::json!({}),
            total_deductions,
            
            net_pay,
            
            bank_name: employee.bank_name.clone(),
            account_number: employee.account_number.clone(),
            account_name: employee.account_name.clone(),
            
            created_at: Utc::now(),
        })
    }

    /// Approve payroll run
    pub fn approve_payroll(
        &self,
        payroll_run: &mut PayrollRun,
        approver_id: Uuid,
    ) -> Result<(), PayrollError> {
        if !payroll_run.can_be_approved() {
            return Err(PayrollError::CannotApprove);
        }

        payroll_run.status = PayrollRunStatus::Approved;
        payroll_run.approved_by = Some(approver_id);
        payroll_run.approved_at = Some(Utc::now());
        payroll_run.updated_at = Utc::now();

        Ok(())
    }

    /// Mark payroll as paid
    pub fn mark_as_paid(
        &self,
        payroll_run: &mut PayrollRun,
    ) -> Result<(), PayrollError> {
        if payroll_run.status != PayrollRunStatus::Approved {
            return Err(PayrollError::Validation(
                "Payroll must be approved before marking as paid".to_string()
            ));
        }

        payroll_run.status = PayrollRunStatus::Paid;
        payroll_run.updated_at = Utc::now();

        Ok(())
    }

    /// Generate pension schedule for PFA remittance
    pub fn generate_pension_schedule(
        &self,
        items: &[PayrollItem],
        employee_details: &HashMap<Uuid, EmployeeSalary>,
        period: &str,
    ) -> Vec<PensionSchedule> {
        // Group by PFA
        let mut by_pfa: HashMap<String, Vec<PensionScheduleEntry>> = HashMap::new();

        for item in items {
            if let Some(employee) = employee_details.get(&item.employee_id) {
                let pfa = "Default PFA".to_string(); // Would come from employee record
                
                let entry = PensionScheduleEntry {
                    employee_name: employee.employee_name.clone(),
                    pension_pin: employee.pension_pin.clone(),
                    rsa_number: None,
                    employee_contribution: item.pension_employee,
                    employer_contribution: item.pension_employer,
                    total: item.pension_employee + item.pension_employer,
                };

                by_pfa.entry(pfa).or_default().push(entry);
            }
        }

        // Create schedules
        by_pfa.into_iter().map(|(pfa_name, entries)| {
            let total_employee: Decimal = entries.iter().map(|e| e.employee_contribution).sum();
            let total_employer: Decimal = entries.iter().map(|e| e.employer_contribution).sum();

            PensionSchedule {
                period: period.to_string(),
                pfa_name,
                entries,
                total_employee,
                total_employer,
                grand_total: total_employee + total_employer,
            }
        }).collect()
    }

    /// Calculate tax preview without creating payroll
    pub fn calculate_tax_preview(
        &self,
        monthly_gross: Decimal,
    ) -> TaxPreviewResponse {
        let pension_calc = self.pension_calculator.calculate(
            monthly_gross * dec!(0.60), // Assume 60% is basic
            monthly_gross * dec!(0.25), // 25% housing
            monthly_gross * dec!(0.15), // 15% transport
        );

        let tax_calc = self.tax_calculator.calculate_monthly_paye(
            monthly_gross,
            pension_calc.employee_contribution,
            pension_calc.nhf_contribution,
        );

        let total_deductions = tax_calc.monthly_tax
            + pension_calc.employee_contribution
            + pension_calc.nhf_contribution;

        TaxPreviewResponse {
            gross_monthly: monthly_gross,
            gross_annual: monthly_gross * dec!(12),
            paye_monthly: tax_calc.monthly_tax,
            paye_annual: tax_calc.annual_tax,
            pension_employee: pension_calc.employee_contribution,
            pension_employer: pension_calc.employer_contribution,
            nhf: pension_calc.nhf_contribution,
            total_deductions,
            net_monthly: monthly_gross - total_deductions,
            effective_tax_rate: tax_calc.effective_rate,
        }
    }
}

/// Tax preview response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxPreviewResponse {
    pub gross_monthly: Decimal,
    pub gross_annual: Decimal,
    pub paye_monthly: Decimal,
    pub paye_annual: Decimal,
    pub pension_employee: Decimal,
    pub pension_employer: Decimal,
    pub nhf: Decimal,
    pub total_deductions: Decimal,
    pub net_monthly: Decimal,
    pub effective_tax_rate: Decimal,
}

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_test_employee() -> EmployeeSalary {
        EmployeeSalary {
            employee_id: Uuid::new_v4(),
            employee_name: "Test Employee".to_string(),
            employee_code: "EMP001".to_string(),
            basic_salary: dec!(250_000),
            housing_allowance: dec!(100_000),
            transport_allowance: dec!(50_000),
            meal_allowance: dec!(20_000),
            utility_allowance: dec!(10_000),
            other_allowances: serde_json::json!({}),
            bank_name: Some("GTBank".to_string()),
            account_number: Some("0123456789".to_string()),
            account_name: Some("Test Employee".to_string()),
            tin: Some("12345678-0001".to_string()),
            pension_pin: Some("PEN123456".to_string()),
            nhf_number: Some("NHF123456".to_string()),
            loan_balance: Decimal::ZERO,
            loan_monthly_repayment: Decimal::ZERO,
        }
    }

    #[test]
    fn test_create_payroll_run() {
        let service = PayrollService::new();
        let tenant_id = Uuid::new_v4();
        
        let request = CreatePayrollRunRequest {
            name: "January 2024 Payroll".to_string(),
            period_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            notes: None,
        };

        let run = service.create_payroll_run(tenant_id, request).unwrap();
        
        assert_eq!(run.status, PayrollRunStatus::Draft);
        assert_eq!(run.total_employees, 0);
    }

    #[test]
    fn test_process_payroll() {
        let service = PayrollService::new();
        let tenant_id = Uuid::new_v4();
        
        let request = CreatePayrollRunRequest {
            name: "January 2024 Payroll".to_string(),
            period_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            notes: None,
        };

        let mut run = service.create_payroll_run(tenant_id, request).unwrap();
        let employees = vec![create_test_employee()];
        let processor_id = Uuid::new_v4();

        let items = service.process_payroll(&mut run, employees, processor_id).unwrap();

        assert_eq!(items.len(), 1);
        assert_eq!(run.total_employees, 1);
        assert!(run.total_gross > Decimal::ZERO);
        assert!(run.total_net > Decimal::ZERO);
        assert!(run.total_net < run.total_gross);
        assert_eq!(run.status, PayrollRunStatus::PendingApproval);
        
        // Verify deductions
        let item = &items[0];
        assert!(item.paye_tax > Decimal::ZERO);
        assert!(item.pension_employee > Decimal::ZERO);
        assert!(item.nhf_deduction > Decimal::ZERO);
        
        println!("Gross: ₦{}", item.gross_pay);
        println!("PAYE: ₦{}", item.paye_tax);
        println!("Pension (Employee): ₦{}", item.pension_employee);
        println!("Pension (Employer): ₦{}", item.pension_employer);
        println!("NHF: ₦{}", item.nhf_deduction);
        println!("Total Deductions: ₦{}", item.total_deductions);
        println!("Net Pay: ₦{}", item.net_pay);
    }

    #[test]
    fn test_approve_payroll() {
        let service = PayrollService::new();
        let tenant_id = Uuid::new_v4();
        
        let request = CreatePayrollRunRequest {
            name: "January 2024 Payroll".to_string(),
            period_start: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            period_end: NaiveDate::from_ymd_opt(2024, 1, 31).unwrap(),
            notes: None,
        };

        let mut run = service.create_payroll_run(tenant_id, request).unwrap();
        let employees = vec![create_test_employee()];
        let processor_id = Uuid::new_v4();

        service.process_payroll(&mut run, employees, processor_id).unwrap();
        
        let approver_id = Uuid::new_v4();
        service.approve_payroll(&mut run, approver_id).unwrap();
        
        assert_eq!(run.status, PayrollRunStatus::Approved);
        assert!(run.approved_by.is_some());
    }

    #[test]
    fn test_tax_preview() {
        let service = PayrollService::new();
        
        let preview = service.calculate_tax_preview(dec!(500_000));
        
        assert!(preview.paye_monthly > Decimal::ZERO);
        assert!(preview.net_monthly < preview.gross_monthly);
        assert_eq!(preview.gross_annual, dec!(6_000_000));
        
        println!("Monthly Gross: ₦{}", preview.gross_monthly);
        println!("PAYE: ₦{}", preview.paye_monthly);
        println!("Pension: ₦{}", preview.pension_employee);
        println!("NHF: ₦{}", preview.nhf);
        println!("Net: ₦{}", preview.net_monthly);
        println!("Effective Rate: {}%", preview.effective_tax_rate);
    }
}
