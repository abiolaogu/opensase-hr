//! West Africa Tax Calculators
//! 
//! Comprehensive tax engines for 16 West African countries including:
//! - Nigeria (PAYE, PenCom, NHF, NHIS)
//! - Ghana (PAYE, SSNIT Tier 1/2/3)
//! - UEMOA/CFA Zone (CI, SN, ML, BF, NE, GW, BJ, TG)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::NaiveDate;

/// Tax bracket for progressive tax calculation
#[derive(Debug, Clone)]
pub struct TaxBracket {
    pub min: Decimal,
    pub max: Option<Decimal>,
    pub rate: Decimal,
}

/// Tax calculation result
#[derive(Debug, Clone)]
pub struct TaxResult {
    pub gross_annual: Decimal,
    pub taxable_income: Decimal,
    pub total_tax: Decimal,
    pub effective_rate: Decimal,
    pub employee_deductions: Vec<TaxComponent>,
    pub employer_contributions: Vec<TaxComponent>,
    pub net_annual: Decimal,
}

/// Individual tax component
#[derive(Debug, Clone)]
pub struct TaxComponent {
    pub name: String,
    pub amount: Decimal,
    pub rate: Option<Decimal>,
}

// ═══════════════════════════════════════════════════════════════════════════
// GHANA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Ghana tax calculator - PAYE, SSNIT Tier 1/2/3
/// Reference: Income Tax Act 2015 (Act 896), National Pensions Act 2008 (Act 766)
pub struct GhanaTaxCalculator {
    paye_brackets: Vec<TaxBracket>,
    ssnit_rate_employee: Decimal,  // 5.5% Tier 1
    ssnit_rate_employer: Decimal,  // 13% Tier 1
    tier2_rate_employee: Decimal,  // 5% mandatory
}

impl GhanaTaxCalculator {
    pub fn new() -> Self {
        Self {
            // Ghana PAYE brackets 2024 (GHS per annum)
            paye_brackets: vec![
                TaxBracket { min: dec!(0), max: Some(dec!(5_880)), rate: dec!(0.0) },
                TaxBracket { min: dec!(5_880), max: Some(dec!(7_200)), rate: dec!(0.05) },
                TaxBracket { min: dec!(7_200), max: Some(dec!(8_760)), rate: dec!(0.10) },
                TaxBracket { min: dec!(8_760), max: Some(dec!(52_800)), rate: dec!(0.175) },
                TaxBracket { min: dec!(52_800), max: Some(dec!(268_800)), rate: dec!(0.25) },
                TaxBracket { min: dec!(268_800), max: Some(dec!(688_800)), rate: dec!(0.30) },
                TaxBracket { min: dec!(688_800), max: None, rate: dec!(0.35) },
            ],
            ssnit_rate_employee: dec!(0.055),
            ssnit_rate_employer: dec!(0.13),
            tier2_rate_employee: dec!(0.05),
        }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> TaxResult {
        // 1. Calculate pension contributions
        let ssnit_employee = gross_annual * self.ssnit_rate_employee;
        let tier2_employee = gross_annual * self.tier2_rate_employee;
        let total_pension_relief = ssnit_employee + tier2_employee;
        
        // 2. Calculate taxable income
        let taxable_income = (gross_annual - total_pension_relief).max(Decimal::ZERO);
        
        // 3. Calculate PAYE
        let paye = self.calculate_progressive_tax(taxable_income);
        
        // 4. Employer contributions
        let ssnit_employer = gross_annual * self.ssnit_rate_employer;
        
        let total_employee_deductions = paye + ssnit_employee + tier2_employee;
        
        TaxResult {
            gross_annual,
            taxable_income,
            total_tax: paye,
            effective_rate: if gross_annual > Decimal::ZERO { 
                paye / gross_annual * dec!(100) 
            } else { 
                Decimal::ZERO 
            },
            employee_deductions: vec![
                TaxComponent { name: "PAYE".to_string(), amount: paye, rate: None },
                TaxComponent { name: "SSNIT Tier 1".to_string(), amount: ssnit_employee, rate: Some(self.ssnit_rate_employee) },
                TaxComponent { name: "Tier 2 Pension".to_string(), amount: tier2_employee, rate: Some(self.tier2_rate_employee) },
            ],
            employer_contributions: vec![
                TaxComponent { name: "SSNIT Tier 1 (Employer)".to_string(), amount: ssnit_employer, rate: Some(self.ssnit_rate_employer) },
            ],
            net_annual: gross_annual - total_employee_deductions,
        }
    }
    
    fn calculate_progressive_tax(&self, taxable_income: Decimal) -> Decimal {
        let mut remaining = taxable_income;
        let mut total_tax = Decimal::ZERO;
        let mut previous_max = Decimal::ZERO;
        
        for bracket in &self.paye_brackets {
            let bracket_max = bracket.max.unwrap_or(Decimal::MAX);
            let bracket_width = bracket_max - previous_max;
            
            if remaining <= Decimal::ZERO {
                break;
            }
            
            let taxable_in_bracket = remaining.min(bracket_width);
            total_tax += taxable_in_bracket * bracket.rate;
            remaining -= taxable_in_bracket;
            previous_max = bracket_max;
        }
        
        total_tax
    }
}

impl Default for GhanaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// UEMOA/CFA ZONE TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// UEMOA harmonized tax calculator for CFA Franc zone countries
/// Covers: CI (Côte d'Ivoire), SN (Senegal), ML (Mali), BF (Burkina Faso),
///         NE (Niger), GW (Guinea-Bissau), BJ (Benin), TG (Togo)
pub struct UemoaTaxCalculator {
    country_code: String,
    its_brackets: Vec<TaxBracket>,
    social_security_rate_employee: Decimal,
    social_security_rate_employer: Decimal,
    professional_expenses_rate: Decimal,
}

impl UemoaTaxCalculator {
    pub fn for_country(country_code: &str) -> Self {
        // UEMOA harmonized brackets (in XOF)
        let base_brackets = vec![
            TaxBracket { min: dec!(0), max: Some(dec!(630_000)), rate: dec!(0.0) },
            TaxBracket { min: dec!(630_000), max: Some(dec!(1_500_000)), rate: dec!(0.10) },
            TaxBracket { min: dec!(1_500_000), max: Some(dec!(4_000_000)), rate: dec!(0.15) },
            TaxBracket { min: dec!(4_000_000), max: Some(dec!(8_000_000)), rate: dec!(0.20) },
            TaxBracket { min: dec!(8_000_000), max: Some(dec!(13_500_000)), rate: dec!(0.25) },
            TaxBracket { min: dec!(13_500_000), max: Some(dec!(50_000_000)), rate: dec!(0.30) },
            TaxBracket { min: dec!(50_000_000), max: None, rate: dec!(0.35) },
        ];
        
        // Country-specific social security rates
        let (ss_employee, ss_employer) = match country_code {
            "CI" => (dec!(0.063), dec!(0.156)),  // Côte d'Ivoire
            "SN" => (dec!(0.056), dec!(0.164)),  // Senegal
            "ML" => (dec!(0.036), dec!(0.164)),  // Mali
            "BF" => (dec!(0.055), dec!(0.160)),  // Burkina Faso
            "NE" => (dec!(0.016), dec!(0.160)),  // Niger
            "BJ" => (dec!(0.036), dec!(0.154)),  // Benin
            "TG" => (dec!(0.040), dec!(0.170)),  // Togo
            _    => (dec!(0.056), dec!(0.164)),  // Default UEMOA average
        };
        
        Self {
            country_code: country_code.to_string(),
            its_brackets: base_brackets,
            social_security_rate_employee: ss_employee,
            social_security_rate_employer: ss_employer,
            professional_expenses_rate: dec!(0.20), // 20% professional deduction
        }
    }
    
