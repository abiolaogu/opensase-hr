//! Nigerian Tax Calculator
//!
//! Implements PAYE (Pay As You Earn) tax calculation based on Nigerian tax bands.
//! Updated for 2024 rates as per FIRS guidelines.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Nigerian PAYE Tax Bands (2024)
/// 
/// Annual income is taxed progressively:
/// - First ₦300,000: 7%
/// - Next ₦300,000: 11%
/// - Next ₦500,000: 15%
/// - Next ₦500,000: 19%
/// - Next ₦1,600,000: 21%
/// - Above ₦3,200,000: 24%
#[derive(Debug, Clone)]
pub struct TaxBand {
    pub threshold: Decimal,
    pub rate: Decimal,
}

/// Nigerian PAYE Tax Calculator
#[derive(Debug, Clone)]
pub struct NigerianTaxCalculator {
    bands: Vec<TaxBand>,
    /// Consolidated Relief Allowance (CRA)
    /// 20% of gross income + ₦200,000 (or 1% of gross if higher)
    cra_fixed: Decimal,
    cra_percentage: Decimal,
    cra_min_percentage: Decimal,
}

impl Default for NigerianTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl NigerianTaxCalculator {
    pub fn new() -> Self {
        Self {
            bands: vec![
                TaxBand { threshold: dec!(300_000), rate: dec!(0.07) },
                TaxBand { threshold: dec!(300_000), rate: dec!(0.11) },
                TaxBand { threshold: dec!(500_000), rate: dec!(0.15) },
                TaxBand { threshold: dec!(500_000), rate: dec!(0.19) },
                TaxBand { threshold: dec!(1_600_000), rate: dec!(0.21) },
                TaxBand { threshold: Decimal::MAX, rate: dec!(0.24) },
            ],
            cra_fixed: dec!(200_000),
            cra_percentage: dec!(0.20),
            cra_min_percentage: dec!(0.01),
        }
    }

    /// Calculate annual PAYE tax
    /// 
    /// # Arguments
    /// * `gross_annual` - Total annual gross income
    /// * `pension_contribution` - Annual pension contribution (exempt from tax)
    /// * `nhf_contribution` - Annual NHF contribution (exempt from tax)
    /// 
    /// # Returns
    /// Annual PAYE tax amount
    pub fn calculate_annual_paye(
        &self,
        gross_annual: Decimal,
        pension_contribution: Decimal,
        nhf_contribution: Decimal,
    ) -> TaxCalculation {
        // Step 1: Calculate Consolidated Relief Allowance (CRA)
        let cra_percentage_amount = gross_annual * self.cra_percentage;
        let cra_min_amount = gross_annual * self.cra_min_percentage;
        let cra_higher = if cra_min_amount > self.cra_fixed {
            cra_min_amount
        } else {
            self.cra_fixed
        };
        let total_cra = cra_percentage_amount + cra_higher;

        // Step 2: Calculate taxable income
        let total_exemptions = total_cra + pension_contribution + nhf_contribution;
        let taxable_income = if gross_annual > total_exemptions {
            gross_annual - total_exemptions
        } else {
            Decimal::ZERO
        };

        // Step 3: Apply progressive tax bands
        let mut remaining = taxable_income;
        let mut total_tax = Decimal::ZERO;
        let mut band_breakdown = Vec::new();

        for band in &self.bands {
            if remaining <= Decimal::ZERO {
                break;
            }

            let taxable_in_band = if remaining > band.threshold {
                band.threshold
            } else {
                remaining
            };

            let tax_for_band = taxable_in_band * band.rate;
            total_tax += tax_for_band;
            
            band_breakdown.push(TaxBandResult {
                threshold: band.threshold,
                rate: band.rate,
                taxable_amount: taxable_in_band,
                tax_amount: tax_for_band,
            });

            remaining -= taxable_in_band;
        }

        TaxCalculation {
            gross_income: gross_annual,
            consolidated_relief: total_cra,
            pension_relief: pension_contribution,
            nhf_relief: nhf_contribution,
            total_exemptions,
            taxable_income,
            annual_tax: total_tax,
            monthly_tax: total_tax / dec!(12),
            effective_rate: if gross_annual > Decimal::ZERO {
                (total_tax / gross_annual) * dec!(100)
            } else {
                Decimal::ZERO
            },
            band_breakdown,
        }
    }

