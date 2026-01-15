//! Non-EU Eastern Europe Tax Engines
//! 
//! Tax calculators for 10 non-EU Eastern European nations:
//! - Ukraine (UA): 18% + 1.5% military, 22% ESV
//! - Moldova (MD): 12% flat, 24% CNAS
//! - Belarus (BY): 13% flat, 35% FSZN
//! - Georgia (GE): 20% flat, 2%+2%+2% pension
//! - Armenia (AM): 20% flat, social payment
//! - Azerbaijan (AZ): 14-25% progressive, DSMF
//! - Russia (RU): 13%/15% flat, 22% PFR
//! - Turkey (TR): 15-40% progressive, SGK
//! - Kosovo (XK): 0-10% progressive
//! - North Macedonia (MK): 10% flat, PIOM

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// UKRAINE (UA) - PDFO + Military Levy + ESV
// ═══════════════════════════════════════════════════════════════════════════

/// Ukraine Income Tax (PDFO) + Military Levy
pub struct UkraineTaxCalculator;

impl UkraineTaxCalculator {
    const PDFO_RATE: Decimal = dec!(0.18);       // 18% income tax
    const MILITARY_LEVY: Decimal = dec!(0.015);  // 1.5% military levy
    const ESV_EMPLOYER: Decimal = dec!(0.22);    // 22% SSC (employer only)
    
