//! Middle East Tax Engines
//! 
//! Tax calculators for 15+ Middle East countries featuring:
//! - GCC: Zero income tax, GPSSA/GOSI (nationals), End of Service Benefits
//! - Levant: Israel (complex Bituach Leumi), Jordan, Lebanon
//! - WPS (Wage Protection System) compliance

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// UAE TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// UAE Configuration (no income tax)
#[derive(Debug, Clone)]
pub struct UAEConfig {
    pub tax_year: i32,
    pub gpssa_employee_rate: Decimal,  // 5% (nationals only)
    pub gpssa_employer_rate: Decimal,  // 12.5% (nationals only)
    pub gpssa_max_salary: Decimal,     // AED 50,000
    pub gratuity_first_5_years: u8,    // 21 days/year
    pub gratuity_after_5_years: u8,    // 30 days/year
}

impl Default for UAEConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            gpssa_employee_rate: dec!(0.05),
            gpssa_employer_rate: dec!(0.125),
            gpssa_max_salary: dec!(50_000),
            gratuity_first_5_years: 21,
            gratuity_after_5_years: 30,
        }
    }
}

/// UAE tax calculator
pub struct UAETaxCalculator {
    config: UAEConfig,
}

impl UAETaxCalculator {
    pub fn new() -> Self {
        Self { config: UAEConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, is_national: bool, years: u8) -> TaxResult {
        // GPSSA for UAE/GCC nationals only
        let (gpssa_employee, gpssa_employer) = if is_national {
            let base = gross_monthly.min(self.config.gpssa_max_salary);
            (
                base * self.config.gpssa_employee_rate,
                base * self.config.gpssa_employer_rate,
            )
        } else {
            (Decimal::ZERO, Decimal::ZERO)
        };
        
        // End of Service Gratuity provision
        let gratuity_days = if years <= 5 {
            Decimal::from(self.config.gratuity_first_5_years)
        } else {
            Decimal::from(self.config.gratuity_after_5_years)
        };
        let daily_wage = gross_monthly / dec!(30);
        let gratuity_provision = daily_wage * gratuity_days / dec!(12);
        
        TaxResult {
            country_code: "AE".to_string(),
            currency: "AED".to_string(),
            gross_monthly,
            income_tax: Decimal::ZERO, // No income tax
            social_security_employee: gpssa_employee,
            social_security_employer: gpssa_employer,
            pension_employee: Decimal::ZERO,
            pension_employer: Decimal::ZERO,
            other_employee: Decimal::ZERO,
            other_employer: gratuity_provision,
            total_employee_deductions: gpssa_employee,
            total_employer_contributions: gpssa_employer + gratuity_provision,
            net_monthly: gross_monthly - gpssa_employee,
            effective_rate: if gross_monthly > Decimal::ZERO && is_national { 
                gpssa_employee / gross_monthly * dec!(100) 
            } else { 
                Decimal::ZERO 
            },
            legal_references: vec![
                "Federal Decree-Law No. 33 of 2021 (Labour Law)".to_string(),
                "Federal Law No. 7 of 1999 (GPSSA)".to_string(),
            ],
        }
    }
    
    /// Calculate end of service gratuity for termination
    pub fn calculate_gratuity(&self, basic_salary: Decimal, years: Decimal, is_resignation: bool) -> Decimal {
        let daily_wage = basic_salary / dec!(30);
        let mut total = Decimal::ZERO;
        
        // First 5 years: 21 days/year
        let years_21 = years.min(dec!(5));
        total += daily_wage * dec!(21) * years_21;
        
        // After 5 years: 30 days/year
        if years > dec!(5) {
            total += daily_wage * dec!(30) * (years - dec!(5));
        }
        
        // Resignation adjustment
        if is_resignation && years < dec!(5) {
            total = total * dec!(0.5);
        }
        if is_resignation && years < dec!(1) {
            total = Decimal::ZERO;
        }
        
        total
    }
}

impl Default for UAETaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// SAUDI ARABIA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Saudi Arabia GOSI config
#[derive(Debug, Clone)]
pub struct SaudiConfig {
    pub tax_year: i32,
    pub gosi_annuities_employee: Decimal,  // 9% (nationals)
    pub gosi_annuities_employer: Decimal,  // 9% (nationals)
    pub gosi_oci_employer: Decimal,        // 2% (all)
    pub gosi_saned_employer: Decimal,      // 1.5% (nationals)
    pub gosi_ceiling: Decimal,             // SAR 45,000
    pub eos_first_5_years: u8,             // 15 days/year
    pub eos_after_5_years: u8,             // 30 days/year
}

impl Default for SaudiConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            gosi_annuities_employee: dec!(0.09),
            gosi_annuities_employer: dec!(0.09),
            gosi_oci_employer: dec!(0.02),
            gosi_saned_employer: dec!(0.015),
            gosi_ceiling: dec!(45_000),
            eos_first_5_years: 15,
            eos_after_5_years: 30,
        }
    }
}

/// Saudi Arabia tax calculator
pub struct SaudiTaxCalculator {
    config: SaudiConfig,
}

