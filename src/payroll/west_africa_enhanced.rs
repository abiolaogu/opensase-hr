//! Enhanced West Africa Tax Configurations
//! 
//! Detailed tax configurations with:
//! - Ghana: NHIL, GETFund, COVID-19 Levy
//! - CFA Zone: Country-specific brackets for SN, CI, ML, BF
//! - Labor law compliance: minimum wage, leave days, maternity weeks

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

/// Enhanced Ghana PAYE Configuration (2024)
/// Includes levies: NHIL, GETFund, COVID-19
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhanaEnhancedConfig {
    pub tax_year: i32,
    pub paye_brackets: Vec<TaxBracketMonthly>,
    pub ssnit_employee_rate: Decimal,     // 5.5%
    pub ssnit_employer_rate: Decimal,     // 13%
    pub tier2_employee_rate: Decimal,     // 5%
    pub tier3_voluntary_max: Decimal,     // Up to 16.5%
    pub nhil_rate: Decimal,               // 2.5% National Health Insurance Levy
    pub getfund_rate: Decimal,            // 2.5% Ghana Education Trust Fund
    pub covid_levy_rate: Decimal,         // 1% COVID-19 Recovery Levy
    pub minimum_wage_monthly: Decimal,
}

impl Default for GhanaEnhancedConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            paye_brackets: vec![
                TaxBracketMonthly { min: dec!(0), max: Some(dec!(490)), rate: dec!(0.00) },
                TaxBracketMonthly { min: dec!(490), max: Some(dec!(600)), rate: dec!(0.05) },
                TaxBracketMonthly { min: dec!(600), max: Some(dec!(730)), rate: dec!(0.10) },
                TaxBracketMonthly { min: dec!(730), max: Some(dec!(3896.67)), rate: dec!(0.175) },
                TaxBracketMonthly { min: dec!(3896.67), max: Some(dec!(20000)), rate: dec!(0.25) },
                TaxBracketMonthly { min: dec!(20000), max: Some(dec!(50000)), rate: dec!(0.30) },
                TaxBracketMonthly { min: dec!(50000), max: None, rate: dec!(0.35) },
            ],
            ssnit_employee_rate: dec!(0.055),
            ssnit_employer_rate: dec!(0.13),
            tier2_employee_rate: dec!(0.05),
            tier3_voluntary_max: dec!(0.165),
            nhil_rate: dec!(0.025),
            getfund_rate: dec!(0.025),
            covid_levy_rate: dec!(0.01),
            minimum_wage_monthly: dec!(16.94) * dec!(22) * dec!(8), // GHS 16.94/day
        }
    }
}

/// Monthly tax bracket (Ghana uses monthly brackets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBracketMonthly {
    pub min: Decimal,
    pub max: Option<Decimal>,
    pub rate: Decimal,
}

/// CFA Zone (BCEAO) Country Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CFAZoneConfig {
    pub country_code: String,
    pub country_name: String,
    pub currency: String,
    pub income_tax_brackets: Vec<AnnualTaxBracket>,
    pub social_security_employee: Decimal,
    pub social_security_employer: Decimal,
    pub pension_employee: Decimal,
    pub pension_employer: Decimal,
    pub health_insurance_rate: Decimal,
    pub minimum_wage_monthly: Decimal,
    pub work_hours_weekly: u8,
    pub paid_leave_days: u8,
    pub maternity_leave_weeks: u8,
    pub social_security_agency: String,
    pub legal_references: Vec<String>,
}

/// Annual tax bracket (CFA countries use annual brackets)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnualTaxBracket {
    pub min: Decimal,
    pub max: Option<Decimal>,
    pub rate: Decimal,
}