    pub fn calculate(&self, gross_annual: Decimal, family_parts: Decimal) -> TaxResult {
        // 1. Social security contributions
        let ss_employee = gross_annual * self.social_security_rate_employee;
        
        // 2. Professional expenses deduction
        let professional_deduction = gross_annual * self.professional_expenses_rate;
        
        // 3. Taxable income
        let taxable_income = (gross_annual - ss_employee - professional_deduction).max(Decimal::ZERO);
        
        // 4. Calculate ITS using quotient familial
        let its = self.calculate_its_with_quotient(taxable_income, family_parts);
        
        // 5. Employer contributions
        let ss_employer = gross_annual * self.social_security_rate_employer;
        
        let total_employee_deductions = its + ss_employee;
        
        TaxResult {
            gross_annual,
            taxable_income,
            total_tax: its,
            effective_rate: if gross_annual > Decimal::ZERO { 
                its / gross_annual * dec!(100) 
            } else { 
                Decimal::ZERO 
            },
            employee_deductions: vec![
                TaxComponent { name: "ITS".to_string(), amount: its, rate: None },
                TaxComponent { 
                    name: "Social Security".to_string(), 
                    amount: ss_employee, 
                    rate: Some(self.social_security_rate_employee) 
                },
            ],
            employer_contributions: vec![
                TaxComponent { 
                    name: "Social Security (Employer)".to_string(), 
                    amount: ss_employer, 
                    rate: Some(self.social_security_rate_employer) 
                },
            ],
            net_annual: gross_annual - total_employee_deductions,
        }
    }
    
    fn calculate_its_with_quotient(&self, taxable_income: Decimal, parts: Decimal) -> Decimal {
        // Quotient familial method:
        // 1. Divide income by parts
        // 2. Calculate tax on quotient  
        // 3. Multiply result by parts
        let quotient = taxable_income / parts;
        let tax_on_quotient = self.calculate_progressive_tax(quotient);
        tax_on_quotient * parts
    }
    
