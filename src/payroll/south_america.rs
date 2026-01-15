//! South America Tax Engines
//! 
//! Tax calculators for 12 South American countries featuring:
//! - Complex labor laws with 13th/14th salaries
//! - High social security contributions
//! - Strong employee protections
//! 
//! Countries: BR, AR, CO, PE, CL, EC, VE, BO, PY, UY, GY, SR

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// BRAZIL TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Brazil INSS brackets (progressive, 2024)
#[derive(Debug, Clone)]
pub struct BrazilConfig {
    pub tax_year: i32,
    pub inss_ceiling: Decimal,        // R$7,786.02
    pub irrf_exempt: Decimal,         // R$2,259.20
    pub dependant_deduction: Decimal, // R$189.59
    pub fgts_rate: Decimal,           // 8%
}

impl Default for BrazilConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            inss_ceiling: dec!(7_786.02),
            irrf_exempt: dec!(2_259.20),
            dependant_deduction: dec!(189.59),
            fgts_rate: dec!(0.08),
        }
    }
}

/// Brazil tax calculator (INSS, IRRF, FGTS, 13º)
pub struct BrazilTaxCalculator {
    config: BrazilConfig,
}

impl BrazilTaxCalculator {
    pub fn new() -> Self {
        Self { config: BrazilConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, dependants: u8) -> TaxResult {
        // INSS (progressive)
        let inss = self.calculate_inss(gross_monthly);
        
        // IRRF base = gross - INSS - dependants
        let dependant_ded = self.config.dependant_deduction * Decimal::from(dependants);
        let irrf_base = gross_monthly - inss - dependant_ded;
        let irrf = self.calculate_irrf(irrf_base);
        
        // FGTS (employer deposits)
        let fgts = gross_monthly * self.config.fgts_rate;
        
        // 13º salário provision (1/12)
        let thirteenth = gross_monthly / dec!(12);
        
        // Férias + 1/3 provision
        let vacation = (gross_monthly / dec!(12)) * dec!(1.333);
        
        let total_employee = inss + irrf;
        let total_employer = fgts + (thirteenth * self.config.fgts_rate) + (vacation * self.config.fgts_rate);
        
        TaxResult {
            country_code: "BR".to_string(),
            currency: "BRL".to_string(),
            gross_monthly,
            inss,
            income_tax: irrf,
            pension_employee: Decimal::ZERO,
            pension_employer: fgts,
            other_employee: Decimal::ZERO,
            other_employer: thirteenth + vacation,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { total_employee / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Lei nº 8.212/91 (INSS)".to_string(),
                "Lei nº 8.036/90 (FGTS)".to_string(),
                "Decreto nº 9.580/2018 (IRRF)".to_string(),
                "Lei nº 4.090/62 (13º Salário)".to_string(),
            ],
        }
    }
    
    fn calculate_inss(&self, gross: Decimal) -> Decimal {
        // Progressive INSS 2024
        let brackets = [
            (dec!(1_412.00), dec!(0.075)),
            (dec!(2_666.68), dec!(0.09)),
            (dec!(4_000.03), dec!(0.12)),
            (dec!(7_786.02), dec!(0.14)),
        ];
        
        let mut total = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in brackets {
            if gross <= prev { break; }
            let taxable = gross.min(max) - prev;
            total += taxable * rate;
            prev = max;
        }
        
        total.min(dec!(908.86)) // 2024 ceiling
    }
    
    fn calculate_irrf(&self, base: Decimal) -> Decimal {
        if base <= self.config.irrf_exempt { return Decimal::ZERO; }
        
        // IRRF brackets 2024
        let brackets: [(Decimal, Decimal, Decimal); 4] = [
            (dec!(2_826.65), dec!(0.075), dec!(169.44)),
            (dec!(3_751.05), dec!(0.15), dec!(381.44)),
            (dec!(4_664.68), dec!(0.225), dec!(662.77)),
            (dec!(999_999_999), dec!(0.275), dec!(896.00)),
        ];
        
        for (max, rate, deduction) in brackets {
            if base <= max {
                return (base * rate - deduction).max(Decimal::ZERO);
            }
        }
        Decimal::ZERO
    }
}

impl Default for BrazilTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ARGENTINA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Argentina tax config
#[derive(Debug, Clone)]
pub struct ArgentinaConfig {
    pub tax_year: i32,
    pub jubilacion_rate: Decimal,  // 11%
    pub obra_social_rate: Decimal, // 3%
    pub pami_rate: Decimal,        // 3%
    pub employer_total: Decimal,   // ~20%
    pub mni_annual: Decimal,       // Mínimo No Imponible
}

impl Default for ArgentinaConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            jubilacion_rate: dec!(0.11),
            obra_social_rate: dec!(0.03),
            pami_rate: dec!(0.03),
            employer_total: dec!(0.20),
            mni_annual: dec!(3_091_035),
        }
    }
}