impl CFAZoneConfig {
    /// Senegal configuration (CSS/IPRES)
    pub fn senegal() -> Self {
        Self {
            country_code: "SN".to_string(),
            country_name: "Senegal".to_string(),
            currency: "XOF".to_string(),
            income_tax_brackets: vec![
                AnnualTaxBracket { min: dec!(0), max: Some(dec!(630_000)), rate: dec!(0.00) },
                AnnualTaxBracket { min: dec!(630_000), max: Some(dec!(1_500_000)), rate: dec!(0.20) },
                AnnualTaxBracket { min: dec!(1_500_000), max: Some(dec!(4_000_000)), rate: dec!(0.30) },
                AnnualTaxBracket { min: dec!(4_000_000), max: Some(dec!(8_000_000)), rate: dec!(0.35) },
                AnnualTaxBracket { min: dec!(8_000_000), max: Some(dec!(13_500_000)), rate: dec!(0.37) },
                AnnualTaxBracket { min: dec!(13_500_000), max: None, rate: dec!(0.40) },
            ],
            social_security_employee: dec!(0.056),
            social_security_employer: dec!(0.155),
            pension_employee: dec!(0.056),
            pension_employer: dec!(0.084),
            health_insurance_rate: dec!(0.07),
            minimum_wage_monthly: dec!(58_900),
            work_hours_weekly: 40,
            paid_leave_days: 24,
            maternity_leave_weeks: 14,
            social_security_agency: "CSS/IPRES".to_string(),
            legal_references: vec![
                "Code Général des Impôts du Sénégal".to_string(),
                "Code du Travail (Loi n° 97-17)".to_string(),
                "Code de la Sécurité Sociale".to_string(),
            ],
        }
    }
    
    /// Côte d'Ivoire configuration (CNPS)
    pub fn cote_divoire() -> Self {
        Self {
            country_code: "CI".to_string(),
            country_name: "Côte d'Ivoire".to_string(),
            currency: "XOF".to_string(),
            income_tax_brackets: vec![
                AnnualTaxBracket { min: dec!(0), max: Some(dec!(300_000)), rate: dec!(0.00) },
                AnnualTaxBracket { min: dec!(300_000), max: Some(dec!(547_000)), rate: dec!(0.016) },
                AnnualTaxBracket { min: dec!(547_000), max: Some(dec!(979_000)), rate: dec!(0.05) },
                AnnualTaxBracket { min: dec!(979_000), max: Some(dec!(1_519_000)), rate: dec!(0.10) },
                AnnualTaxBracket { min: dec!(1_519_000), max: Some(dec!(2_644_000)), rate: dec!(0.15) },
                AnnualTaxBracket { min: dec!(2_644_000), max: Some(dec!(4_669_000)), rate: dec!(0.20) },
                AnnualTaxBracket { min: dec!(4_669_000), max: Some(dec!(10_106_000)), rate: dec!(0.25) },
                AnnualTaxBracket { min: dec!(10_106_000), max: None, rate: dec!(0.36) },
            ],
            social_security_employee: dec!(0.063),
            social_security_employer: dec!(0.155),
            pension_employee: dec!(0.063),
            pension_employer: dec!(0.077),
            health_insurance_rate: dec!(0.05),  // CMU
            minimum_wage_monthly: dec!(75_000),
            work_hours_weekly: 40,
            paid_leave_days: 26,
            maternity_leave_weeks: 14,
            social_security_agency: "CNPS".to_string(),
            legal_references: vec![
                "Code Général des Impôts de Côte d'Ivoire".to_string(),
                "Code du Travail (Loi n° 2015-532)".to_string(),
                "Code de Prévoyance Sociale".to_string(),
            ],
        }
    }
    
    /// Mali configuration (INPS)
    pub fn mali() -> Self {
        Self {
            country_code: "ML".to_string(),
            country_name: "Mali".to_string(),
            currency: "XOF".to_string(),
            income_tax_brackets: vec![
                AnnualTaxBracket { min: dec!(0), max: Some(dec!(330_000)), rate: dec!(0.00) },
                AnnualTaxBracket { min: dec!(330_000), max: Some(dec!(580_000)), rate: dec!(0.05) },
                AnnualTaxBracket { min: dec!(580_000), max: Some(dec!(960_000)), rate: dec!(0.13) },
                AnnualTaxBracket { min: dec!(960_000), max: Some(dec!(1_560_000)), rate: dec!(0.22) },
                AnnualTaxBracket { min: dec!(1_560_000), max: Some(dec!(3_000_000)), rate: dec!(0.30) },
                AnnualTaxBracket { min: dec!(3_000_000), max: None, rate: dec!(0.40) },
            ],
            social_security_employee: dec!(0.036),
            social_security_employer: dec!(0.146),
            pension_employee: dec!(0.036),
            pension_employer: dec!(0.088),
            health_insurance_rate: dec!(0.04),
            minimum_wage_monthly: dec!(40_000),
            work_hours_weekly: 40,
            paid_leave_days: 30,
            maternity_leave_weeks: 14,
            social_security_agency: "INPS".to_string(),
            legal_references: vec![
                "Code Général des Impôts du Mali".to_string(),
                "Code du Travail (Loi n° 92-020)".to_string(),
            ],
        }
    }
    