    fn calculate_progressive_tax(&self, income: Decimal) -> Decimal {
        let mut remaining = income;
        let mut total_tax = Decimal::ZERO;
        let mut previous_max = Decimal::ZERO;
        
        for bracket in &self.its_brackets {
            let bracket_max = bracket.max.unwrap_or(Decimal::MAX);
            let bracket_width = bracket_max - previous_max;
            
            if remaining <= Decimal::ZERO {
                break;
            }
            
            let taxable_in_bracket = remaining.min(bracket_width);
            total_tax += taxable_in_bracket * bracket.rate;
            remaining -= taxable_in_bracket;
            previous_max = bracket_max;
        }
        
        total_tax
    }
    
    pub fn country_code(&self) -> &str {
        &self.country_code
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// WEST AFRICA TAX REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Registry for all West African tax calculators
pub struct WestAfricaTaxRegistry;

impl WestAfricaTaxRegistry {
    /// Get supported countries
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("NG", "Nigeria", "NGN"),
            ("GH", "Ghana", "GHS"),
            ("CI", "Côte d'Ivoire", "XOF"),
            ("SN", "Senegal", "XOF"),
            ("ML", "Mali", "XOF"),
            ("BF", "Burkina Faso", "XOF"),
            ("NE", "Niger", "XOF"),
            ("GN", "Guinea", "GNF"),
            ("BJ", "Benin", "XOF"),
            ("TG", "Togo", "XOF"),
            ("SL", "Sierra Leone", "SLE"),
            ("LR", "Liberia", "LRD"),
            ("MR", "Mauritania", "MRU"),
            ("GW", "Guinea-Bissau", "XOF"),
            ("GM", "Gambia", "GMD"),
            ("CV", "Cape Verde", "CVE"),
        ]
    }
    
    /// Check if country is UEMOA/CFA zone
    pub fn is_uemoa_country(country_code: &str) -> bool {
        matches!(country_code, "CI" | "SN" | "ML" | "BF" | "NE" | "GW" | "BJ" | "TG")
    }
    
    /// Get currency for country
    pub fn get_currency(country_code: &str) -> &'static str {
        match country_code {
            "NG" => "NGN",
            "GH" => "GHS",
            "GN" => "GNF",
            "SL" => "SLE",
            "LR" => "LRD",
            "MR" => "MRU",
            "GM" => "GMD",
            "CV" => "CVE",
            _ => "XOF", // UEMOA countries use CFA Franc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ghana_tax_calculator() {
        let calculator = GhanaTaxCalculator::new();
        
        // Test with GHS 120,000 annual (10k/month)
        let result = calculator.calculate(dec!(120_000));
        
        assert!(result.total_tax > Decimal::ZERO);
        assert!(result.net_annual < result.gross_annual);
        
        // SSNIT should be 5.5%
        let ssnit = result.employee_deductions.iter()
            .find(|d| d.name == "SSNIT Tier 1")
            .unwrap();
        assert_eq!(ssnit.amount, dec!(120_000) * dec!(0.055));
    }
    
    #[test]
    fn test_uemoa_tax_calculator() {
        let calculator = UemoaTaxCalculator::for_country("CI");
        
        // Test with XOF 12,000,000 annual (1M/month)
        let result = calculator.calculate(dec!(12_000_000), dec!(1.0));
        
        assert!(result.total_tax > Decimal::ZERO);
        assert!(result.net_annual < result.gross_annual);
        assert_eq!(calculator.country_code(), "CI");
    }
    
    #[test]
    fn test_uemoa_family_quotient() {
        let calculator = UemoaTaxCalculator::for_country("SN");
        
        let single = calculator.calculate(dec!(12_000_000), dec!(1.0));
        let married_2kids = calculator.calculate(dec!(12_000_000), dec!(2.0)); // Married + 2 children = 2 parts
        
        // Family quotient should reduce tax
        assert!(married_2kids.total_tax < single.total_tax);
    }
    
    #[test]
    fn test_west_africa_registry() {
        let countries = WestAfricaTaxRegistry::supported_countries();
        assert_eq!(countries.len(), 16);
        
        assert!(WestAfricaTaxRegistry::is_uemoa_country("CI"));
        assert!(WestAfricaTaxRegistry::is_uemoa_country("SN"));
        assert!(!WestAfricaTaxRegistry::is_uemoa_country("NG"));
        assert!(!WestAfricaTaxRegistry::is_uemoa_country("GH"));
        
        assert_eq!(WestAfricaTaxRegistry::get_currency("NG"), "NGN");
        assert_eq!(WestAfricaTaxRegistry::get_currency("CI"), "XOF");
    }
}