    pub fn calculate(gross_monthly: Decimal) -> UkraineTaxResult {
        let pdfo = gross_monthly * Self::PDFO_RATE;
        let military = gross_monthly * Self::MILITARY_LEVY;
        let esv_employer = gross_monthly * Self::ESV_EMPLOYER;
        
        UkraineTaxResult {
            zarplata: gross_monthly,
            pdfo,
            viyskovyi_zbir: military,
            esv_employer,
            net_pay: gross_monthly - pdfo - military,
            employer_cost: gross_monthly + esv_employer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UkraineTaxResult {
    pub zarplata: Decimal,         // зарплата (salary)
    pub pdfo: Decimal,             // ПДФО (income tax)
    pub viyskovyi_zbir: Decimal,   // військовий збір (military levy)
    pub esv_employer: Decimal,     // ЄСВ (SSC)
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// MOLDOVA (MD) - 12% Flat
// ═══════════════════════════════════════════════════════════════════════════

/// Moldova Income Tax
pub struct MoldovaTaxCalculator;

impl MoldovaTaxCalculator {
    const RATE: Decimal = dec!(0.12);        // 12% flat
    const CNAS_EE: Decimal = dec!(0.06);     // 6% social (employee)
    const CNAS_ER: Decimal = dec!(0.18);     // 18% social (employer)
    const MED_EE: Decimal = dec!(0.045);     // 4.5% medical (employee)
    
    pub fn calculate(gross_monthly: Decimal) -> MoldovaTaxResult {
        let income_tax = gross_monthly * Self::RATE;
        let cnas_ee = gross_monthly * Self::CNAS_EE;
        let cnas_er = gross_monthly * Self::CNAS_ER;
        let medical = gross_monthly * Self::MED_EE;
        
        MoldovaTaxResult {
            salariu: gross_monthly,
            impozit: income_tax,
            cnas_employee: cnas_ee,
            cnas_employer: cnas_er,
            medical,
            net_pay: gross_monthly - income_tax - cnas_ee - medical,
            employer_cost: gross_monthly + cnas_er,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoldovaTaxResult {
    pub salariu: Decimal,
    pub impozit: Decimal,
    pub cnas_employee: Decimal,
    pub cnas_employer: Decimal,
    pub medical: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// BELARUS (BY) - 13% Flat
// ═══════════════════════════════════════════════════════════════════════════

/// Belarus Income Tax
pub struct BelarusTaxCalculator;

impl BelarusTaxCalculator {
    const RATE: Decimal = dec!(0.13);        // 13% flat
    const FSZN_EE: Decimal = dec!(0.01);     // 1% pension (employee)
    const FSZN_ER: Decimal = dec!(0.34);     // 34% (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> BelarusTaxResult {
        let income_tax = gross_monthly * Self::RATE;
        let fszn_ee = gross_monthly * Self::FSZN_EE;
        let fszn_er = gross_monthly * Self::FSZN_ER;
        
        BelarusTaxResult {
            zarplata: gross_monthly,
            padatkovyi: income_tax,
            fszn_employee: fszn_ee,
            fszn_employer: fszn_er,
            net_pay: gross_monthly - income_tax - fszn_ee,
            employer_cost: gross_monthly + fszn_er,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BelarusTaxResult {
    pub zarplata: Decimal,
    pub padatkovyi: Decimal,
    pub fszn_employee: Decimal,
    pub fszn_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// GEORGIA (GE) - 20% Flat + 2%+2%+2% Pension
// ═══════════════════════════════════════════════════════════════════════════

/// Georgia Income Tax + Pension
pub struct GeorgiaTaxCalculator;

impl GeorgiaTaxCalculator {
    const RATE: Decimal = dec!(0.20);            // 20% flat
    const PENSION_EE: Decimal = dec!(0.02);      // 2% employee
    const PENSION_ER: Decimal = dec!(0.02);      // 2% employer
    const PENSION_GOV: Decimal = dec!(0.02);     // 2% government (up to GEL 24,000/year)
    
    pub fn calculate(gross_monthly: Decimal) -> GeorgiaTaxResult {
        let income_tax = gross_monthly * Self::RATE;
        let pension_ee = gross_monthly * Self::PENSION_EE;
        let pension_er = gross_monthly * Self::PENSION_ER;
        
        GeorgiaTaxResult {
            khelfasi: gross_monthly,
            income_tax,
            pension_employee: pension_ee,
            pension_employer: pension_er,
            pension_government: gross_monthly * Self::PENSION_GOV,
            net_pay: gross_monthly - income_tax - pension_ee,
            employer_cost: gross_monthly + pension_er,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeorgiaTaxResult {
    pub khelfasi: Decimal,         // ხელფასი (salary)
    pub income_tax: Decimal,
    pub pension_employee: Decimal,
    pub pension_employer: Decimal,
    pub pension_government: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// ARMENIA (AM) - 20% Flat
// ═══════════════════════════════════════════════════════════════════════════

/// Armenia Income Tax
pub struct ArmeniaTaxCalculator;

impl ArmeniaTaxCalculator {
    const RATE: Decimal = dec!(0.20);        // 20% flat (from 2023)
    const SOCIAL_EE: Decimal = dec!(0.045);  // 4.5% social (employee, capped)
    const SOCIAL_ER: Decimal = dec!(0.05);   // 5% social (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> ArmeniaTaxResult {
        let income_tax = gross_monthly * Self::RATE;
        let social_ee = gross_monthly * Self::SOCIAL_EE;
        let social_er = gross_monthly * Self::SOCIAL_ER;
        
        ArmeniaTaxResult {
            ashkhatavardz: gross_monthly,
            income_tax,
            social_employee: social_ee,
            social_employer: social_er,
            net_pay: gross_monthly - income_tax - social_ee,
            employer_cost: gross_monthly + social_er,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmeniaTaxResult {
    pub ashkhatavardz: Decimal,    // աdelays (salary)
    pub income_tax: Decimal,
    pub social_employee: Decimal,
    pub social_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// AZERBAIJAN (AZ) - Progressive 14-25%
// ═══════════════════════════════════════════════════════════════════════════

/// Azerbaijan Income Tax (progressive)
pub struct AzerbaijanTaxCalculator;

impl AzerbaijanTaxCalculator {
    const DSMF_EE: Decimal = dec!(0.03);    // 3% social (employee)
    const DSMF_ER: Decimal = dec!(0.22);    // 22% social (employer)
    const UNEMP_EE: Decimal = dec!(0.005);  // 0.5% unemployment (employee)
    const UNEMP_ER: Decimal = dec!(0.005);  // 0.5% unemployment (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> AzerbaijanTaxResult {
        // Progressive: 14% up to AZN 8,000, 25% above
        let annual = gross_monthly * dec!(12);
        let income_tax = if annual <= dec!(8000) {
            gross_monthly * dec!(0.14)
        } else {
            let base = dec!(8000) / dec!(12) * dec!(0.14);
            let excess = (gross_monthly - dec!(8000) / dec!(12)) * dec!(0.25);
            base + excess.max(Decimal::ZERO)
        };
        
        let dsmf_ee = gross_monthly * Self::DSMF_EE;
        let dsmf_er = gross_monthly * Self::DSMF_ER;
        let unemp_ee = gross_monthly * Self::UNEMP_EE;
        let unemp_er = gross_monthly * Self::UNEMP_ER;
        
        AzerbaijanTaxResult {
            maas: gross_monthly,
            income_tax,
            dsmf_employee: dsmf_ee,
            dsmf_employer: dsmf_er,
            unemployment_employee: unemp_ee,
            unemployment_employer: unemp_er,
            net_pay: gross_monthly - income_tax - dsmf_ee - unemp_ee,
            employer_cost: gross_monthly + dsmf_er + unemp_er,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzerbaijanTaxResult {
    pub maas: Decimal,
    pub income_tax: Decimal,
    pub dsmf_employee: Decimal,
    pub dsmf_employer: Decimal,
    pub unemployment_employee: Decimal,
    pub unemployment_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// RUSSIA (RU) - 13%/15% Flat
// ═══════════════════════════════════════════════════════════════════════════

/// Russia Income Tax (NDFL)
pub struct RussiaTaxCalculator;

impl RussiaTaxCalculator {
    const RATE_STANDARD: Decimal = dec!(0.13);   // 13% up to 5M RUB
    const RATE_HIGH: Decimal = dec!(0.15);       // 15% above 5M RUB
    const PFR_ER: Decimal = dec!(0.22);          // 22% pension (employer)
    const FSS_ER: Decimal = dec!(0.029);         // 2.9% social (employer)
    const FOMS_ER: Decimal = dec!(0.051);        // 5.1% medical (employer)
    
    pub fn calculate(gross_monthly: Decimal, ytd_income: Decimal) -> RussiaTaxResult {
        // 15% kicks in above 5M RUB annually
        let rate = if ytd_income + gross_monthly > dec!(5000000) {
            Self::RATE_HIGH
        } else {
            Self::RATE_STANDARD
        };
        
        let ndfl = gross_monthly * rate;
        let pfr = gross_monthly * Self::PFR_ER;
        let fss = gross_monthly * Self::FSS_ER;
        let foms = gross_monthly * Self::FOMS_ER;
        
        RussiaTaxResult {
            zarplata: gross_monthly,
            ndfl,
            pfr_employer: pfr,
            fss_employer: fss,
            foms_employer: foms,
            net_pay: gross_monthly - ndfl,
            employer_cost: gross_monthly + pfr + fss + foms,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RussiaTaxResult {
    pub zarplata: Decimal,
    pub ndfl: Decimal,              // НДФЛ
    pub pfr_employer: Decimal,      // ПФР
    pub fss_employer: Decimal,      // ФСС
    pub foms_employer: Decimal,     // ФОМС
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// TURKEY (TR) - Progressive 15-40%
// ═══════════════════════════════════════════════════════════════════════════

/// Turkey Income Tax (Gelir Vergisi)
pub struct TurkeyTaxCalculator;

impl TurkeyTaxCalculator {
    const SGK_EE: Decimal = dec!(0.14);      // 14% social (employee)
    const SGK_ER: Decimal = dec!(0.205);     // 20.5% social (employer)
    const UNEMP_EE: Decimal = dec!(0.01);    // 1% unemployment (employee)
    const UNEMP_ER: Decimal = dec!(0.02);    // 2% unemployment (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> TurkeyTaxResult {
        let annual = gross_monthly * dec!(12);
        
        // 2024 brackets (simplified to TRY)
        let income_tax = Self::calculate_progressive(annual) / dec!(12);
        
        let sgk_ee = gross_monthly * Self::SGK_EE;
        let sgk_er = gross_monthly * Self::SGK_ER;
        let unemp_ee = gross_monthly * Self::UNEMP_EE;
        let unemp_er = gross_monthly * Self::UNEMP_ER;
        
        TurkeyTaxResult {
            maas: gross_monthly,
            gelir_vergisi: income_tax,
            sgk_employee: sgk_ee,
            sgk_employer: sgk_er,
            unemployment_employee: unemp_ee,
            unemployment_employer: unemp_er,
            net_pay: gross_monthly - income_tax - sgk_ee - unemp_ee,
            employer_cost: gross_monthly + sgk_er + unemp_er,
        }
    }
    
    fn calculate_progressive(annual: Decimal) -> Decimal {
        // 2024 brackets: 15%, 20%, 27%, 35%, 40%
        let brackets: [(Decimal, Decimal); 5] = [
            (dec!(110000), dec!(0.15)),
            (dec!(230000), dec!(0.20)),
            (dec!(580000), dec!(0.27)),
            (dec!(3000000), dec!(0.35)),
            (dec!(999999999), dec!(0.40)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if annual <= prev { break; }
            tax += (annual.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurkeyTaxResult {
    pub maas: Decimal,
    pub gelir_vergisi: Decimal,    // income tax
    pub sgk_employee: Decimal,
    pub sgk_employer: Decimal,
    pub unemployment_employee: Decimal,
    pub unemployment_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// KOSOVO (XK) - Progressive 0-10%
// ═══════════════════════════════════════════════════════════════════════════

/// Kosovo Income Tax
pub struct KosovoTaxCalculator;

impl KosovoTaxCalculator {
    const TRUST_EE: Decimal = dec!(0.05);    // 5% pension (employee)
    const TRUST_ER: Decimal = dec!(0.05);    // 5% pension (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> KosovoTaxResult {
        let annual = gross_monthly * dec!(12);
        
        // Progressive: 0% up to €960, 4% €960-3000, 8% €3000-5400, 10% above
        let income_tax = Self::calculate_progressive(annual) / dec!(12);
        
        let trust_ee = gross_monthly * Self::TRUST_EE;
        let trust_er = gross_monthly * Self::TRUST_ER;
        
        KosovoTaxResult {
            rroga: gross_monthly,
            income_tax,
            trust_employee: trust_ee,
            trust_employer: trust_er,
            net_pay: gross_monthly - income_tax - trust_ee,
            employer_cost: gross_monthly + trust_er,
        }
    }
    
    fn calculate_progressive(annual: Decimal) -> Decimal {
        let bracket1_max = dec!(960);
        let bracket2_max = dec!(3000);
        let bracket3_max = dec!(5400);
        
        if annual <= bracket1_max { return Decimal::ZERO; }
        if annual <= bracket2_max { return (annual - bracket1_max) * dec!(0.04); }
        if annual <= bracket3_max { 
            let tier1 = (bracket2_max - bracket1_max) * dec!(0.04);
            return tier1 + (annual - bracket2_max) * dec!(0.08); 
        }
        let tier1 = (bracket2_max - bracket1_max) * dec!(0.04);
        let tier2 = (bracket3_max - bracket2_max) * dec!(0.08);
        tier1 + tier2 + (annual - bracket3_max) * dec!(0.10)
    }

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KosovoTaxResult {
    pub rroga: Decimal,
    pub income_tax: Decimal,
    pub trust_employee: Decimal,
    pub trust_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// NORTH MACEDONIA (MK) - 10% Flat
// ═══════════════════════════════════════════════════════════════════════════

/// North Macedonia Income Tax
pub struct NorthMacedoniaTaxCalculator;

impl NorthMacedoniaTaxCalculator {
    const RATE: Decimal = dec!(0.10);        // 10% flat
    const PIOM_PENSION_ER: Decimal = dec!(0.188);   // 18.8% pension (employer)
    const HEALTH_ER: Decimal = dec!(0.075);  // 7.5% health (employer)
    const UNEMP_ER: Decimal = dec!(0.012);   // 1.2% unemployment (employer)
    
    pub fn calculate(gross_monthly: Decimal) -> NorthMacedoniaTaxResult {
        let income_tax = gross_monthly * Self::RATE;
        let piom = gross_monthly * Self::PIOM_PENSION_ER;
        let health = gross_monthly * Self::HEALTH_ER;
        let unemp = gross_monthly * Self::UNEMP_ER;
        
        NorthMacedoniaTaxResult {
            plata: gross_monthly,
            income_tax,
            piom_employer: piom,
            health_employer: health,
            unemployment_employer: unemp,
            net_pay: gross_monthly - income_tax,
            employer_cost: gross_monthly + piom + health + unemp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NorthMacedoniaTaxResult {
    pub plata: Decimal,
    pub income_tax: Decimal,
    pub piom_employer: Decimal,
    pub health_employer: Decimal,
    pub unemployment_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Non-EU Eastern Europe Registry
pub struct EasternEuropeNonEuRegistry;

impl EasternEuropeNonEuRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("UA", "Ukraine", "UAH"),
            ("MD", "Moldova", "MDL"),
            ("BY", "Belarus", "BYN"),
            ("GE", "Georgia", "GEL"),
            ("AM", "Armenia", "AMD"),
            ("AZ", "Azerbaijan", "AZN"),
            ("RU", "Russia", "RUB"),
            ("TR", "Turkey", "TRY"),
            ("XK", "Kosovo", "EUR"),
            ("MK", "North Macedonia", "MKD"),
        ]
    }
    
    pub fn has_flat_tax(code: &str) -> bool {
        matches!(code, "UA" | "MD" | "BY" | "GE" | "AM" | "RU" | "MK")
    }
    
    pub fn flat_tax_rate(code: &str) -> Option<Decimal> {
        match code {
            "UA" => Some(dec!(0.18)),
            "MD" => Some(dec!(0.12)),
            "BY" => Some(dec!(0.13)),
            "GE" => Some(dec!(0.20)),
            "AM" => Some(dec!(0.20)),
            "RU" => Some(dec!(0.13)),
            "MK" => Some(dec!(0.10)),
            _ => None,
        }
    }
    
    pub fn has_military_levy(code: &str) -> bool { code == "UA" }
    pub fn uses_euro(code: &str) -> bool { code == "XK" }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ukraine() {
        let result = UkraineTaxCalculator::calculate(dec!(50000));
        assert_eq!(result.pdfo, dec!(9000)); // 18%
        assert_eq!(result.viyskovyi_zbir, dec!(750)); // 1.5%
    }
    
    #[test]
    fn test_georgia() {
        let result = GeorgiaTaxCalculator::calculate(dec!(5000));
        assert_eq!(result.income_tax, dec!(1000)); // 20%
        assert_eq!(result.pension_employee, dec!(100)); // 2%
    }
    
    #[test]
    fn test_russia_standard() {
        let result = RussiaTaxCalculator::calculate(dec!(100000), Decimal::ZERO);
        assert_eq!(result.ndfl, dec!(13000)); // 13%
    }
    
    #[test]
    fn test_russia_high_income() {
        let result = RussiaTaxCalculator::calculate(dec!(500000), dec!(4900000));
        assert_eq!(result.ndfl, dec!(75000)); // 15%
    }
    
    #[test]
    fn test_turkey() {
        let result = TurkeyTaxCalculator::calculate(dec!(50000));
        assert!(result.gelir_vergisi > Decimal::ZERO);
        assert!(result.sgk_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_moldova() {
        let result = MoldovaTaxCalculator::calculate(dec!(20000));
        assert_eq!(result.impozit, dec!(2400)); // 12%
    }
    
    #[test]
    fn test_belarus() {
        let result = BelarusTaxCalculator::calculate(dec!(5000));
        assert_eq!(result.padatkovyi, dec!(650)); // 13%
    }
    
    #[test]
    fn test_kosovo() {
        let result = KosovoTaxCalculator::calculate(dec!(1000));
        assert!(result.income_tax >= Decimal::ZERO);
    }
    
    #[test]
    fn test_north_macedonia() {
        let result = NorthMacedoniaTaxCalculator::calculate(dec!(50000));
        assert_eq!(result.income_tax, dec!(5000)); // 10%
    }
    
    #[test]
    fn test_registry() {
        assert_eq!(EasternEuropeNonEuRegistry::supported_countries().len(), 10);
        assert!(EasternEuropeNonEuRegistry::has_flat_tax("UA"));
        assert!(EasternEuropeNonEuRegistry::has_military_levy("UA"));
        assert_eq!(EasternEuropeNonEuRegistry::flat_tax_rate("GE"), Some(dec!(0.20)));
    }
}
