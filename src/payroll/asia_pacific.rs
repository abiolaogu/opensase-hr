//! Asia Pacific Tax Engines
//! 
//! Tax calculators for South Asia, Southeast Asia, and Pacific:
//! - India (IN): New Tax Regime, PF 12%, Professional Tax
//! - Indonesia (ID): PPh 21 progressive, BPJS
//! - Vietnam (VN): Progressive 5-35%, SI/HI/UI
//! - Philippines (PH): 0-35%, SSS/PhilHealth/Pag-IBIG
//! - Thailand (TH): 0-35%, SSF
//! - Malaysia (MY): 0-30%, EPF/SOCSO
//! - Pakistan (PK): Progressive, EOBI
//! - Bangladesh (BD): Progressive, Provident Fund
//! - Sri Lanka (LK): APIT progressive

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// INDIA (IN) - New Tax Regime + PF + Professional Tax
// ═══════════════════════════════════════════════════════════════════════════

/// India Income Tax (New Tax Regime - Default from FY 2023-24)
pub struct IndiaTaxCalculator;

impl IndiaTaxCalculator {
    pub fn calculate_annual(gross_annual: Decimal) -> IndiaTaxResult {
        // Standard deduction: INR 50,000
        let standard_deduction = dec!(50000);
        let taxable = (gross_annual - standard_deduction).max(Decimal::ZERO);
        
        // New Tax Regime brackets (FY 2023-24)
        let tax = Self::calculate_slab(taxable);
        let surcharge = Self::calculate_surcharge(taxable, tax);
        let cess = (tax + surcharge) * dec!(0.04); // 4% Health & Education Cess
        
        let total_tax = tax + surcharge + cess;
        
        // PF contribution
        let pf_employee = gross_annual * dec!(0.12);
        let pf_employer = gross_annual * dec!(0.12);
        
        IndiaTaxResult {
            gross_annual,
            standard_deduction,
            taxable,
            income_tax: tax,
            surcharge,
            cess,
            total_tax,
            pf_employee,
            pf_employer,
            net_annual: gross_annual - total_tax - pf_employee,
        }
    }
    
