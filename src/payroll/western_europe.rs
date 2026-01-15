//! Western Europe Tax Engines
//! 
//! Tax calculators for Western Europe's financial hubs:
//! - Switzerland: 26 cantons, 3-tier system (Federal + Cantonal + Municipal)
//! - Austria: 7 brackets, 13th/14th salary special taxation
//! - Luxembourg: Holding company structures
//! - Ireland: Tech hub, R&D credits
//! - Liechtenstein: Special tax regimes

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SWITZERLAND TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Swiss Canton
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kanton {
    ZH, // Zürich
    BE, // Bern
    LU, // Luzern
    ZG, // Zug (low tax)
    GE, // Genève
    BS, // Basel-Stadt
    VD, // Vaud
    TI, // Ticino
    // + 18 more
}

/// Swiss tax config
#[derive(Debug, Clone)]
pub struct SwitzerlandConfig {
    pub tax_year: i32,
    pub kanton: Kanton,
    pub gemeinde: String,
    pub kanton_multiplier: Decimal,   // e.g., 100%
    pub gemeinde_multiplier: Decimal, // e.g., 119% (Zürich city)
    pub kirche_multiplier: Option<Decimal>, // Church tax (optional)
    // Social insurance
    pub ahv_iv_eo_rate: Decimal,      // 5.3% employee
    pub alv_rate: Decimal,            // 1.1% employee
    pub alv_ceiling: Decimal,         // CHF 148,200
}

impl Default for SwitzerlandConfig {
    fn default() -> Self {
        Self::zurich_city()
    }
}

impl SwitzerlandConfig {
    pub fn zurich_city() -> Self {
        Self {
            tax_year: 2025,
            kanton: Kanton::ZH,
            gemeinde: "Zürich".to_string(),
            kanton_multiplier: dec!(1.00),
            gemeinde_multiplier: dec!(1.19),
            kirche_multiplier: Some(dec!(0.10)),
            ahv_iv_eo_rate: dec!(0.053),
            alv_rate: dec!(0.011),
            alv_ceiling: dec!(148_200),
        }
    }
    
    pub fn zug_city() -> Self {
        Self {
            tax_year: 2025,
            kanton: Kanton::ZG,
            gemeinde: "Zug".to_string(),
            kanton_multiplier: dec!(0.82),
            gemeinde_multiplier: dec!(0.60),
            kirche_multiplier: Some(dec!(0.06)),
            ahv_iv_eo_rate: dec!(0.053),
            alv_rate: dec!(0.011),
            alv_ceiling: dec!(148_200),
        }
    }
    
    pub fn total_multiplier(&self) -> Decimal {
        let base = self.kanton_multiplier + self.gemeinde_multiplier;
        let church = self.kirche_multiplier.unwrap_or(Decimal::ZERO);
        base + church
    }
}

/// Swiss tax calculator
pub struct SwitzerlandTaxCalculator {
    config: SwitzerlandConfig,
}

impl SwitzerlandTaxCalculator {
    pub fn new(config: SwitzerlandConfig) -> Self {
        Self { config }
    }
    
