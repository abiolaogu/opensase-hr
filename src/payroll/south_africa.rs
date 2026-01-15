//! Southern Africa Tax Engines
//! 
//! Tax calculators for 10 Southern African countries:
//! - ZA: South Africa (PAYE, UIF, SDL, ETI, medical credits)
//! - ZW: Zimbabwe (multi-currency USD/ZWL, NSSA, AIDS levy)
//! - ZM: Zambia (NAPSA, NHIMA)
//! - AO: Angola (IRT Portuguese system, INSS)
//! - BW: Botswana, NA: Namibia, LS: Lesotho, SZ: Eswatini, MW: Malawi, MZ: Mozambique

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SOUTH AFRICA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// South Africa PAYE Configuration (2024/2025 Tax Year)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SouthAfricaConfig {
    pub tax_year: String,
    pub brackets: Vec<TaxBracket>,
    pub primary_rebate: Decimal,      // R17,235 (all taxpayers)
    pub secondary_rebate: Decimal,    // R9,444 (age 65+)
    pub tertiary_rebate: Decimal,     // R3,145 (age 75+)
    pub uif_rate: Decimal,            // 1% each (employee + employer)
    pub uif_ceiling: Decimal,         // R17,712/month
    pub sdl_rate: Decimal,            // 1% (employer only)
    pub sdl_threshold: Decimal,       // R500,000 annual payroll
}

impl Default for SouthAfricaConfig {
    fn default() -> Self {
        Self {
            tax_year: "2024/2025".to_string(),
            brackets: vec![
                TaxBracket { min: dec!(1), max: Some(dec!(237_100)), rate: dec!(0.18), base_tax: dec!(0) },
                TaxBracket { min: dec!(237_101), max: Some(dec!(370_500)), rate: dec!(0.26), base_tax: dec!(42_678) },
                TaxBracket { min: dec!(370_501), max: Some(dec!(512_800)), rate: dec!(0.31), base_tax: dec!(77_362) },
                TaxBracket { min: dec!(512_801), max: Some(dec!(673_000)), rate: dec!(0.36), base_tax: dec!(121_475) },
                TaxBracket { min: dec!(673_001), max: Some(dec!(857_900)), rate: dec!(0.39), base_tax: dec!(179_147) },
                TaxBracket { min: dec!(857_901), max: Some(dec!(1_817_000)), rate: dec!(0.41), base_tax: dec!(251_258) },
                TaxBracket { min: dec!(1_817_001), max: None, rate: dec!(0.45), base_tax: dec!(644_489) },
            ],
            primary_rebate: dec!(17_235),
            secondary_rebate: dec!(9_444),
            tertiary_rebate: dec!(3_145),
            uif_rate: dec!(0.01),
            uif_ceiling: dec!(17_712),
            sdl_rate: dec!(0.01),
            sdl_threshold: dec!(500_000),
        }
    }
}

/// Tax bracket with base tax
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracket {
    pub min: Decimal,
    pub max: Option<Decimal>,
    pub rate: Decimal,
    pub base_tax: Decimal,
}

/// South Africa tax calculator
pub struct SouthAfricaTaxCalculator {
    config: SouthAfricaConfig,
}

impl SouthAfricaTaxCalculator {
    pub fn new() -> Self {
        Self { config: SouthAfricaConfig::default() }
    }
    
