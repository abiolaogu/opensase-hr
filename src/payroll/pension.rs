//! Pension Calculator
//!
//! Nigerian PenCom pension calculation (Contributory Pension Scheme).
//! Also includes NHF (National Housing Fund) calculations.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Nigerian Pension Calculator (PenCom Rates)
/// 
/// Contributory Pension Scheme rates:
/// - Employee contribution: 8% of (Basic + Housing + Transport)
/// - Employer contribution: 10% of (Basic + Housing + Transport)
/// - NHF: 2.5% of Basic Salary
#[derive(Debug, Clone)]
pub struct PensionCalculator {
    employee_rate: Decimal,
    employer_rate: Decimal,
    nhf_rate: Decimal,
}

impl Default for PensionCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl PensionCalculator {
    pub fn new() -> Self {
        Self {
            employee_rate: dec!(0.08),  // 8%
            employer_rate: dec!(0.10),  // 10%
            nhf_rate: dec!(0.025),      // 2.5%
        }
    }

    /// Create calculator with custom rates
    pub fn with_rates(employee_rate: Decimal, employer_rate: Decimal, nhf_rate: Decimal) -> Self {
        Self {
            employee_rate,
            employer_rate,
            nhf_rate,
        }
    }

    /// Calculate pension contributions
    /// 
    /// # Arguments
    /// * `basic_salary` - Basic salary amount
    /// * `housing_allowance` - Housing allowance amount
    /// * `transport_allowance` - Transport allowance amount
    /// 
    /// # Returns
    /// Pension calculation result with employee and employer contributions
    pub fn calculate(
        &self,
        basic_salary: Decimal,
        housing_allowance: Decimal,
        transport_allowance: Decimal,
    ) -> PensionCalculation {
        // Pension is calculated on Basic + Housing + Transport
        let pensionable_earnings = basic_salary + housing_allowance + transport_allowance;
        
        let employee_contribution = pensionable_earnings * self.employee_rate;
        let employer_contribution = pensionable_earnings * self.employer_rate;
        let total_contribution = employee_contribution + employer_contribution;
        
        // NHF is only on Basic Salary
        let nhf_contribution = basic_salary * self.nhf_rate;

        PensionCalculation {
            basic_salary,
            housing_allowance,
            transport_allowance,
            pensionable_earnings,
            employee_contribution,
            employer_contribution,
            total_contribution,
            nhf_contribution,
            employee_rate: self.employee_rate,
            employer_rate: self.employer_rate,
        }
    }

    /// Calculate pension for monthly salary
    pub fn calculate_monthly(
        &self,
        monthly_basic: Decimal,
        monthly_housing: Decimal,
        monthly_transport: Decimal,
    ) -> PensionCalculation {
        self.calculate(monthly_basic, monthly_housing, monthly_transport)
    }

    /// Calculate pension for annual salary
    pub fn calculate_annual(
        &self,
        annual_basic: Decimal,
        annual_housing: Decimal,
        annual_transport: Decimal,
    ) -> PensionCalculation {
        self.calculate(annual_basic, annual_housing, annual_transport)
    }
}

/// Result of pension calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PensionCalculation {
    pub basic_salary: Decimal,
    pub housing_allowance: Decimal,
    pub transport_allowance: Decimal,
    pub pensionable_earnings: Decimal,
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total_contribution: Decimal,
    pub nhf_contribution: Decimal,
    pub employee_rate: Decimal,
    pub employer_rate: Decimal,
}

impl PensionCalculation {
    /// Get total deductions from employee salary
    pub fn total_employee_deductions(&self) -> Decimal {
        self.employee_contribution + self.nhf_contribution
    }

    /// Get total employer cost
    pub fn total_employer_cost(&self) -> Decimal {
        self.employer_contribution
    }
}

/// NSITF (National Social Insurance Trust Fund) Calculator
/// 
/// Employer pays 1% of total monthly payroll to NSITF
#[derive(Debug, Clone)]
pub struct NsitfCalculator {
    rate: Decimal,
}

impl Default for NsitfCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl NsitfCalculator {
    pub fn new() -> Self {
        Self { rate: dec!(0.01) } // 1%
    }

    /// Calculate NSITF contribution (employer only)
    pub fn calculate(&self, total_payroll: Decimal) -> Decimal {
        total_payroll * self.rate
    }
}

/// ITF (Industrial Training Fund) Calculator
/// 
/// Employers with 5+ employees or turnover > ₦50M pay 1% of annual payroll
#[derive(Debug, Clone)]
pub struct ItfCalculator {
    rate: Decimal,
}

impl Default for ItfCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl ItfCalculator {
    pub fn new() -> Self {
        Self { rate: dec!(0.01) } // 1%
    }

    /// Calculate ITF contribution (employer only)
    pub fn calculate(&self, total_payroll: Decimal) -> Decimal {
        total_payroll * self.rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pension_calculation() {
        let calculator = PensionCalculator::new();
        
        let result = calculator.calculate(
            dec!(250_000),  // Basic
            dec!(100_000),  // Housing
            dec!(50_000),   // Transport
        );
        
        // Pensionable = 250k + 100k + 50k = 400k
        assert_eq!(result.pensionable_earnings, dec!(400_000));
        
        // Employee = 8% of 400k = 32k
        assert_eq!(result.employee_contribution, dec!(32_000));
        
        // Employer = 10% of 400k = 40k
        assert_eq!(result.employer_contribution, dec!(40_000));
        
        // NHF = 2.5% of Basic (250k) = 6,250
        assert_eq!(result.nhf_contribution, dec!(6_250));
        
        // Total employee deductions = 32k + 6,250 = 38,250
        assert_eq!(result.total_employee_deductions(), dec!(38_250));
    }

    #[test]
    fn test_nsitf_calculation() {
        let calculator = NsitfCalculator::new();
        
        // Total monthly payroll of ₦10,000,000
        let result = calculator.calculate(dec!(10_000_000));
        
        // 1% = 100,000
        assert_eq!(result, dec!(100_000));
    }

    #[test]
    fn test_itf_calculation() {
        let calculator = ItfCalculator::new();
        
        // Annual payroll of ₦120,000,000
        let result = calculator.calculate(dec!(120_000_000));
        
        // 1% = 1,200,000
        assert_eq!(result, dec!(1_200_000));
    }

    #[test]
    fn test_zero_salary() {
        let calculator = PensionCalculator::new();
        let result = calculator.calculate(Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
        
        assert_eq!(result.employee_contribution, Decimal::ZERO);
        assert_eq!(result.employer_contribution, Decimal::ZERO);
        assert_eq!(result.nhf_contribution, Decimal::ZERO);
    }
}