    /// Burkina Faso configuration (CNSS)
    pub fn burkina_faso() -> Self {
        Self {
            country_code: "BF".to_string(),
            country_name: "Burkina Faso".to_string(),
            currency: "XOF".to_string(),
            income_tax_brackets: vec![
                AnnualTaxBracket { min: dec!(0), max: Some(dec!(360_000)), rate: dec!(0.00) },
                AnnualTaxBracket { min: dec!(360_000), max: Some(dec!(600_000)), rate: dec!(0.12) },
                AnnualTaxBracket { min: dec!(600_000), max: Some(dec!(960_000)), rate: dec!(0.15) },
                AnnualTaxBracket { min: dec!(960_000), max: Some(dec!(1_440_000)), rate: dec!(0.20) },
                AnnualTaxBracket { min: dec!(1_440_000), max: Some(dec!(2_040_000)), rate: dec!(0.22) },
                AnnualTaxBracket { min: dec!(2_040_000), max: Some(dec!(3_000_000)), rate: dec!(0.25) },
                AnnualTaxBracket { min: dec!(3_000_000), max: None, rate: dec!(0.275) },
            ],
            social_security_employee: dec!(0.055),
            social_security_employer: dec!(0.165),
            pension_employee: dec!(0.055),
            pension_employer: dec!(0.055),
            health_insurance_rate: dec!(0.05),
            minimum_wage_monthly: dec!(34_664),
            work_hours_weekly: 40,
            paid_leave_days: 30,
            maternity_leave_weeks: 14,
            social_security_agency: "CNSS".to_string(),
            legal_references: vec![
                "Code Général des Impôts du Burkina Faso".to_string(),
                "Code du Travail (Loi n° 028-2008/AN)".to_string(),
            ],
        }
    }
    
    /// Get config for any CFA country
    pub fn for_country(country_code: &str) -> Option<Self> {
        match country_code {
            "SN" => Some(Self::senegal()),
            "CI" => Some(Self::cote_divoire()),
            "ML" => Some(Self::mali()),
            "BF" => Some(Self::burkina_faso()),
            _ => None,
        }
    }
}

/// Phone number validation patterns for West Africa
pub fn validate_phone_number(phone: &str, country: &str) -> (bool, String) {
    let patterns: std::collections::HashMap<&str, (&str, usize)> = [
        ("NG", ("+234", 14)),  // +234xxxxxxxxxx (10 local digits)
        ("GH", ("+233", 13)),  // +233xxxxxxxxx (9 local digits)
        ("SN", ("+221", 13)),  // +221xxxxxxxxx (9 local digits)
        ("CI", ("+225", 14)),  // +225xxxxxxxxxx (10 local digits)
        ("ML", ("+223", 12)),  // +223xxxxxxxx (8 local digits)
        ("BF", ("+226", 12)),  // +226xxxxxxxx (8 local digits)
        ("NE", ("+227", 12)),  // +227xxxxxxxx (8 local digits)
        ("GN", ("+224", 13)),  // +224xxxxxxxxx (9 local digits)
        ("BJ", ("+229", 12)),  // +229xxxxxxxx (8 local digits)
        ("TG", ("+228", 12)),  // +228xxxxxxxx (8 local digits)
        ("SL", ("+232", 12)),  // +232xxxxxxxx (8 local digits)
        ("LR", ("+231", 11)),  // +231xxxxxxx (7 local digits)
        ("MR", ("+222", 12)),  // +222xxxxxxxx (8 local digits)
        ("GW", ("+245", 11)),  // +245xxxxxxx (7 local digits)
        ("GM", ("+220", 11)),  // +220xxxxxxx (7 local digits)
        ("CV", ("+238", 11)),  // +238xxxxxxx (7 local digits)
    ].into_iter().collect();
    
    if let Some(&(prefix, expected_len)) = patterns.get(country) {
        if phone.len() == expected_len && phone.starts_with(prefix) {
            return (true, String::new());
        }
        return (false, format!("Expected {} format with {} digits", prefix, expected_len));
    }
    
    (false, format!("Unknown country code: {}", country))
}