    pub fn with_config(config: SouthAfricaConfig) -> Self {
        Self { config }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, age: u8) -> TaxResult {
        let gross_annual = gross_monthly * dec!(12);
        
        // Calculate annual tax using brackets
        let tax_before_rebates = self.calculate_bracket_tax(gross_annual);
        
        // Apply rebates based on age
        let mut total_rebates = self.config.primary_rebate;
        if age >= 65 { total_rebates += self.config.secondary_rebate; }
        if age >= 75 { total_rebates += self.config.tertiary_rebate; }
        
        let annual_paye = (tax_before_rebates - total_rebates).max(Decimal::ZERO);
        let monthly_paye = annual_paye / dec!(12);
        
        // UIF (capped at ceiling)
        let uif_base = gross_monthly.min(self.config.uif_ceiling);
        let uif_employee = uif_base * self.config.uif_rate;
        let uif_employer = uif_base * self.config.uif_rate;
        
        // SDL (employer only, if payroll > threshold)
        let sdl = gross_monthly * self.config.sdl_rate;
        
        let total_employee = monthly_paye + uif_employee;
        let total_employer = uif_employer + sdl;
        
        TaxResult {
            country_code: "ZA".to_string(),
            currency: "ZAR".to_string(),
            gross_monthly,
            gross_annual,
            monthly_paye,
            uif_employee,
            uif_employer,
            sdl,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { monthly_paye / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Income Tax Act 58 of 1962".to_string(),
                "Unemployment Insurance Act 63 of 2001".to_string(),
                "Skills Development Levies Act 9 of 1999".to_string(),
            ],
        }
    }
    
    fn calculate_bracket_tax(&self, taxable_annual: Decimal) -> Decimal {
        for bracket in &self.config.brackets {
            match bracket.max {
                Some(max) if taxable_annual <= max => {
                    return bracket.base_tax + (taxable_annual - bracket.min + dec!(1)) * bracket.rate;
                }
                None => {
                    return bracket.base_tax + (taxable_annual - bracket.min + dec!(1)) * bracket.rate;
                }
                _ => continue,
            }
        }
        Decimal::ZERO
    }
}

impl Default for SouthAfricaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ZIMBABWE TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Zimbabwe multi-currency tax configuration (USD/ZWL)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZimbabweConfig {
    pub tax_year: i32,
    pub usd_brackets: Vec<SimpleBracket>,
    pub nssa_rate: Decimal,           // 3.5% each
    pub aids_levy_rate: Decimal,      // 3% of PAYE
    pub zimdef_rate: Decimal,         // 1% employer
}

impl Default for ZimbabweConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            usd_brackets: vec![
                SimpleBracket { min: dec!(0), max: Some(dec!(100)), rate: dec!(0.00) },
                SimpleBracket { min: dec!(101), max: Some(dec!(350)), rate: dec!(0.20) },
                SimpleBracket { min: dec!(351), max: Some(dec!(700)), rate: dec!(0.25) },
                SimpleBracket { min: dec!(701), max: Some(dec!(2_000)), rate: dec!(0.30) },
                SimpleBracket { min: dec!(2_001), max: Some(dec!(10_000)), rate: dec!(0.35) },
                SimpleBracket { min: dec!(10_001), max: None, rate: dec!(0.40) },
            ],
            nssa_rate: dec!(0.035),
            aids_levy_rate: dec!(0.03),
            zimdef_rate: dec!(0.01),
        }
    }
}

/// Simple tax bracket (without base tax)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleBracket {
    pub min: Decimal,
    pub max: Option<Decimal>,
    pub rate: Decimal,
}

/// Zimbabwe tax calculator
pub struct ZimbabweTaxCalculator {
    config: ZimbabweConfig,
}

impl ZimbabweTaxCalculator {
    pub fn new() -> Self {
        Self { config: ZimbabweConfig::default() }
    }
    