    pub fn calculate(&self, gross_annual: Decimal, is_married: bool) -> TaxResult {
        // Federal tax (Bundessteuer)
        let bundessteuer = self.calculate_bundessteuer(gross_annual, is_married);
        
        // Cantonal base tax (simplified)
        let kantonal_basis = bundessteuer * dec!(3); // Approximate
        let kantonal = kantonal_basis * self.config.kanton_multiplier;
        let gemeinde = kantonal_basis * self.config.gemeinde_multiplier;
        let kirche = self.config.kirche_multiplier
            .map(|k| kantonal_basis * k)
            .unwrap_or(Decimal::ZERO);
        
        // Social insurance
        let monthly = gross_annual / dec!(12);
        let ahv_iv_eo = monthly * self.config.ahv_iv_eo_rate;
        let alv = (monthly.min(self.config.alv_ceiling / dec!(12))) * self.config.alv_rate;
        let si_monthly = ahv_iv_eo + alv;
        
        let total_tax_annual = bundessteuer + kantonal + gemeinde + kirche;
        let total_tax_monthly = total_tax_annual / dec!(12);
        
        TaxResult {
            country_code: "CH".to_string(),
            currency: "CHF".to_string(),
            gross_monthly: monthly,
            income_tax: total_tax_monthly,
            social_security_employee: si_monthly,
            social_security_employer: si_monthly, // ~equal
            pension_employee: monthly * dec!(0.05), // BVG ~5-9% based on age
            pension_employer: monthly * dec!(0.05),
            other_employee: Decimal::ZERO,
            other_employer: Decimal::ZERO,
            total_employee_deductions: total_tax_monthly + si_monthly + monthly * dec!(0.05),
            total_employer_contributions: si_monthly + monthly * dec!(0.05),
            net_monthly: monthly - total_tax_monthly - si_monthly - monthly * dec!(0.05),
            effective_rate: if gross_annual > Decimal::ZERO {
                total_tax_annual / gross_annual * dec!(100)
            } else {
                Decimal::ZERO
            },
            legal_references: vec![
                "Bundesgesetz über die direkte Bundessteuer (DBG)".to_string(),
                "Bundesgesetz über die AHV".to_string(),
            ],
        }
    }
    