    fn calculate_slab(taxable: Decimal) -> Decimal {
        // New regime: 0/5/10/15/20/30%
        let brackets: [(Decimal, Decimal); 6] = [
            (dec!(300000), dec!(0)),
            (dec!(600000), dec!(0.05)),
            (dec!(900000), dec!(0.10)),
            (dec!(1200000), dec!(0.15)),
            (dec!(1500000), dec!(0.20)),
            (Decimal::MAX, dec!(0.30)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if taxable <= prev { break; }
            tax += (taxable.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
    
    fn calculate_surcharge(taxable: Decimal, tax: Decimal) -> Decimal {
        if taxable > dec!(50000000) { tax * dec!(0.37) }
        else if taxable > dec!(20000000) { tax * dec!(0.25) }
        else if taxable > dec!(10000000) { tax * dec!(0.15) }
        else if taxable > dec!(5000000) { tax * dec!(0.10) }
        else { Decimal::ZERO }
    }
    
    /// Professional Tax (Maharashtra example)
    pub fn professional_tax_maharashtra(gross_monthly: Decimal) -> Decimal {
        if gross_monthly <= dec!(7500) { Decimal::ZERO }
        else if gross_monthly <= dec!(10000) { dec!(175) }
        else { dec!(200) } // Max ₹2,500/year
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndiaTaxResult {
    pub gross_annual: Decimal,
    pub standard_deduction: Decimal,
    pub taxable: Decimal,
    pub income_tax: Decimal,
    pub surcharge: Decimal,
    pub cess: Decimal,
    pub total_tax: Decimal,
    pub pf_employee: Decimal,
    pub pf_employer: Decimal,
    pub net_annual: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// INDONESIA (ID) - PPh 21 + BPJS
// ═══════════════════════════════════════════════════════════════════════════

/// Indonesia PPh 21 (Income Tax)
pub struct IndonesiaTaxCalculator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndonesiaMaritalStatus { Single, Married, MarriedSpouseWorking }

impl IndonesiaTaxCalculator {
    pub fn calculate_monthly(gross_monthly: Decimal, status: IndonesiaMaritalStatus) -> IndonesiaTaxResult {
        // PTKP (Non-Taxable Income) annual values
        let ptkp_annual = match status {
            IndonesiaMaritalStatus::Single => dec!(54000000),
            IndonesiaMaritalStatus::Married => dec!(58500000),
            IndonesiaMaritalStatus::MarriedSpouseWorking => dec!(54000000),
        };
        
        let ptkp_monthly = ptkp_annual / dec!(12);
        let taxable = (gross_monthly - ptkp_monthly).max(Decimal::ZERO);
        let tax = Self::apply_ter(taxable * dec!(12)) / dec!(12);
        
        // BPJS Ketenagakerjaan (JHT)
        let jht_ee = gross_monthly * dec!(0.02);
        let jht_er = gross_monthly * dec!(0.037);
        
        // BPJS Kesehatan
        let bpjs_kes_ee = gross_monthly * dec!(0.01);
        let bpjs_kes_er = gross_monthly * dec!(0.04);
        
        IndonesiaTaxResult {
            gaji: gross_monthly,
            ptkp: ptkp_monthly,
            pph21: tax,
            jht_employee: jht_ee,
            jht_employer: jht_er,
            bpjs_employee: bpjs_kes_ee,
            bpjs_employer: bpjs_kes_er,
            net_pay: gross_monthly - tax - jht_ee - bpjs_kes_ee,
            employer_cost: gross_monthly + jht_er + bpjs_kes_er,
        }
    }
    
    fn apply_ter(annual_taxable: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 4] = [
            (dec!(60000000), dec!(0.05)),
            (dec!(250000000), dec!(0.15)),
            (dec!(500000000), dec!(0.25)),
            (Decimal::MAX, dec!(0.35)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if annual_taxable <= prev { break; }
            tax += (annual_taxable.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndonesiaTaxResult {
    pub gaji: Decimal,
    pub ptkp: Decimal,
    pub pph21: Decimal,
    pub jht_employee: Decimal,
    pub jht_employer: Decimal,
    pub bpjs_employee: Decimal,
    pub bpjs_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// VIETNAM (VN) - Progressive 5-35%
// ═══════════════════════════════════════════════════════════════════════════

/// Vietnam Personal Income Tax (PIT)
pub struct VietnamTaxCalculator;

impl VietnamTaxCalculator {
    const SI_EE: Decimal = dec!(0.08);    // 8% social insurance
    const HI_EE: Decimal = dec!(0.015);   // 1.5% health insurance
    const UI_EE: Decimal = dec!(0.01);    // 1% unemployment
    const SI_ER: Decimal = dec!(0.175);   // 17.5% employer social
    
    pub fn calculate_monthly(gross_monthly: Decimal) -> VietnamTaxResult {
        // Personal deduction: VND 11M, dependent: VND 4.4M each
        let personal_deduction = dec!(11000000);
        let si = gross_monthly * Self::SI_EE;
        let hi = gross_monthly * Self::HI_EE;
        let ui = gross_monthly * Self::UI_EE;
        
        let taxable = (gross_monthly - personal_deduction - si - hi - ui).max(Decimal::ZERO);
        let pit = Self::calculate_progressive(taxable);
        
        VietnamTaxResult {
            luong: gross_monthly,
            personal_deduction,
            social_insurance: si,
            health_insurance: hi,
            unemployment: ui,
            pit,
            net_pay: gross_monthly - pit - si - hi - ui,
            employer_cost: gross_monthly + gross_monthly * Self::SI_ER,
        }
    }
    
    fn calculate_progressive(taxable: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 7] = [
            (dec!(5000000), dec!(0.05)),
            (dec!(10000000), dec!(0.10)),
            (dec!(18000000), dec!(0.15)),
            (dec!(32000000), dec!(0.20)),
            (dec!(52000000), dec!(0.25)),
            (dec!(80000000), dec!(0.30)),
            (Decimal::MAX, dec!(0.35)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if taxable <= prev { break; }
            tax += (taxable.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VietnamTaxResult {
    pub luong: Decimal,
    pub personal_deduction: Decimal,
    pub social_insurance: Decimal,
    pub health_insurance: Decimal,
    pub unemployment: Decimal,
    pub pit: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// PHILIPPINES (PH) - 0-35% + SSS/PhilHealth/Pag-IBIG
// ═══════════════════════════════════════════════════════════════════════════

/// Philippines Income Tax
pub struct PhilippinesTaxCalculator;

impl PhilippinesTaxCalculator {
    pub fn calculate_monthly(gross_monthly: Decimal) -> PhilippinesTaxResult {
        let annual = gross_monthly * dec!(12);
        let tax = Self::calculate_annual(annual) / dec!(12);
        
        // SSS (Social Security) - simplified
        let sss = (gross_monthly * dec!(0.045)).min(dec!(1350));
        // PhilHealth
        let philhealth = (gross_monthly * dec!(0.025)).min(dec!(1800));
        // Pag-IBIG
        let pagibig = dec!(100);
        
        PhilippinesTaxResult {
            sahod: gross_monthly,
            income_tax: tax,
            sss,
            philhealth,
            pagibig,
            net_pay: gross_monthly - tax - sss - philhealth - pagibig,
        }
    }
    
    fn calculate_annual(annual: Decimal) -> Decimal {
        // TRAIN Law brackets
        if annual <= dec!(250000) { Decimal::ZERO }
        else if annual <= dec!(400000) { (annual - dec!(250000)) * dec!(0.15) }
        else if annual <= dec!(800000) { dec!(22500) + (annual - dec!(400000)) * dec!(0.20) }
        else if annual <= dec!(2000000) { dec!(102500) + (annual - dec!(800000)) * dec!(0.25) }
        else if annual <= dec!(8000000) { dec!(402500) + (annual - dec!(2000000)) * dec!(0.30) }
        else { dec!(2202500) + (annual - dec!(8000000)) * dec!(0.35) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilippinesTaxResult {
    pub sahod: Decimal,
    pub income_tax: Decimal,
    pub sss: Decimal,
    pub philhealth: Decimal,
    pub pagibig: Decimal,
    pub net_pay: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// THAILAND (TH) - 0-35%
// ═══════════════════════════════════════════════════════════════════════════

/// Thailand Personal Income Tax
pub struct ThailandTaxCalculator;

impl ThailandTaxCalculator {
    const SSF_EE: Decimal = dec!(0.05);  // 5% SSF (employee)
    const SSF_ER: Decimal = dec!(0.05);  // 5% SSF (employer)
    const SSF_CAP: Decimal = dec!(750);  // Monthly cap
    
    pub fn calculate_monthly(gross_monthly: Decimal) -> ThailandTaxResult {
        let annual = gross_monthly * dec!(12);
        let tax = Self::calculate_annual(annual) / dec!(12);
        
        let ssf_ee = (gross_monthly * Self::SSF_EE).min(Self::SSF_CAP);
        let ssf_er = (gross_monthly * Self::SSF_ER).min(Self::SSF_CAP);
        
        ThailandTaxResult {
            ngoen_duan: gross_monthly,
            income_tax: tax,
            ssf_employee: ssf_ee,
            ssf_employer: ssf_er,
            net_pay: gross_monthly - tax - ssf_ee,
            employer_cost: gross_monthly + ssf_er,
        }
    }
    
    fn calculate_annual(annual: Decimal) -> Decimal {
        // After personal allowance (60K) and expense deduction (100K)
        let taxable = (annual - dec!(160000)).max(Decimal::ZERO);
        
        if taxable <= dec!(150000) { Decimal::ZERO }
        else if taxable <= dec!(300000) { (taxable - dec!(150000)) * dec!(0.05) }
        else if taxable <= dec!(500000) { dec!(7500) + (taxable - dec!(300000)) * dec!(0.10) }
        else if taxable <= dec!(750000) { dec!(27500) + (taxable - dec!(500000)) * dec!(0.15) }
        else if taxable <= dec!(1000000) { dec!(65000) + (taxable - dec!(750000)) * dec!(0.20) }
        else if taxable <= dec!(2000000) { dec!(115000) + (taxable - dec!(1000000)) * dec!(0.25) }
        else if taxable <= dec!(5000000) { dec!(365000) + (taxable - dec!(2000000)) * dec!(0.30) }
        else { dec!(1265000) + (taxable - dec!(5000000)) * dec!(0.35) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThailandTaxResult {
    pub ngoen_duan: Decimal,  // เงินเดือน (salary)
    pub income_tax: Decimal,
    pub ssf_employee: Decimal,
    pub ssf_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// MALAYSIA (MY) - 0-30% + EPF/SOCSO
// ═══════════════════════════════════════════════════════════════════════════

/// Malaysia Income Tax + EPF
pub struct MalaysiaTaxCalculator;

impl MalaysiaTaxCalculator {
    const EPF_EE: Decimal = dec!(0.11);   // 11% EPF (employee)
    const EPF_ER: Decimal = dec!(0.12);   // 12% EPF (employer)
    const SOCSO_EE: Decimal = dec!(0.005); // 0.5% SOCSO
    const EIS_EE: Decimal = dec!(0.002);   // 0.2% EIS
    
    pub fn calculate_monthly(gross_monthly: Decimal) -> MalaysiaTaxResult {
        let annual = gross_monthly * dec!(12);
        let tax = Self::calculate_annual(annual) / dec!(12);
        
        let epf_ee = gross_monthly * Self::EPF_EE;
        let epf_er = gross_monthly * Self::EPF_ER;
        let socso = gross_monthly * Self::SOCSO_EE;
        let eis = gross_monthly * Self::EIS_EE;
        
        MalaysiaTaxResult {
            gaji: gross_monthly,
            pcb: tax,           // Potongan Cukai Bulanan
            epf_employee: epf_ee,
            epf_employer: epf_er,
            socso,
            eis,
            net_pay: gross_monthly - tax - epf_ee - socso - eis,
            employer_cost: gross_monthly + epf_er,
        }
    }
    
    fn calculate_annual(annual: Decimal) -> Decimal {
        // After RM9,000 personal relief
        let taxable = (annual - dec!(9000)).max(Decimal::ZERO);
        
        if taxable <= dec!(5000) { Decimal::ZERO }
        else if taxable <= dec!(20000) { (taxable - dec!(5000)) * dec!(0.01) }
        else if taxable <= dec!(35000) { dec!(150) + (taxable - dec!(20000)) * dec!(0.03) }
        else if taxable <= dec!(50000) { dec!(600) + (taxable - dec!(35000)) * dec!(0.06) }
        else if taxable <= dec!(70000) { dec!(1500) + (taxable - dec!(50000)) * dec!(0.11) }
        else if taxable <= dec!(100000) { dec!(3700) + (taxable - dec!(70000)) * dec!(0.19) }
        else if taxable <= dec!(400000) { dec!(9400) + (taxable - dec!(100000)) * dec!(0.25) }
        else if taxable <= dec!(600000) { dec!(84400) + (taxable - dec!(400000)) * dec!(0.26) }
        else if taxable <= dec!(2000000) { dec!(136400) + (taxable - dec!(600000)) * dec!(0.28) }
        else { dec!(528400) + (taxable - dec!(2000000)) * dec!(0.30) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalaysiaTaxResult {
    pub gaji: Decimal,
    pub pcb: Decimal,
    pub epf_employee: Decimal,
    pub epf_employer: Decimal,
    pub socso: Decimal,
    pub eis: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// PAKISTAN (PK) - Progressive
// ═══════════════════════════════════════════════════════════════════════════

/// Pakistan Income Tax
pub struct PakistanTaxCalculator;

impl PakistanTaxCalculator {
    const EOBI_EE: Decimal = dec!(0.01);  // 1% EOBI (employee)
    const EOBI_ER: Decimal = dec!(0.05);  // 5% EOBI (employer)
    
    pub fn calculate_monthly(gross_monthly: Decimal) -> PakistanTaxResult {
        let annual = gross_monthly * dec!(12);
        let tax = Self::calculate_annual(annual) / dec!(12);
        let eobi_ee = gross_monthly * Self::EOBI_EE;
        let eobi_er = gross_monthly * Self::EOBI_ER;
        
        PakistanTaxResult {
            tankhuah: gross_monthly,
            income_tax: tax,
            eobi_employee: eobi_ee,
            eobi_employer: eobi_er,
            net_pay: gross_monthly - tax - eobi_ee,
            employer_cost: gross_monthly + eobi_er,
        }
    }
    
    fn calculate_annual(annual: Decimal) -> Decimal {
        if annual <= dec!(600000) { Decimal::ZERO }
        else if annual <= dec!(1200000) { (annual - dec!(600000)) * dec!(0.05) }
        else if annual <= dec!(2400000) { dec!(30000) + (annual - dec!(1200000)) * dec!(0.15) }
        else if annual <= dec!(3600000) { dec!(210000) + (annual - dec!(2400000)) * dec!(0.25) }
        else if annual <= dec!(6000000) { dec!(510000) + (annual - dec!(3600000)) * dec!(0.30) }
        else { dec!(1230000) + (annual - dec!(6000000)) * dec!(0.35) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PakistanTaxResult {
    pub tankhuah: Decimal,
    pub income_tax: Decimal,
    pub eobi_employee: Decimal,
    pub eobi_employer: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// BANGLADESH (BD) - Progressive
// ═══════════════════════════════════════════════════════════════════════════

/// Bangladesh Income Tax
pub struct BangladeshTaxCalculator;

impl BangladeshTaxCalculator {
    pub fn calculate_monthly(gross_monthly: Decimal) -> BangladeshTaxResult {
        let annual = gross_monthly * dec!(12);
        let tax = Self::calculate_annual(annual) / dec!(12);
        
        // Provident fund (if applicable)
        let pf = gross_monthly * dec!(0.10);
        
        BangladeshTaxResult {
            beton: gross_monthly,
            income_tax: tax,
            provident_fund: pf,
            net_pay: gross_monthly - tax - pf,
        }
    }
    
    fn calculate_annual(annual: Decimal) -> Decimal {
        // Tax-free: BDT 350,000
        let taxable = (annual - dec!(350000)).max(Decimal::ZERO);
        
        if taxable <= dec!(100000) { taxable * dec!(0.05) }
        else if taxable <= dec!(400000) { dec!(5000) + (taxable - dec!(100000)) * dec!(0.10) }
        else if taxable <= dec!(700000) { dec!(35000) + (taxable - dec!(400000)) * dec!(0.15) }
        else if taxable <= dec!(1100000) { dec!(80000) + (taxable - dec!(700000)) * dec!(0.20) }
        else { dec!(160000) + (taxable - dec!(1100000)) * dec!(0.25) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BangladeshTaxResult {
    pub beton: Decimal,
    pub income_tax: Decimal,
    pub provident_fund: Decimal,
    pub net_pay: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

pub struct AsiaPacificRegistry;

impl AsiaPacificRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("IN", "India", "INR"),
            ("ID", "Indonesia", "IDR"),
            ("VN", "Vietnam", "VND"),
            ("PH", "Philippines", "PHP"),
            ("TH", "Thailand", "THB"),
            ("MY", "Malaysia", "MYR"),
            ("PK", "Pakistan", "PKR"),
            ("BD", "Bangladesh", "BDT"),
        ]
    }
    
    pub fn has_mandatory_pension(code: &str) -> bool {
        matches!(code, "IN" | "ID" | "PH" | "MY" | "TH")
    }
    
    pub fn max_tax_rate(code: &str) -> Option<Decimal> {
        match code {
            "IN" => Some(dec!(0.30)),
            "ID" | "VN" | "TH" => Some(dec!(0.35)),
            "PH" => Some(dec!(0.35)),
            "MY" => Some(dec!(0.30)),
            "PK" | "BD" => Some(dec!(0.35)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_india() {
        let result = IndiaTaxCalculator::calculate_annual(dec!(1200000));
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.pf_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_indonesia() {
        let result = IndonesiaTaxCalculator::calculate_monthly(dec!(15000000), IndonesiaMaritalStatus::Single);
        assert!(result.pph21 >= Decimal::ZERO);
        assert!(result.jht_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_vietnam() {
        let result = VietnamTaxCalculator::calculate_monthly(dec!(30000000));
        assert!(result.pit >= Decimal::ZERO);
        assert!(result.social_insurance > Decimal::ZERO);
    }
    
    #[test]
    fn test_philippines() {
        let result = PhilippinesTaxCalculator::calculate_monthly(dec!(50000));
        assert!(result.income_tax >= Decimal::ZERO);
        assert!(result.sss > Decimal::ZERO);
    }
    
    #[test]
    fn test_thailand() {
        let result = ThailandTaxCalculator::calculate_monthly(dec!(80000));
        assert!(result.ssf_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_malaysia() {
        let result = MalaysiaTaxCalculator::calculate_monthly(dec!(8000));
        assert!(result.epf_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_pakistan() {
        let result = PakistanTaxCalculator::calculate_monthly(dec!(150000));
        assert!(result.eobi_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_bangladesh() {
        let result = BangladeshTaxCalculator::calculate_monthly(dec!(100000));
        assert!(result.provident_fund > Decimal::ZERO);
    }
    
    #[test]
    fn test_registry() {
        assert_eq!(AsiaPacificRegistry::supported_countries().len(), 8);
        assert!(AsiaPacificRegistry::has_mandatory_pension("IN"));
    }
}