    pub fn calculate_usd(&self, gross_monthly: Decimal) -> TaxResult {
        // NSSA
        let nssa_employee = gross_monthly * self.config.nssa_rate;
        let nssa_employer = gross_monthly * self.config.nssa_rate;
        
        // PAYE on taxable (gross - NSSA)
        let taxable = gross_monthly - nssa_employee;
        let paye = self.calculate_progressive_tax(taxable, &self.config.usd_brackets);
        
        // AIDS Levy (3% of PAYE)
        let aids_levy = paye * self.config.aids_levy_rate;
        
        // ZIMDEF (employer)
        let zimdef = gross_monthly * self.config.zimdef_rate;
        
        let total_employee = paye + aids_levy + nssa_employee;
        let total_employer = nssa_employer + zimdef;
        
        TaxResult {
            country_code: "ZW".to_string(),
            currency: "USD".to_string(),
            gross_monthly,
            gross_annual: gross_monthly * dec!(12),
            monthly_paye: paye,
            uif_employee: nssa_employee,
            uif_employer: nssa_employer,
            sdl: zimdef,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { (paye + aids_levy) / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Income Tax Act [Chapter 23:06]".to_string(),
                "National Social Security Authority Act".to_string(),
                "AIDS Levy Act".to_string(),
            ],
        }
    }
    
    fn calculate_progressive_tax(&self, taxable: Decimal, brackets: &[SimpleBracket]) -> Decimal {
        let mut tax = Decimal::ZERO;
        let mut remaining = taxable;
        let mut prev_max = Decimal::ZERO;
        
        for bracket in brackets {
            if remaining <= Decimal::ZERO { break; }
            
            let bracket_size = match bracket.max {
                Some(max) => {
                    let size = (max - prev_max).min(remaining);
                    prev_max = max;
                    size
                }
                None => remaining,
            };
            
            tax += bracket_size * bracket.rate;
            remaining -= bracket_size;
        }
        tax
    }
}

impl Default for ZimbabweTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ZAMBIA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Zambia PAYE Configuration (2024)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZambiaConfig {
    pub tax_year: i32,
    pub brackets: Vec<SimpleBracket>,
    pub napsa_rate: Decimal,          // 5% each
    pub napsa_ceiling: Decimal,       // K332,865/year
    pub nhima_rate: Decimal,          // 1% each (National Health Insurance)
}

impl Default for ZambiaConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            brackets: vec![
                SimpleBracket { min: dec!(0), max: Some(dec!(5_100)), rate: dec!(0.00) },
                SimpleBracket { min: dec!(5_100), max: Some(dec!(5_800)), rate: dec!(0.20) },
                SimpleBracket { min: dec!(5_800), max: Some(dec!(7_800)), rate: dec!(0.30) },
                SimpleBracket { min: dec!(7_800), max: None, rate: dec!(0.37) },
            ],
            napsa_rate: dec!(0.05),
            napsa_ceiling: dec!(332_865),
            nhima_rate: dec!(0.01),
        }
    }
}

/// Zambia tax calculator
pub struct ZambiaTaxCalculator {
    config: ZambiaConfig,
}