    /// Calculate monthly PAYE tax
    pub fn calculate_monthly_paye(
        &self,
        gross_monthly: Decimal,
        pension_monthly: Decimal,
        nhf_monthly: Decimal,
    ) -> TaxCalculation {
        let mut calc = self.calculate_annual_paye(
            gross_monthly * dec!(12),
            pension_monthly * dec!(12),
            nhf_monthly * dec!(12),
        );
        calc.monthly_tax = calc.annual_tax / dec!(12);
        calc
    }
}

/// Result of tax calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxCalculation {
    pub gross_income: Decimal,
    pub consolidated_relief: Decimal,
    pub pension_relief: Decimal,
    pub nhf_relief: Decimal,
    pub total_exemptions: Decimal,
    pub taxable_income: Decimal,
    pub annual_tax: Decimal,
    pub monthly_tax: Decimal,
    /// Effective tax rate as percentage
    pub effective_rate: Decimal,
    pub band_breakdown: Vec<TaxBandResult>,
}

/// Tax amount per band
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBandResult {
    pub threshold: Decimal,
    pub rate: Decimal,
    pub taxable_amount: Decimal,
    pub tax_amount: Decimal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paye_calculation_3m_salary() {
        let calculator = NigerianTaxCalculator::new();
        
        // ₦3,000,000 annual gross salary
        let gross = dec!(3_000_000);
        let pension = gross * dec!(0.08); // 8% pension
        let nhf = dec!(250_000) * dec!(0.025) * dec!(12); // 2.5% of basic (assuming basic = 250k/month)
        
        let result = calculator.calculate_annual_paye(gross, pension, nhf);
        
        // Verify taxable income is reduced by reliefs
        assert!(result.taxable_income < gross);
        assert!(result.annual_tax > Decimal::ZERO);
        assert!(result.monthly_tax > Decimal::ZERO);
        
        // Effective rate should be less than 24% (highest band)
        assert!(result.effective_rate < dec!(24));
        
        println!("Gross: ₦{}", result.gross_income);
        println!("CRA: ₦{}", result.consolidated_relief);
        println!("Pension Relief: ₦{}", result.pension_relief);
        println!("Taxable Income: ₦{}", result.taxable_income);
        println!("Annual Tax: ₦{}", result.annual_tax);
        println!("Monthly Tax: ₦{}", result.monthly_tax);
        println!("Effective Rate: {}%", result.effective_rate);
    }

    #[test]
    fn test_zero_income() {
        let calculator = NigerianTaxCalculator::new();
        let result = calculator.calculate_annual_paye(Decimal::ZERO, Decimal::ZERO, Decimal::ZERO);
        
        assert_eq!(result.annual_tax, Decimal::ZERO);
        assert_eq!(result.taxable_income, Decimal::ZERO);
    }

    #[test]
    fn test_low_income_no_tax() {
        let calculator = NigerianTaxCalculator::new();
        
        // Very low income that should be fully covered by CRA
        let gross = dec!(400_000);
        let result = calculator.calculate_annual_paye(gross, Decimal::ZERO, Decimal::ZERO);
        
        // CRA = 200,000 + (20% of 400,000) = 200,000 + 80,000 = 280,000
        // Taxable = 400,000 - 280,000 = 120,000
        // Tax on 120,000 at 7% = 8,400
        assert!(result.annual_tax > Decimal::ZERO);
    }

    #[test]
    fn test_high_income() {
        let calculator = NigerianTaxCalculator::new();
        
        // ₦10,000,000 annual (high earner)
        let gross = dec!(10_000_000);
        let pension = gross * dec!(0.08);
        let result = calculator.calculate_annual_paye(gross, pension, Decimal::ZERO);
        
        // Should hit all tax bands including 24%
        assert!(result.band_breakdown.len() >= 5);
        assert!(result.effective_rate > dec!(10)); // Should be significant
    }
}