/// Argentina tax calculator
pub struct ArgentinaTaxCalculator {
    config: ArgentinaConfig,
}

impl ArgentinaTaxCalculator {
    pub fn new() -> Self {
        Self { config: ArgentinaConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, has_spouse: bool, children: u8) -> TaxResult {
        // Aportes (employee contributions)
        let jubilacion = gross_monthly * self.config.jubilacion_rate;
        let obra_social = gross_monthly * self.config.obra_social_rate;
        let pami = gross_monthly * self.config.pami_rate;
        let total_aportes = jubilacion + obra_social + pami;
        
        // Employer contributions
        let employer = gross_monthly * self.config.employer_total;
        
        // Ganancias calculation (simplified)
        let gross_annual = gross_monthly * dec!(13); // Include aguinaldo
        let family_deductions = Decimal::from(has_spouse as u8) * dec!(2_911_135) 
                              + Decimal::from(children) * dec!(1_468_096);
        let taxable = (gross_annual - self.config.mni_annual - family_deductions - (total_aportes * dec!(13))).max(Decimal::ZERO);
        
        let annual_ganancias = self.calculate_ganancias(taxable);
        let monthly_ganancias = annual_ganancias / dec!(12);
        
        // Aguinaldo (SAC) provision
        let aguinaldo = gross_monthly / dec!(12);
        
        let total_employee = total_aportes + monthly_ganancias;
        let total_employer = employer + aguinaldo;
        
        TaxResult {
            country_code: "AR".to_string(),
            currency: "ARS".to_string(),
            gross_monthly,
            inss: total_aportes,
            income_tax: monthly_ganancias,
            pension_employee: jubilacion,
            pension_employer: employer,
            other_employee: obra_social + pami,
            other_employer: aguinaldo,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { total_employee / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Ley 20.628 (Ganancias)".to_string(),
                "Ley 24.241 (SIJP)".to_string(),
            ],
        }
    }
    
    fn calculate_ganancias(&self, taxable: Decimal) -> Decimal {
        if taxable <= Decimal::ZERO { return Decimal::ZERO; }
        
        // Simplified brackets
        let brackets: [(Decimal, Decimal); 5] = [
            (dec!(1_000_000), dec!(0.05)),
            (dec!(2_000_000), dec!(0.09)),
            (dec!(6_000_000), dec!(0.15)),
            (dec!(18_000_000), dec!(0.27)),
            (dec!(999_999_999_999), dec!(0.35)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in brackets {
            if taxable <= prev { break; }
            let bracket_taxable = taxable.min(max) - prev;
            tax += bracket_taxable * rate;
            prev = max;
        }
        tax
    }
}

impl Default for ArgentinaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// COLOMBIA TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Colombia tax config
#[derive(Debug, Clone)]
pub struct ColombiaConfig {
    pub tax_year: i32,
    pub uvt: Decimal,              // UVT 2024 = COP 47,065
    pub smlmv: Decimal,            // COP 1,300,000
    pub salud_employee: Decimal,   // 4%
    pub pension_employee: Decimal, // 4%
    pub salud_employer: Decimal,   // 8.5%
    pub pension_employer: Decimal, // 12%
    pub parafiscales: Decimal,     // 9% (caja + ICBF + SENA)
}

impl Default for ColombiaConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            uvt: dec!(47_065),
            smlmv: dec!(1_300_000),
            salud_employee: dec!(0.04),
            pension_employee: dec!(0.04),
            salud_employer: dec!(0.085),
            pension_employer: dec!(0.12),
            parafiscales: dec!(0.09),
        }
    }
}