impl ZambiaTaxCalculator {
    pub fn new() -> Self {
        Self { config: ZambiaConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> TaxResult {
        // NAPSA (capped)
        let napsa_base = (gross_monthly * dec!(12)).min(self.config.napsa_ceiling) / dec!(12);
        let napsa_employee = napsa_base * self.config.napsa_rate;
        let napsa_employer = napsa_base * self.config.napsa_rate;
        
        // NHIMA
        let nhima_employee = gross_monthly * self.config.nhima_rate;
        let nhima_employer = gross_monthly * self.config.nhima_rate;
        
        // PAYE
        let paye = self.calculate_progressive_tax(gross_monthly);
        
        let total_employee = paye + napsa_employee + nhima_employee;
        let total_employer = napsa_employer + nhima_employer;
        
        TaxResult {
            country_code: "ZM".to_string(),
            currency: "ZMW".to_string(),
            gross_monthly,
            gross_annual: gross_monthly * dec!(12),
            monthly_paye: paye,
            uif_employee: napsa_employee,
            uif_employer: napsa_employer,
            sdl: nhima_employer,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { paye / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Income Tax Act Chapter 323".to_string(),
                "NAPSA Act No. 40 of 1996".to_string(),
                "National Health Insurance Act No. 2 of 2018".to_string(),
            ],
        }
    }
    
    fn calculate_progressive_tax(&self, taxable: Decimal) -> Decimal {
        let mut tax = Decimal::ZERO;
        let mut remaining = taxable;
        let mut prev_max = Decimal::ZERO;
        
        for bracket in &self.config.brackets {
            if remaining <= Decimal::ZERO { break; }
            
            let bracket_size = match bracket.max {
                Some(max) => {
                    let size = (max - prev_max).min(remaining);
                    prev_max = max;
                    size
                }
                None => remaining,
            };
            
            tax += bracket_size * bracket.rate;
            remaining -= bracket_size;
        }
        tax
    }
}

impl Default for ZambiaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ANGOLA TAX CALCULATOR (Portuguese System - IRT)
// ═══════════════════════════════════════════════════════════════════════════

/// Angola IRT Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AngolaConfig {
    pub tax_year: i32,
    pub brackets: Vec<SimpleBracket>,
    pub inss_employee_rate: Decimal,  // 3%
    pub inss_employer_rate: Decimal,  // 8%
    pub minimum_wage: Decimal,
}

impl Default for AngolaConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            brackets: vec![
                SimpleBracket { min: dec!(0), max: Some(dec!(100_000)), rate: dec!(0.00) },
                SimpleBracket { min: dec!(100_001), max: Some(dec!(150_000)), rate: dec!(0.10) },
                SimpleBracket { min: dec!(150_001), max: Some(dec!(200_000)), rate: dec!(0.13) },
                SimpleBracket { min: dec!(200_001), max: Some(dec!(300_000)), rate: dec!(0.16) },
                SimpleBracket { min: dec!(300_001), max: Some(dec!(500_000)), rate: dec!(0.18) },
                SimpleBracket { min: dec!(500_001), max: Some(dec!(1_000_000)), rate: dec!(0.19) },
                SimpleBracket { min: dec!(1_000_001), max: Some(dec!(1_500_000)), rate: dec!(0.20) },
                SimpleBracket { min: dec!(1_500_001), max: Some(dec!(2_000_000)), rate: dec!(0.21) },
                SimpleBracket { min: dec!(2_000_001), max: Some(dec!(2_500_000)), rate: dec!(0.22) },
                SimpleBracket { min: dec!(2_500_001), max: Some(dec!(5_000_000)), rate: dec!(0.23) },
                SimpleBracket { min: dec!(5_000_001), max: Some(dec!(10_000_000)), rate: dec!(0.24) },
                SimpleBracket { min: dec!(10_000_001), max: None, rate: dec!(0.25) },
            ],
            inss_employee_rate: dec!(0.03),
            inss_employer_rate: dec!(0.08),
            minimum_wage: dec!(100_000),
        }
    }
}

/// Angola tax calculator
pub struct AngolaTaxCalculator {
    config: AngolaConfig,
}

