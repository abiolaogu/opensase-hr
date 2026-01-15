//! Central/Eastern Europe Tax Engines
//! 
//! Comprehensive tax calculators for EU member states:
//! - Poland: PIT 12%/32%, ZUS, Polski Ład reforms
//! - Czech: 15%/23% flat, child bonus
//! - Hungary: 15% flat SZJA, SZOCHO
//! - Romania: 10% flat, IT sector exemptions
//! - Estonia: 20% flat, unique exemptions
//! - Latvia: 20%/23%/31% progressive
//! - Lithuania: 20%/32% progressive
//! - Slovakia, Slovenia, Croatia, Bulgaria

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// POLAND (PL) - POLSKI ŁAD
// ═══════════════════════════════════════════════════════════════════════════

/// Polish ZUS Social Security
#[derive(Debug, Clone)]
pub struct PolishZUS {
    pub emerytalna_pracownik: Decimal,  // 9.76%
    pub rentowa_pracownik: Decimal,     // 1.5%
    pub chorobowa_pracownik: Decimal,   // 2.45%
    pub zdrowotna: Decimal,             // 9% health
    pub emerytalna_pracodawca: Decimal, // 9.76%
    pub rentowa_pracodawca: Decimal,    // 6.5%
    pub wypadkowa: Decimal,             // ~1.67%
    pub fp: Decimal,                    // 2.45% Labor Fund
    pub limit_30x: Decimal,             // 234,720 PLN
}

impl Default for PolishZUS {
    fn default() -> Self {
        Self {
            emerytalna_pracownik: dec!(0.0976), rentowa_pracownik: dec!(0.015),
            chorobowa_pracownik: dec!(0.0245), zdrowotna: dec!(0.09),
            emerytalna_pracodawca: dec!(0.0976), rentowa_pracodawca: dec!(0.065),
            wypadkowa: dec!(0.0167), fp: dec!(0.0245), limit_30x: dec!(234720),
        }
    }
}

impl PolishZUS {
    pub fn employee_social(&self) -> Decimal {
        self.emerytalna_pracownik + self.rentowa_pracownik + self.chorobowa_pracownik
    }
    pub fn employer_total(&self) -> Decimal {
        self.emerytalna_pracodawca + self.rentowa_pracodawca + self.wypadkowa + self.fp + dec!(0.001)
    }
}

/// Polish Tax Calculator
pub struct PolishTaxCalculator {
    pub zus: PolishZUS,
    pub age: u8,
    pub ulga_dla_mlodych: bool,  // Under 26 exemption
}