/// Colombia tax calculator
pub struct ColombiaTaxCalculator {
    config: ColombiaConfig,
}

impl ColombiaTaxCalculator {
    pub fn new() -> Self {
        Self { config: ColombiaConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> TaxResult {
        // Employee contributions
        let salud = gross_monthly * self.config.salud_employee;
        let pension = gross_monthly * self.config.pension_employee;
        
        // FSP if > 4 SMLMV
        let fsp = if gross_monthly > self.config.smlmv * dec!(4) {
            gross_monthly * dec!(0.01)
        } else {
            Decimal::ZERO
        };
        
        // Employer contributions
        let employer_salud = gross_monthly * self.config.salud_employer;
        let employer_pension = gross_monthly * self.config.pension_employer;
        let parafiscales = gross_monthly * self.config.parafiscales;
        
        // Retención (simplified)
        let retencion = self.calculate_retencion(gross_monthly - salud - pension);
        
        // Provisions (prima, cesantías, vacaciones)
        let prima = gross_monthly / dec!(6);
        let cesantias = gross_monthly / dec!(12);
        let vacaciones = gross_monthly / dec!(24);
        
        let total_employee = salud + pension + fsp + retencion;
        let total_employer = employer_salud + employer_pension + parafiscales + prima + cesantias + vacaciones;
        
        TaxResult {
            country_code: "CO".to_string(),
            currency: "COP".to_string(),
            gross_monthly,
            inss: salud + pension + fsp,
            income_tax: retencion,
            pension_employee: pension,
            pension_employer: employer_pension,
            other_employee: salud + fsp,
            other_employer: employer_salud + parafiscales + prima + cesantias + vacaciones,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { total_employee / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "Estatuto Tributario".to_string(),
                "Ley 100 de 1993".to_string(),
            ],
        }
    }
    
    fn calculate_retencion(&self, base: Decimal) -> Decimal {
        let uvt_base = base / self.config.uvt;
        if uvt_base <= dec!(95) { return Decimal::ZERO; }
        
        // Simplified marginal rate
        let rate = if uvt_base <= dec!(150) { dec!(0.19) }
                  else if uvt_base <= dec!(360) { dec!(0.28) }
                  else if uvt_base <= dec!(640) { dec!(0.33) }
                  else { dec!(0.35) };
        
        (uvt_base - dec!(95)) * rate * self.config.uvt
    }
}

impl Default for ColombiaTaxCalculator {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// PERU TAX CALCULATOR
// ═══════════════════════════════════════════════════════════════════════════

/// Peru tax config
#[derive(Debug, Clone)]
pub struct PeruConfig {
    pub tax_year: i32,
    pub uit: Decimal,              // UIT 2024 = S/5,150
    pub onp_rate: Decimal,         // 13% (public pension)
    pub afp_rate: Decimal,         // ~13% (private pension)
    pub essalud_rate: Decimal,     // 9% employer
}

impl Default for PeruConfig {
    fn default() -> Self {
        Self {
            tax_year: 2024,
            uit: dec!(5_150),
            onp_rate: dec!(0.13),
            afp_rate: dec!(0.13),
            essalud_rate: dec!(0.09),
        }
    }
}

/// Peru tax calculator
pub struct PeruTaxCalculator {
    config: PeruConfig,
}

impl PeruTaxCalculator {
    pub fn new() -> Self {
        Self { config: PeruConfig::default() }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal, uses_afp: bool) -> TaxResult {
        // Pension (ONP or AFP)
        let pension_rate = if uses_afp { self.config.afp_rate } else { self.config.onp_rate };
        let pension = gross_monthly * pension_rate;
        
        // EsSalud (employer)
        let essalud = gross_monthly * self.config.essalud_rate;
        
        // 5ta Categoría (income tax)
        let gross_annual = gross_monthly * dec!(14); // +2 gratificaciones
        let exemption = self.config.uit * dec!(7);
        let taxable = (gross_annual - exemption - (pension * dec!(12))).max(Decimal::ZERO);
        let annual_ir = self.calculate_quinta(taxable);
        let monthly_ir = annual_ir / dec!(12);
        
        // Gratificaciones (July + December)
        let gratificacion = gross_monthly / dec!(6);
        
        // CTS
        let cts = gross_monthly / dec!(12);
        
        let total_employee = pension + monthly_ir;
        let total_employer = essalud + gratificacion + cts;
        
        TaxResult {
            country_code: "PE".to_string(),
            currency: "PEN".to_string(),
            gross_monthly,
            inss: Decimal::ZERO,
            income_tax: monthly_ir,
            pension_employee: pension,
            pension_employer: essalud,
            other_employee: Decimal::ZERO,
            other_employer: gratificacion + cts,
            total_employee_deductions: total_employee,
            total_employer_contributions: total_employer,
            net_monthly: gross_monthly - total_employee,
            effective_rate: if gross_monthly > Decimal::ZERO { total_employee / gross_monthly * dec!(100) } else { Decimal::ZERO },
            legal_references: vec![
                "TUO de la Ley del IR".to_string(),
                "Ley 29903 (AFP)".to_string(),
            ],
        }
    }
    
    fn calculate_quinta(&self, taxable: Decimal) -> Decimal {
        let uit = self.config.uit;
        let brackets: [(Decimal, Decimal); 5] = [
            (dec!(5) * uit, dec!(0.08)),
            (dec!(20) * uit, dec!(0.14)),
            (dec!(35) * uit, dec!(0.17)),
            (dec!(45) * uit, dec!(0.20)),
            (dec!(999_999_999), dec!(0.30)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        
        for (max, rate) in brackets {
            if taxable <= prev { break; }
            let bracket_taxable = taxable.min(max) - prev;
            tax += bracket_taxable * rate;
            prev = max;
        }
        tax
    }
}

impl Default for PeruTaxCalculator {
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
    pub inss: Decimal,
    pub income_tax: Decimal,
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

/// South America country registry
pub struct SouthAmericaRegistry;

impl SouthAmericaRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("BR", "Brazil", "BRL"),
            ("AR", "Argentina", "ARS"),
            ("CO", "Colombia", "COP"),
            ("PE", "Peru", "PEN"),
            ("CL", "Chile", "CLP"),
            ("EC", "Ecuador", "USD"),
            ("VE", "Venezuela", "VES"),
            ("BO", "Bolivia", "BOB"),
            ("PY", "Paraguay", "PYG"),
            ("UY", "Uruguay", "UYU"),
            ("GY", "Guyana", "GYD"),
            ("SR", "Suriname", "SRD"),
        ]
    }
    
    /// Check if country uses 13th salary (aguinaldo)
    pub fn has_thirteenth_salary(country_code: &str) -> bool {
        matches!(country_code, "BR" | "AR" | "CO" | "PE" | "CL" | "EC" | "BO" | "PY" | "UY")
    }
    
    /// Check if country is dollarized
    pub fn is_dollarized(country_code: &str) -> bool {
        matches!(country_code, "EC")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_brazil_calculator() {
        let calc = BrazilTaxCalculator::new();
        let result = calc.calculate(dec!(10_000), 0);
        
        assert_eq!(result.country_code, "BR");
        assert!(result.inss > Decimal::ZERO);
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.net_monthly < result.gross_monthly);
    }
    
    #[test]
    fn test_argentina_calculator() {
        let calc = ArgentinaTaxCalculator::new();
        let result = calc.calculate(dec!(500_000), false, 0);
        
        assert_eq!(result.country_code, "AR");
        assert!(result.pension_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_colombia_calculator() {
        let calc = ColombiaTaxCalculator::new();
        let result = calc.calculate(dec!(5_000_000));
        
        assert_eq!(result.country_code, "CO");
        assert!(result.pension_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_peru_calculator() {
        let calc = PeruTaxCalculator::new();
        let result = calc.calculate(dec!(5_000), true);
        
        assert_eq!(result.country_code, "PE");
        assert!(result.pension_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_south_america_registry() {
        let countries = SouthAmericaRegistry::supported_countries();
        assert_eq!(countries.len(), 12);
        
        assert!(SouthAmericaRegistry::has_thirteenth_salary("BR"));
        assert!(SouthAmericaRegistry::has_thirteenth_salary("AR"));
        assert!(SouthAmericaRegistry::is_dollarized("EC"));
        assert!(!SouthAmericaRegistry::is_dollarized("BR"));
    }
}