impl SaudiTaxCalculator {
    pub fn new() -> Self {
        Self { config: SaudiConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, is_saudi: bool) -> TaxResult {
        let gosi_base = gross_monthly.min(self.config.gosi_ceiling);
        
        // GOSI for Saudi nationals vs expats
        let (gosi_employee, gosi_employer) = if is_saudi {
            (
                gosi_base * self.config.gosi_annuities_employee,
                gosi_base * (
                    self.config.gosi_annuities_employer + 
                    self.config.gosi_oci_employer + 
                    self.config.gosi_saned_employer
                ),
            )
        } else {
            // Expats: Only OCI from employer
            (Decimal::ZERO, gosi_base * self.config.gosi_oci_employer)
        };
        
        TaxResult {
            country_code: "SA".to_string(),
            currency: "SAR".to_string(),
            gross_monthly,
            income_tax: Decimal::ZERO, // No income tax
            social_security_employee: gosi_employee,
            social_security_employer: gosi_employer,
            pension_employee: Decimal::ZERO,
            pension_employer: Decimal::ZERO,
            other_employee: Decimal::ZERO,
            other_employer: Decimal::ZERO,
            total_employee_deductions: gosi_employee,
            total_employer_contributions: gosi_employer,
            net_monthly: gross_monthly - gosi_employee,
            effective_rate: if gross_monthly > Decimal::ZERO && is_saudi { 
                gosi_employee / gross_monthly * dec!(100) 
            } else { 
                Decimal::ZERO 
            },
            legal_references: vec![
                "Saudi Labor Law (Royal Decree M/51)".to_string(),
                "GOSI Law (Royal Decree M/33)".to_string(),
            ],
        }
    }
}

impl Default for SaudiTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ISRAEL TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Israel tax config (complex system)
#[derive(Debug, Clone)]
pub struct IsraelConfig {
    pub tax_year: i32,
    pub credit_point_value: Decimal,       // ₪235/month
    pub basic_credit_points_male: Decimal, // 2.25
    pub basic_credit_points_female: Decimal, // 2.75
    // Bituach Leumi
    pub bl_reduced_ceiling: Decimal,       // 60% avg wage
    pub bl_employee_reduced: Decimal,      // 0.4%
    pub bl_employee_full: Decimal,         // 7%
    // Pension (mandatory)
    pub pension_employee: Decimal,         // 6%
    pub pension_employer: Decimal,         // 6.5%
    pub severance_rate: Decimal,           // 8.33%
}

impl Default for IsraelConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            credit_point_value: dec!(235),
            basic_credit_points_male: dec!(2.25),
            basic_credit_points_female: dec!(2.75),
            bl_reduced_ceiling: dec!(7_522),
            bl_employee_reduced: dec!(0.004),
            bl_employee_full: dec!(0.07),
            pension_employee: dec!(0.06),
            pension_employer: dec!(0.065),
            severance_rate: dec!(0.0833),
        }
    }
}

/// Israel tax calculator
pub struct IsraelTaxCalculator {
    config: IsraelConfig,
}

impl IsraelTaxCalculator {
    pub fn new() -> Self {
        Self { config: IsraelConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, is_female: bool) -> TaxResult {
        // Bituach Leumi (National Insurance)
        let bl_reduced = gross_monthly.min(self.config.bl_reduced_ceiling) * self.config.bl_employee_reduced;
        let bl_full = (gross_monthly - self.config.bl_reduced_ceiling).max(Decimal::ZERO) * self.config.bl_employee_full;
        let bituach_leumi = bl_reduced + bl_full;
        
        // Pension (mandatory)
        let pension_employee = gross_monthly * self.config.pension_employee;
        let pension_employer = gross_monthly * self.config.pension_employer;
        let severance = gross_monthly * self.config.severance_rate;
        
        // Income tax calculation (progressive)
        let taxable = gross_monthly - pension_employee;
        let tax_before_credits = self.calculate_brackets(taxable);
        
        // Credit points
        let credit_points = if is_female { 
            self.config.basic_credit_points_female 
        } else { 
            self.config.basic_credit_points_male 
        };
        let credits = credit_points * self.config.credit_point_value;
        let income_tax = (tax_before_credits - credits).max(Decimal::ZERO);
        
        let total_employee = bituach_leumi + pension_employee + income_tax;
        let total_employer = pension_employer + severance;
        
        TaxResult {
            country_code: "IL".to_string(),
            currency: "ILS".to_string(),
            gross_monthly,
            income_tax,
            social_security_employee: bituach_leumi,
            social_security_employer: Decimal::ZERO,
            pension_employee,
            pension_employer,
            other_employee: Decimal::ZERO,
            other_employer: severance,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { 
                total_employee / gross_monthly * dec!(100) 
            } else { 
                Decimal::ZERO 
            },
            legal_references: vec![
                "Income Tax Ordinance".to_string(),
                "National Insurance Law".to_string(),
                "Mandatory Pension Law 2008".to_string(),
            ],
        }
    }
    
    fn calculate_brackets(&self, taxable: Decimal) -> Decimal {
        // Israel 2024 monthly tax brackets
        let brackets: [(Decimal, Decimal); 7] = [
            (dec!(7_010), dec!(0.10)),
            (dec!(10_060), dec!(0.14)),
            (dec!(16_150), dec!(0.20)),
            (dec!(22_440), dec!(0.31)),
            (dec!(46_690), dec!(0.35)),
            (dec!(60_130), dec!(0.47)),
            (dec!(999_999_999), dec!(0.50)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in brackets {
            if taxable <= prev { break; }
            let bracket = taxable.min(max) - prev;
            tax += bracket * rate;
            prev = max;
        }
        tax
    }
}

impl Default for IsraelTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COMMON TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Tax calculation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxResult {
    pub country_code: String,
    pub currency: String,
    pub gross_monthly: Decimal,
    pub income_tax: Decimal,
    pub social_security_employee: Decimal,
    pub social_security_employer: Decimal,
    pub pension_employee: Decimal,
    pub pension_employer: Decimal,
    pub other_employee: Decimal,
    pub other_employer: Decimal,
    pub total_employee_deductions: Decimal,
    pub total_employer_contributions: Decimal,
    pub net_monthly: Decimal,
    pub effective_rate: Decimal,
    pub legal_references: Vec<String>,
}

/// Middle East country registry
pub struct MiddleEastRegistry;

impl MiddleEastRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            // GCC (zero income tax)
            ("AE", "UAE", "AED"),
            ("SA", "Saudi Arabia", "SAR"),
            ("QA", "Qatar", "QAR"),
            ("KW", "Kuwait", "KWD"),
            ("BH", "Bahrain", "BHD"),
            ("OM", "Oman", "OMR"),
            // Levant & Other
            ("IL", "Israel", "ILS"),
            ("JO", "Jordan", "JOD"),
            ("LB", "Lebanon", "LBP"),
            ("IQ", "Iraq", "IQD"),
            ("TR", "Turkey", "TRY"),
            ("IR", "Iran", "IRR"),
        ]
    }
    