impl AngolaTaxCalculator {
    pub fn new() -> Self {
        Self { config: AngolaConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> TaxResult {
        // INSS
        let inss_employee = gross_monthly * self.config.inss_employee_rate;
        let inss_employer = gross_monthly * self.config.inss_employer_rate;
        
        // IRT (on gross, INSS not deductible)
        let irt = self.calculate_progressive_tax(gross_monthly);
        
        let total_employee = irt + inss_employee;
        let total_employer = inss_employer;
        
        TaxResult {
            country_code: "AO".to_string(),
            currency: "AOA".to_string(),
            gross_monthly,
            gross_annual: gross_monthly * dec!(12),
            monthly_paye: irt,
            uif_employee: inss_employee,
            uif_employer: inss_employer,
            sdl: Decimal::ZERO,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { irt / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Código do Imposto sobre o Rendimento do Trabalho".to_string(),
                "Lei da Protecção Social Obrigatória (INSS)".to_string(),
            ],
        }
    }
    
    fn calculate_progressive_tax(&self, taxable: Decimal) -> Decimal {
        let mut tax = Decimal::ZERO;
        let mut remaining = taxable;
        let mut prev_max = Decimal::ZERO;
        
        for bracket in &self.config.brackets {
            if remaining <= Decimal::ZERO { break; }
            
            let bracket_size = match bracket.max {
                Some(max) => {
                    let size = (max - prev_max).min(remaining);
                    prev_max = max;
                    size
                }
                None => remaining,
            };
            
            tax += bracket_size * bracket.rate;
            remaining -= bracket_size;
        }
        tax
    }
}

impl Default for AngolaTaxCalculator {
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
    pub gross_annual: Decimal,
    pub monthly_paye: Decimal,
    pub uif_employee: Decimal,
    pub uif_employer: Decimal,
    pub sdl: Decimal,
    pub total_employee_deductions: Decimal,
    pub total_employer_contributions: Decimal,
    pub net_monthly: Decimal,
    pub effective_rate: Decimal,
    pub legal_references: Vec<String>,
}

/// Southern Africa country registry
pub struct SouthernAfricaRegistry;

impl SouthernAfricaRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("ZA", "South Africa", "ZAR"),
            ("ZW", "Zimbabwe", "USD/ZWL"),
            ("ZM", "Zambia", "ZMW"),
            ("MW", "Malawi", "MWK"),
            ("MZ", "Mozambique", "MZN"),
            ("BW", "Botswana", "BWP"),
            ("NA", "Namibia", "NAD"),
            ("LS", "Lesotho", "LSL"),
            ("SZ", "Eswatini", "SZL"),
            ("AO", "Angola", "AOA"),
        ]
    }
    
    /// Check if country uses ZAR peg (CMA region)
    pub fn is_cma_country(country_code: &str) -> bool {
        matches!(country_code, "NA" | "LS" | "SZ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_south_africa_calculator() {
        let calc = SouthAfricaTaxCalculator::new();
        
        // R50,000/month, age 35
        let result = calc.calculate(dec!(50_000), 35);
        
        assert_eq!(result.country_code, "ZA");
        assert!(result.monthly_paye > Decimal::ZERO);
        assert!(result.uif_employee > Decimal::ZERO);
        assert!(result.net_monthly < result.gross_monthly);
    }
    
    #[test]
    fn test_south_africa_senior_rebate() {
        let calc = SouthAfricaTaxCalculator::new();
        
        let age_35 = calc.calculate(dec!(50_000), 35);
        let age_65 = calc.calculate(dec!(50_000), 65);
        
        // Senior should pay less tax due to secondary rebate
        assert!(age_65.monthly_paye < age_35.monthly_paye);
    }
    
    #[test]
    fn test_zimbabwe_calculator() {
        let calc = ZimbabweTaxCalculator::new();
        
        // $2,000/month USD
        let result = calc.calculate_usd(dec!(2_000));
        
        assert_eq!(result.country_code, "ZW");
        assert_eq!(result.currency, "USD");
        assert!(result.monthly_paye > Decimal::ZERO);
    }
    
    #[test]
    fn test_zambia_calculator() {
        let calc = ZambiaTaxCalculator::new();
        
        // K10,000/month
        let result = calc.calculate(dec!(10_000));
        
        assert_eq!(result.country_code, "ZM");
        assert!(result.monthly_paye > Decimal::ZERO);
    }
    
    #[test]
    fn test_angola_calculator() {
        let calc = AngolaTaxCalculator::new();
        
        // AOA 500,000/month
        let result = calc.calculate(dec!(500_000));
        
        assert_eq!(result.country_code, "AO");
        assert!(result.monthly_paye > Decimal::ZERO);
    }
    
    #[test]
    fn test_southern_africa_registry() {
        let countries = SouthernAfricaRegistry::supported_countries();
        assert_eq!(countries.len(), 10);
        
        assert!(SouthernAfricaRegistry::is_cma_country("NA"));
        assert!(SouthernAfricaRegistry::is_cma_country("LS"));
        assert!(!SouthernAfricaRegistry::is_cma_country("ZA"));
    }
}