/// Legal framework summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborLawSummary {
    pub country_code: String,
    pub minimum_wage: MinimumWage,
    pub working_hours: WorkingHours,
    pub leave_entitlements: LeaveEntitlements,
    pub termination_rules: TerminationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimumWage {
    pub monthly: Decimal,
    pub currency: String,
    pub effective_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingHours {
    pub standard_weekly: u8,
    pub max_daily: u8,
    pub overtime_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveEntitlements {
    pub annual_leave_days: u8,
    pub sick_leave_days: u8,
    pub maternity_weeks: u8,
    pub paternity_days: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminationRules {
    pub notice_period_months: u8,
    pub severance_per_year: Decimal,
    pub max_severance_months: u8,
}

impl LaborLawSummary {
    pub fn for_nigeria() -> Self {
        Self {
            country_code: "NG".to_string(),
            minimum_wage: MinimumWage {
                monthly: dec!(70_000),
                currency: "NGN".to_string(),
                effective_date: "2024-01-01".to_string(),
            },
            working_hours: WorkingHours {
                standard_weekly: 40,
                max_daily: 8,
                overtime_rate: dec!(1.5),
            },
            leave_entitlements: LeaveEntitlements {
                annual_leave_days: 6, // After 12 months
                sick_leave_days: 12,
                maternity_weeks: 12,
                paternity_days: 0, // Not mandated
            },
            termination_rules: TerminationRules {
                notice_period_months: 1,
                severance_per_year: dec!(0), // Not mandated
                max_severance_months: 0,
            },
        }
    }
    
    pub fn for_ghana() -> Self {
        Self {
            country_code: "GH".to_string(),
            minimum_wage: MinimumWage {
                monthly: dec!(16.94) * dec!(22),
                currency: "GHS".to_string(),
                effective_date: "2024-01-01".to_string(),
            },
            working_hours: WorkingHours {
                standard_weekly: 40,
                max_daily: 8,
                overtime_rate: dec!(1.5),
            },
            leave_entitlements: LeaveEntitlements {
                annual_leave_days: 15,
                sick_leave_days: 0, // No statutory minimum
                maternity_weeks: 12,
                paternity_days: 0,
            },
            termination_rules: TerminationRules {
                notice_period_months: 1,
                severance_per_year: dec!(1), // 1 week per year
                max_severance_months: 3,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cfa_zone_configs() {
        let sn = CFAZoneConfig::senegal();
        assert_eq!(sn.country_code, "SN");
        assert_eq!(sn.social_security_agency, "CSS/IPRES");
        
        let ci = CFAZoneConfig::cote_divoire();
        assert_eq!(ci.country_code, "CI");
        assert_eq!(ci.income_tax_brackets.len(), 8);
        
        let ml = CFAZoneConfig::mali();
        assert_eq!(ml.minimum_wage_monthly, dec!(40_000));
        
        let bf = CFAZoneConfig::burkina_faso();
        assert_eq!(bf.paid_leave_days, 30);
    }
    
    #[test]
    fn test_phone_validation() {
        // Nigeria: +234 (4) + 10 digits = 14 chars
        let (valid, _) = validate_phone_number("+2348031234567", "NG");
        assert!(valid);
        
        // Ghana: +233 (4) + 9 digits = 13 chars
        let (valid, _) = validate_phone_number("+233201234567", "GH");
        assert!(valid);
        
        // Invalid: no country code
        let (valid, err) = validate_phone_number("08031234567", "NG");
        assert!(!valid);
        assert!(!err.is_empty());
    }
    
    #[test]
    fn test_labor_law_summary() {
        let ng = LaborLawSummary::for_nigeria();
        assert_eq!(ng.minimum_wage.monthly, dec!(70_000));
        assert_eq!(ng.leave_entitlements.maternity_weeks, 12);
        
        let gh = LaborLawSummary::for_ghana();
        assert_eq!(gh.leave_entitlements.annual_leave_days, 15);
    }
}