    /// Check if country is GCC (zero income tax)
    pub fn is_gcc(country_code: &str) -> bool {
        matches!(country_code, "AE" | "SA" | "QA" | "KW" | "BH" | "OM")
    }
    
    /// Check if country requires WPS (Wage Protection System)
    pub fn requires_wps(country_code: &str) -> bool {
        matches!(country_code, "AE" | "SA" | "QA" | "BH" | "OM")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_uae_calculator_expat() {
        let calc = UAETaxCalculator::new();
        
        // Expat: No deductions (no GPSSA)
        let result = calc.calculate(dec!(20_000), false, 3);
        
        assert_eq!(result.country_code, "AE");
        assert_eq!(result.income_tax, Decimal::ZERO);
        assert_eq!(result.social_security_employee, Decimal::ZERO);
        assert_eq!(result.net_monthly, dec!(20_000));
    }
    
    #[test]
    fn test_uae_calculator_national() {
        let calc = UAETaxCalculator::new();
        
        // National: GPSSA applies
        let result = calc.calculate(dec!(20_000), true, 3);
        
        assert_eq!(result.country_code, "AE");
        assert!(result.social_security_employee > Decimal::ZERO); // 5%
        assert!(result.social_security_employer > Decimal::ZERO); // 12.5%
    }
    
    #[test]
    fn test_uae_gratuity() {
        let calc = UAETaxCalculator::new();
        
        // 3 years: 21 days * 3 * (salary/30)
        let gratuity = calc.calculate_gratuity(dec!(15_000), dec!(3), false);
        assert!(gratuity > Decimal::ZERO);
        
        // Resignation under 1 year: zero
        let gratuity_resign = calc.calculate_gratuity(dec!(15_000), dec!(0.5), true);
        assert_eq!(gratuity_resign, Decimal::ZERO);
    }
    
    #[test]
    fn test_saudi_calculator() {
        let calc = SaudiTaxCalculator::new();
        
        // Saudi national: 9% GOSI
        let result = calc.calculate(dec!(20_000), true);
        assert_eq!(result.country_code, "SA");
        assert!(result.social_security_employee > Decimal::ZERO);
        
        // Expat: No employee deduction
        let result_expat = calc.calculate(dec!(20_000), false);
        assert_eq!(result_expat.social_security_employee, Decimal::ZERO);
        assert!(result_expat.social_security_employer > Decimal::ZERO); // OCI 2%
    }
    
    #[test]
    fn test_israel_calculator() {
        let calc = IsraelTaxCalculator::new();
        
        // High earner
        let result = calc.calculate(dec!(30_000), false);
        
        assert_eq!(result.country_code, "IL");
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.social_security_employee > Decimal::ZERO);
        assert!(result.pension_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_middle_east_registry() {
        let countries = MiddleEastRegistry::supported_countries();
        assert!(countries.len() >= 12);
        
        assert!(MiddleEastRegistry::is_gcc("AE"));
        assert!(MiddleEastRegistry::is_gcc("SA"));
        assert!(!MiddleEastRegistry::is_gcc("IL"));
        
        assert!(MiddleEastRegistry::requires_wps("AE"));
        assert!(MiddleEastRegistry::requires_wps("SA"));
    }
}