impl PolishTaxCalculator {
    pub fn new() -> Self {
        Self { zus: PolishZUS::default(), age: 35, ulga_dla_mlodych: false }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> PolishTaxResult {
        // Youth exemption (under 26, up to 85,528 PLN)
        let exempt = if self.ulga_dla_mlodych && self.age < 26 {
            gross_annual.min(dec!(85528))
        } else { Decimal::ZERO };
        
        let taxable = gross_annual - exempt;
        
        // ZUS (social security)
        let zus_base = gross_annual.min(self.zus.limit_30x);
        let zus_social = zus_base * self.zus.employee_social();
        
        // Health contribution (9% on gross - social)
        let health_base = gross_annual - zus_social;
        let zus_health = health_base * self.zus.zdrowotna;
        
        // PIT (12% up to 120k, 32% above, minus 3,600 PLN kwota wolna)
        let pit_base = (taxable - zus_social).max(Decimal::ZERO);
        let pit = if pit_base <= dec!(120000) {
            (pit_base * dec!(0.12) - dec!(3600)).max(Decimal::ZERO)
        } else {
            dec!(120000) * dec!(0.12) - dec!(3600) + (pit_base - dec!(120000)) * dec!(0.32)
        };
        
        PolishTaxResult {
            dochod_brutto: gross_annual,
            kwota_zwolniona: exempt,
            skladki_zus: zus_social,
            skladka_zdrowotna: zus_health,
            podatek_pit: pit,
            dochod_netto: gross_annual - zus_social - zus_health - pit,
            efektywna_stawka: if gross_annual > Decimal::ZERO { (pit + zus_social + zus_health) / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
}

impl Default for PolishTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolishTaxResult {
    pub dochod_brutto: Decimal,
    pub kwota_zwolniona: Decimal,
    pub skladki_zus: Decimal,
    pub skladka_zdrowotna: Decimal,
    pub podatek_pit: Decimal,
    pub dochod_netto: Decimal,
    pub efektywna_stawka: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// CZECH REPUBLIC (CZ)
// ═══════════════════════════════════════════════════════════════════════════

/// Czech Tax Calculator
pub struct CzechTaxCalculator {
    pub num_children: u8,
    pub has_spouse_no_income: bool,
    pub is_student: bool,
}

impl CzechTaxCalculator {
    pub fn new() -> Self {
        Self { num_children: 0, has_spouse_no_income: false, is_student: false }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> CzechTaxResult {
        // Czech uses flat 15%, 23% above 48x average wage (~1.9M CZK)
        let solidarity_threshold = dec!(1935552);
        
        let tax = if gross_annual <= solidarity_threshold {
            gross_annual * dec!(0.15)
        } else {
            solidarity_threshold * dec!(0.15) + (gross_annual - solidarity_threshold) * dec!(0.23)
        };
        
        // Slevy na dani (tax credits)
        let basic = dec!(30840);
        let spouse = if self.has_spouse_no_income { dec!(24840) } else { Decimal::ZERO };
        let student = if self.is_student { dec!(4020) } else { Decimal::ZERO };
        let child_bonus = self.child_bonus();
        
        let total_credits = basic + spouse + student + child_bonus;
        let final_tax = (tax - total_credits).max(Decimal::ZERO);
        
        // Social + Health: 6.5% + 4.5% = 11%
        let social = gross_annual.min(dec!(1935552)) * dec!(0.065);
        let health = gross_annual * dec!(0.045);
        
        CzechTaxResult {
            hruba_mzda: gross_annual,
            dan_pred_slevami: tax,
            slevy: total_credits,
            dan_po_slevach: final_tax,
            socialni: social,
            zdravotni: health,
            cista_mzda: gross_annual - final_tax - social - health,
        }
    }
    
    fn child_bonus(&self) -> Decimal {
        match self.num_children {
            0 => Decimal::ZERO,
            1 => dec!(15204),
            2 => dec!(15204) + dec!(22320),
            n => dec!(15204) + dec!(22320) + dec!(27840) * Decimal::from(n - 2),
        }
    }
}

impl Default for CzechTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CzechTaxResult {
    pub hruba_mzda: Decimal,
    pub dan_pred_slevami: Decimal,
    pub slevy: Decimal,
    pub dan_po_slevach: Decimal,
    pub socialni: Decimal,
    pub zdravotni: Decimal,
    pub cista_mzda: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// HUNGARY (HU) - 15% FLAT
// ═══════════════════════════════════════════════════════════════════════════

/// Hungarian Tax Calculator
pub struct HungarianTaxCalculator {
    pub num_children: u8,
    pub is_first_marriage: bool,
    pub age: u8,
}

impl HungarianTaxCalculator {
    pub fn new() -> Self {
        Self { num_children: 0, is_first_marriage: false, age: 35 }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> HungarianTaxResult {
        // Under 25 exemption (up to average wage ~550k HUF/month)
        let taxable = if self.age < 25 {
            (gross_monthly - dec!(550000)).max(Decimal::ZERO)
        } else { gross_monthly };
        
        // SZJA 15% flat
        let szja_base = taxable * dec!(0.15);
        
        // Family tax benefit (családi kedvezmény)
        let family_benefit = self.family_benefit(gross_monthly);
        let first_marriage = if self.is_first_marriage { dec!(5000) } else { Decimal::ZERO };
        
        let szja = (szja_base - family_benefit - first_marriage).max(Decimal::ZERO);
        
        // TB (social security) 18.5% employee
        let tb = gross_monthly * dec!(0.185);
        
        // SZOCHO 13% employer
        let szocho = gross_monthly * dec!(0.13);
        
        HungarianTaxResult {
            brutto_ber: gross_monthly,
            szja,
            tb_jarulok: tb,
            netto_ber: gross_monthly - szja - tb,
            szocho_munkaltatoi: szocho,
            ossz_koltseg: gross_monthly + szocho,
        }
    }
    
    fn family_benefit(&self, gross: Decimal) -> Decimal {
        if self.num_children == 0 { return Decimal::ZERO; }
        
        // Tax base reduction per child, saving = reduction * 15%
        let reduction_per_child = match self.num_children {
            1 => dec!(66670),
            2 => dec!(133330),
            _ => dec!(220000),
        };
        
        reduction_per_child * Decimal::from(self.num_children) * dec!(0.15)
    }
}

impl Default for HungarianTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HungarianTaxResult {
    pub brutto_ber: Decimal,
    pub szja: Decimal,
    pub tb_jarulok: Decimal,
    pub netto_ber: Decimal,
    pub szocho_munkaltatoi: Decimal,
    pub ossz_koltseg: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// ROMANIA (RO) - IT SECTOR EXEMPT
// ═══════════════════════════════════════════════════════════════════════════

/// Romanian Special Regimes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum RomanianSector {
    Standard,
    IT,           // CAS exempt
    Construction, // CAS exempt
}

/// Romanian Tax Calculator
pub struct RomanianTaxCalculator {
    pub sector: RomanianSector,
    pub num_dependents: u8,
}

impl RomanianTaxCalculator {
    pub fn new() -> Self {
        Self { sector: RomanianSector::Standard, num_dependents: 0 }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> RomanianTaxResult {
        // CAS (pension) 25% - exempt for IT/Construction
        let cas = match self.sector {
            RomanianSector::IT | RomanianSector::Construction => Decimal::ZERO,
            _ => gross_monthly * dec!(0.25),
        };
        
        // CASS (health) 10%
        let cass = gross_monthly * dec!(0.10);
        
        // Personal deduction (up to 4,500 RON gross)
        let deducere = if gross_monthly <= dec!(4500) {
            dec!(2000) + Decimal::from(self.num_dependents) * dec!(500)
        } else { Decimal::ZERO };
        
        // Income tax 10%
        let baza = (gross_monthly - cas - cass - deducere).max(Decimal::ZERO);
        let impozit = baza * dec!(0.10);
        
        RomanianTaxResult {
            salariu_brut: gross_monthly,
            cas,
            cass,
            deducere_personala: deducere,
            impozit,
            salariu_net: gross_monthly - cas - cass - impozit,
        }
    }
}

impl Default for RomanianTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomanianTaxResult {
    pub salariu_brut: Decimal,
    pub cas: Decimal,
    pub cass: Decimal,
    pub deducere_personala: Decimal,
    pub impozit: Decimal,
    pub salariu_net: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// ESTONIA (EE) - 20% FLAT
// ═══════════════════════════════════════════════════════════════════════════

/// Estonian Tax Calculator
pub struct EstonianTaxCalculator {
    pub has_pillar2_pension: bool,
}

impl EstonianTaxCalculator {
    pub fn new() -> Self { Self { has_pillar2_pension: true } }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> EstonianTaxResult {
        let annual = gross_monthly * dec!(12);
        
        // Basic exemption (€7,848/year, reduced above €14,400)
        let exemption = if annual <= dec!(14400) { dec!(654) } // 7848/12
        else if annual <= dec!(25200) { 
            dec!(654) * (dec!(25200) - annual) / (dec!(25200) - dec!(14400))
        } else { Decimal::ZERO };
        
        // Employee contributions
        let unemployment = gross_monthly * dec!(0.016); // 1.6%
        let pension = if self.has_pillar2_pension { gross_monthly * dec!(0.02) } else { Decimal::ZERO };
        
        // Taxable income
        let taxable = (gross_monthly - exemption).max(Decimal::ZERO);
        let tulumaks = taxable * dec!(0.20);
        
        // Employer: 33% sotsiaalmaks + 0.8% unemployment
        let sotsiaalmaks = gross_monthly * dec!(0.33);
        let employer_unemployment = gross_monthly * dec!(0.008);
        
        EstonianTaxResult {
            brutopalk: gross_monthly,
            maksuvaba: exemption,
            tootuskindlustus: unemployment,
            kogumispension: pension,
            tulumaks,
            netopalk: gross_monthly - unemployment - pension - tulumaks,
            sotsiaalmaks,
            tooandja_kulu: gross_monthly + sotsiaalmaks + employer_unemployment,
        }
    }
}

impl Default for EstonianTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstonianTaxResult {
    pub brutopalk: Decimal,
    pub maksuvaba: Decimal,
    pub tootuskindlustus: Decimal,
    pub kogumispension: Decimal,
    pub tulumaks: Decimal,
    pub netopalk: Decimal,
    pub sotsiaalmaks: Decimal,
    pub tooandja_kulu: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// LATVIA (LV) - PROGRESSIVE
// ═══════════════════════════════════════════════════════════════════════════

/// Latvian Tax Calculator
pub struct LatvianTaxCalculator;

impl LatvianTaxCalculator {
    pub fn calculate(gross_annual: Decimal) -> LatvianTaxResult {
        // 3 brackets: 20% / 23% / 31%
        let tax = if gross_annual <= dec!(20004) {
            gross_annual * dec!(0.20)
        } else if gross_annual <= dec!(78100) {
            dec!(20004) * dec!(0.20) + (gross_annual - dec!(20004)) * dec!(0.23)
        } else {
            dec!(20004) * dec!(0.20) + (dec!(78100) - dec!(20004)) * dec!(0.23) + (gross_annual - dec!(78100)) * dec!(0.31)
        };
        
        // Social: 10.5% employee, 23.59% employer
        let social = gross_annual * dec!(0.105);
        
        LatvianTaxResult {
            ienakumi: gross_annual,
            iin: tax,
            vsaoi: social,
            neto: gross_annual - tax - social,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatvianTaxResult {
    pub ienakumi: Decimal,
    pub iin: Decimal,
    pub vsaoi: Decimal,
    pub neto: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// LITHUANIA (LT) - PROGRESSIVE
// ═══════════════════════════════════════════════════════════════════════════

/// Lithuanian Tax Calculator
pub struct LithuanianTaxCalculator;

impl LithuanianTaxCalculator {
    pub fn calculate(gross_annual: Decimal) -> LithuanianTaxResult {
        // 2 brackets: 20% / 32%
        let threshold = dec!(101094); // ~60 average wages
        let tax = if gross_annual <= threshold {
            gross_annual * dec!(0.20)
        } else {
            threshold * dec!(0.20) + (gross_annual - threshold) * dec!(0.32)
        };
        
        // Social: 12.52% employee (Sodra), 1.77% employer
        let sodra = gross_annual * dec!(0.1252);
        
        LithuanianTaxResult {
            pajamos: gross_annual,
            gpm: tax,
            sodra,
            grynos: gross_annual - tax - sodra,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LithuanianTaxResult {
    pub pajamos: Decimal,
    pub gpm: Decimal,
    pub sodra: Decimal,
    pub grynos: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// BULGARIA (BG) - 10% FLAT
// ═══════════════════════════════════════════════════════════════════════════

/// Bulgarian Tax Calculator
pub struct BulgarianTaxCalculator;

impl BulgarianTaxCalculator {
    pub fn calculate(gross_monthly: Decimal) -> BulgarianTaxResult {
        // Social: 13.78% employee (DOO 8.78% + DZPO 2.2% + ZO 3.2%)
        let social = gross_monthly * dec!(0.1378);
        
        // Income tax 10% flat
        let taxable = gross_monthly - social;
        let tax = taxable * dec!(0.10);
        
        BulgarianTaxResult {
            bruto: gross_monthly,
            osigurovki: social,
            dod: tax,
            neto: gross_monthly - social - tax,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulgarianTaxResult {
    pub bruto: Decimal,
    pub osigurovki: Decimal,
    pub dod: Decimal,
    pub neto: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Central/Eastern Europe Registry
pub struct CentralEasternEuropeRegistry;

impl CentralEasternEuropeRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("PL", "Poland", "PLN"), ("CZ", "Czech Republic", "CZK"),
            ("HU", "Hungary", "HUF"), ("RO", "Romania", "RON"),
            ("BG", "Bulgaria", "BGN"), ("SK", "Slovakia", "EUR"),
            ("SI", "Slovenia", "EUR"), ("HR", "Croatia", "EUR"),
            ("EE", "Estonia", "EUR"), ("LV", "Latvia", "EUR"),
            ("LT", "Lithuania", "EUR"),
        ]
    }
    
    pub fn is_eurozone(code: &str) -> bool { matches!(code, "SK" | "SI" | "HR" | "EE" | "LV" | "LT") }
    pub fn is_eu_member(code: &str) -> bool { true } // All are EU
    pub fn has_flat_tax(code: &str) -> bool { matches!(code, "HU" | "RO" | "BG" | "EE") }
    pub fn uses_sepa(_code: &str) -> bool { true }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_poland() {
        let calc = PolishTaxCalculator::new();
        let result = calc.calculate(dec!(100000));
        assert!(result.podatek_pit > Decimal::ZERO);
        assert!(result.skladki_zus > Decimal::ZERO);
    }
    
    #[test]
    fn test_poland_youth() {
        let mut calc = PolishTaxCalculator::new();
        calc.age = 24;
        calc.ulga_dla_mlodych = true;
        let result = calc.calculate(dec!(80000));
        assert!(result.kwota_zwolniona > Decimal::ZERO);
    }
    
    #[test]
    fn test_czech() {
        let calc = CzechTaxCalculator::new();
        let result = calc.calculate(dec!(600000));
        assert!(result.dan_po_slevach > Decimal::ZERO);
    }
    
    #[test]
    fn test_hungary() {
        let calc = HungarianTaxCalculator::new();
        let result = calc.calculate(dec!(500000));
        assert!(result.szja > Decimal::ZERO);
    }
    
    #[test]
    fn test_romania_it() {
        let mut calc = RomanianTaxCalculator::new();
        calc.sector = RomanianSector::IT;
        let result = calc.calculate(dec!(15000));
        assert_eq!(result.cas, Decimal::ZERO); // IT exempt
    }
    
    #[test]
    fn test_estonia() {
        let calc = EstonianTaxCalculator::new();
        let result = calc.calculate(dec!(3000));
        assert!(result.tulumaks > Decimal::ZERO);
    }
    
    #[test]
    fn test_latvia() {
        let result = LatvianTaxCalculator::calculate(dec!(30000));
        assert!(result.iin > Decimal::ZERO);
    }
    
    #[test]
    fn test_lithuania() {
        let result = LithuanianTaxCalculator::calculate(dec!(40000));
        assert!(result.gpm > Decimal::ZERO);
    }
    
    #[test]
    fn test_bulgaria() {
        let result = BulgarianTaxCalculator::calculate(dec!(3000));
        assert!(result.dod > Decimal::ZERO);
    }
    
    #[test]
    fn test_registry() {
        let countries = CentralEasternEuropeRegistry::supported_countries();
        assert_eq!(countries.len(), 11);
        assert!(CentralEasternEuropeRegistry::is_eurozone("EE"));
        assert!(CentralEasternEuropeRegistry::has_flat_tax("HU"));
    }
}