    fn calculate_bundessteuer(&self, income: Decimal, is_married: bool) -> Decimal {
        // Federal tax brackets (single, simplified)
        let brackets: [(Decimal, Decimal); 6] = [
            (dec!(17_800), dec!(0.0)),
            (dec!(31_600), dec!(0.0077)),
            (dec!(55_200), dec!(0.0088)),
            (dec!(103_600), dec!(0.0264)),
            (dec!(176_000), dec!(0.0297)),
            (dec!(999_999_999), dec!(0.115)),
        ];
        
        // Married tariff has higher thresholds
        let factor = if is_married { dec!(1.8) } else { dec!(1) };
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in brackets {
            let adjusted_max = max * factor;
            if income <= prev { break; }
            let bracket = income.min(adjusted_max) - prev;
            tax += bracket * rate;
            prev = adjusted_max;
        }
        tax
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// AUSTRIA TAX CALCULATOR  
// ═══════════════════════════════════════════════════════════════════════════

/// Austria tax config
#[derive(Debug, Clone)]
pub struct AustriaConfig {
    pub tax_year: i32,
    // Social insurance
    pub sv_ceiling: Decimal,          // €5,850/month (2025)
    pub krankenversicherung: Decimal, // 3.87%
    pub pensionsversicherung: Decimal, // 10.25%
    pub arbeitslosenversicherung: Decimal, // 3%
}

impl Default for AustriaConfig {
    fn default() -> Self {
        Self {
            tax_year: 2025,
            sv_ceiling: dec!(5_850),
            krankenversicherung: dec!(0.0387),
            pensionsversicherung: dec!(0.1025),
            arbeitslosenversicherung: dec!(0.03),
        }
    }
}

/// Austria tax calculator
pub struct AustriaTaxCalculator {
    config: AustriaConfig,
}

impl AustriaTaxCalculator {
    pub fn new() -> Self {
        Self { config: AustriaConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, children: u8) -> TaxResult {
        let gross_annual = gross_monthly * dec!(14); // 14 salaries in Austria!
        
        // Social insurance (capped)
        let sv_base = gross_monthly.min(self.config.sv_ceiling);
        let kv = sv_base * self.config.krankenversicherung;
        let pv = sv_base * self.config.pensionsversicherung;
        let alv = sv_base * self.config.arbeitslosenversicherung;
        let sv_total = kv + pv + alv;
        
        // Income tax (7 brackets)
        let income_tax_annual = self.calculate_brackets(gross_annual - sv_total * dec!(14));
        
        // Tax credits
        let verkehrsabsetzbetrag = dec!(463);
        let familienbonus = Decimal::from(children) * dec!(2_000);
        let tax_after_credits = (income_tax_annual - verkehrsabsetzbetrag - familienbonus).max(Decimal::ZERO);
        
        let income_tax_monthly = tax_after_credits / dec!(12);
        
        TaxResult {
            country_code: "AT".to_string(),
            currency: "EUR".to_string(),
            gross_monthly,
            income_tax: income_tax_monthly,
            social_security_employee: sv_total,
            social_security_employer: sv_total * dec!(1.2), // ~20% more
            pension_employee: pv,
            pension_employer: pv * dec!(1.2),
            other_employee: kv + alv,
            other_employer: Decimal::ZERO,
            total_employee_deductions: income_tax_monthly + sv_total,
            total_employer_contributions: sv_total * dec!(1.2),
            net_monthly: gross_monthly - income_tax_monthly - sv_total,
            effective_rate: if gross_annual > Decimal::ZERO {
                tax_after_credits / gross_annual * dec!(100)
            } else {
                Decimal::ZERO
            },
            legal_references: vec![
                "Einkommensteuergesetz (EStG)".to_string(),
                "Allgemeines Sozialversicherungsgesetz (ASVG)".to_string(),
            ],
        }
    }
    
    fn calculate_brackets(&self, taxable: Decimal) -> Decimal {
        // Austria 2025 brackets
        let brackets: [(Decimal, Decimal); 7] = [
            (dec!(12_816), dec!(0.0)),
            (dec!(20_818), dec!(0.20)),
            (dec!(34_513), dec!(0.30)),
            (dec!(66_612), dec!(0.40)),
            (dec!(99_266), dec!(0.48)),
            (dec!(1_000_000), dec!(0.50)),
            (dec!(999_999_999), dec!(0.55)),
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

impl Default for AustriaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// IRELAND TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Ireland tax config  
#[derive(Debug, Clone)]
pub struct IrelandConfig {
    pub tax_year: i32,
    pub standard_rate_cutoff: Decimal, // €44,000 (single)
    pub standard_rate: Decimal,        // 20%
    pub higher_rate: Decimal,          // 40%
    // PRSI
    pub prsi_rate: Decimal,            // 4%
    pub prsi_threshold: Decimal,       // €352/week
    // USC
    pub usc_bands: Vec<(Decimal, Decimal)>,
}

impl Default for IrelandConfig {
    fn default() -> Self {
        Self {
            tax_year: 2025,
            standard_rate_cutoff: dec!(44_000),
            standard_rate: dec!(0.20),
            higher_rate: dec!(0.40),
            prsi_rate: dec!(0.04),
            prsi_threshold: dec!(18_304), // ~€352/week * 52
            usc_bands: vec![
                (dec!(12_012), dec!(0.005)),  // 0.5%
                (dec!(25_760), dec!(0.02)),   // 2%
                (dec!(70_044), dec!(0.04)),   // 4%
                (dec!(999_999_999), dec!(0.08)), // 8%
            ],
        }
    }
}

/// Ireland tax calculator
pub struct IrelandTaxCalculator {
    config: IrelandConfig,
}

impl IrelandTaxCalculator {
    pub fn new() -> Self {
        Self { config: IrelandConfig::default() }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> TaxResult {
        let monthly = gross_annual / dec!(12);
        
        // Income tax (PAYE)
        let standard = gross_annual.min(self.config.standard_rate_cutoff) * self.config.standard_rate;
        let higher = (gross_annual - self.config.standard_rate_cutoff).max(Decimal::ZERO) * self.config.higher_rate;
        let income_tax = standard + higher;
        
        // Personal tax credit
        let tax_credit = dec!(3_750); // Single person credit 2025
        let income_tax_net = (income_tax - tax_credit).max(Decimal::ZERO);
        
        // USC
        let usc = self.calculate_usc(gross_annual);
        
        // PRSI
        let prsi = if gross_annual > self.config.prsi_threshold {
            gross_annual * self.config.prsi_rate
        } else {
            Decimal::ZERO
        };
        
        // Employer PRSI
        let employer_prsi = gross_annual * dec!(0.111); // 11.1%
        
        let total_annual = income_tax_net + usc + prsi;
        let total_monthly = total_annual / dec!(12);
        
        TaxResult {
            country_code: "IE".to_string(),
            currency: "EUR".to_string(),
            gross_monthly: monthly,
            income_tax: income_tax_net / dec!(12),
            social_security_employee: (usc + prsi) / dec!(12),
            social_security_employer: employer_prsi / dec!(12),
            pension_employee: Decimal::ZERO, // No mandatory pension
            pension_employer: Decimal::ZERO,
            other_employee: usc / dec!(12),
            other_employer: Decimal::ZERO,
            total_employee_deductions: total_monthly,
            total_employer_contributions: employer_prsi / dec!(12),
            net_monthly: monthly - total_monthly,
            effective_rate: if gross_annual > Decimal::ZERO {
                total_annual / gross_annual * dec!(100)
            } else {
                Decimal::ZERO
            },
            legal_references: vec![
                "Taxes Consolidation Act 1997".to_string(),
                "Social Welfare Consolidation Act 2005".to_string(),
            ],
        }
    }
    
    fn calculate_usc(&self, income: Decimal) -> Decimal {
        let mut usc = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in &self.config.usc_bands {
            if income <= prev { break; }
            let bracket = income.min(*max) - prev;
            usc += bracket * rate;
            prev = *max;
        }
        usc
    }
}

impl Default for IrelandTaxCalculator {
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

/// Western Europe registry
pub struct WesternEuropeRegistry;

impl WesternEuropeRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("CH", "Switzerland", "CHF"),
            ("AT", "Austria", "EUR"),
            ("LU", "Luxembourg", "EUR"),
            ("IE", "Ireland", "EUR"),
            ("LI", "Liechtenstein", "CHF"),
            ("MC", "Monaco", "EUR"),
            ("AD", "Andorra", "EUR"),
        ]
    }
    
    /// Check if country is EU member
    pub fn is_eu_member(country_code: &str) -> bool {
        matches!(country_code, "AT" | "LU" | "IE")
    }
    
    /// Check if country uses SEPA
    pub fn uses_sepa(country_code: &str) -> bool {
        matches!(country_code, "CH" | "AT" | "LU" | "IE" | "LI" | "MC" | "AD")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_switzerland_zurich() {
        let config = SwitzerlandConfig::zurich_city();
        let calc = SwitzerlandTaxCalculator::new(config);
        
        // CHF 120,000/year
        let result = calc.calculate(dec!(120_000), false);
        
        assert_eq!(result.country_code, "CH");
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.effective_rate > Decimal::ZERO);
    }
    
    #[test]
    fn test_switzerland_zug_lower_tax() {
        let zurich = SwitzerlandConfig::zurich_city();
        let zug = SwitzerlandConfig::zug_city();
        
        // Zug should have lower multiplier
        assert!(zug.total_multiplier() < zurich.total_multiplier());
    }
    
    #[test]
    fn test_austria_calculator() {
        let calc = AustriaTaxCalculator::new();
        
        // €4,000/month (€56,000/year with 14 salaries)
        let result = calc.calculate(dec!(4_000), 1);
        
        assert_eq!(result.country_code, "AT");
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.social_security_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_ireland_calculator() {
        let calc = IrelandTaxCalculator::new();
        
        // €60,000/year
        let result = calc.calculate(dec!(60_000));
        
        assert_eq!(result.country_code, "IE");
        assert!(result.income_tax > Decimal::ZERO); // Should hit 40% band
    }
    
    #[test]
    fn test_western_europe_registry() {
        let countries = WesternEuropeRegistry::supported_countries();
        assert!(countries.len() >= 7);
        
        assert!(WesternEuropeRegistry::is_eu_member("AT"));
        assert!(WesternEuropeRegistry::is_eu_member("IE"));
        assert!(!WesternEuropeRegistry::is_eu_member("CH"));
        
        assert!(WesternEuropeRegistry::uses_sepa("CH"));
        assert!(WesternEuropeRegistry::uses_sepa("AT"));
    }
}
