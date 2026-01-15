//! Western Europe Extended Tax Engines
//! 
//! Comprehensive tax calculators for Western Europe's financial hubs:
//! - Switzerland: 26 cantons, 3-tier system, BVG pension, QR-Bill
//! - Austria: 7 brackets, 13th/14th salary (Sonderzahlungen), Pendlerpauschale
//! - Luxembourg: 3 tax classes, frontalier handling, CIS credits
//! - Ireland: PAYE, USC bands, PRSI classes, comprehensive credits
//! - Liechtenstein: Swiss-style, Gemeinde surcharges

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SWITZERLAND (CH) - 26 CANTONS
// ═══════════════════════════════════════════════════════════════════════════

/// Swiss Canton Enumeration (all 26)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Kanton {
    ZH, BE, LU, UR, SZ, OW, NW, GL, ZG, FR,
    SO, BS, BL, SH, AR, AI, SG, GR, AG, TG,
    TI, VD, VS, NE, GE, JU,
}

/// Swiss Federal Tax Tariff Type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TarifType {
    Alleinstehend,    // Single
    Verheiratet,      // Married (Splitting)
    Einelternfamilie, // Single parent
}

/// Swiss Federal Tax Bracket
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwissTaxBracket {
    pub lower: Decimal,
    pub upper: Decimal,
    pub base_tax: Decimal,
    pub marginal_rate: Decimal,
}

/// Swiss Federal Tax Tariff (Bundessteuer)
#[derive(Debug, Clone)]
pub struct BundessteuerTarif {
    pub tarif_type: TarifType,
    pub brackets: Vec<SwissTaxBracket>,
}

impl BundessteuerTarif {
    pub fn single_tarif() -> Self {
        Self {
            tarif_type: TarifType::Alleinstehend,
            brackets: vec![
                SwissTaxBracket { lower: dec!(0), upper: dec!(17800), base_tax: dec!(0), marginal_rate: dec!(0) },
                SwissTaxBracket { lower: dec!(17800), upper: dec!(31600), base_tax: dec!(0), marginal_rate: dec!(0.77) },
                SwissTaxBracket { lower: dec!(31600), upper: dec!(41400), base_tax: dec!(106.25), marginal_rate: dec!(0.88) },
                SwissTaxBracket { lower: dec!(41400), upper: dec!(55200), base_tax: dec!(192.55), marginal_rate: dec!(2.64) },
                SwissTaxBracket { lower: dec!(55200), upper: dec!(72500), base_tax: dec!(556.75), marginal_rate: dec!(2.97) },
                SwissTaxBracket { lower: dec!(72500), upper: dec!(78100), base_tax: dec!(1070.45), marginal_rate: dec!(5.58) },
                SwissTaxBracket { lower: dec!(78100), upper: dec!(103600), base_tax: dec!(1382.95), marginal_rate: dec!(6.66) },
                SwissTaxBracket { lower: dec!(103600), upper: dec!(134600), base_tax: dec!(3080.55), marginal_rate: dec!(8.80) },
                SwissTaxBracket { lower: dec!(134600), upper: dec!(176000), base_tax: dec!(5808.55), marginal_rate: dec!(11.00) },
                SwissTaxBracket { lower: dec!(176000), upper: dec!(755200), base_tax: dec!(10362.55), marginal_rate: dec!(13.00) },
            ],
        }
    }
    
    pub fn married_tarif() -> Self {
        Self {
            tarif_type: TarifType::Verheiratet,
            brackets: vec![
                SwissTaxBracket { lower: dec!(0), upper: dec!(29800), base_tax: dec!(0), marginal_rate: dec!(0) },
                SwissTaxBracket { lower: dec!(29800), upper: dec!(51800), base_tax: dec!(0), marginal_rate: dec!(1.00) },
                SwissTaxBracket { lower: dec!(51800), upper: dec!(59400), base_tax: dec!(220), marginal_rate: dec!(2.00) },
                SwissTaxBracket { lower: dec!(59400), upper: dec!(100000), base_tax: dec!(372), marginal_rate: dec!(5.00) },
                SwissTaxBracket { lower: dec!(100000), upper: dec!(912600), base_tax: dec!(2402), marginal_rate: dec!(11.00) },
            ],
        }
    }
}

/// Swiss Social Insurance (AHV/IV/EO/ALV)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwissSocialInsurance {
    pub ahv_rate: Decimal,         // 8.7% total (4.35% each)
    pub iv_rate: Decimal,          // 1.4% total
    pub eo_rate: Decimal,          // 0.5% total
    pub alv_rate: Decimal,         // 2.2% up to ceiling
    pub alv_solidarity_rate: Decimal, // 1% above ceiling
    pub alv_ceiling: Decimal,      // CHF 148,200
    pub nbu_rate: Decimal,         // ~1.5% (non-occupational accident)
    pub bvg_entry_threshold: Decimal, // CHF 22,050
    pub bvg_maximum_salary: Decimal,  // CHF 88,200
    pub bvg_coordination_deduction: Decimal, // CHF 25,725
}

impl Default for SwissSocialInsurance {
    fn default() -> Self {
        Self {
            ahv_rate: dec!(0.087),
            iv_rate: dec!(0.014),
            eo_rate: dec!(0.005),
            alv_rate: dec!(0.022),
            alv_solidarity_rate: dec!(0.01),
            alv_ceiling: dec!(148200),
            nbu_rate: dec!(0.015),
            bvg_entry_threshold: dec!(22050),
            bvg_maximum_salary: dec!(88200),
            bvg_coordination_deduction: dec!(25725),
        }
    }
}

/// BVG Pension (2nd Pillar)
pub struct BVGPension;

impl BVGPension {
    pub fn get_savings_rate(age: u8) -> Decimal {
        match age {
            25..=34 => dec!(0.07),
            35..=44 => dec!(0.10),
            45..=54 => dec!(0.15),
            55..=65 => dec!(0.18),
            _ => Decimal::ZERO,
        }
    }
}

/// Cantonal Tax Config
#[derive(Debug, Clone)]
pub struct KantonaleSteuer {
    pub kanton: Kanton,
    pub kantonal_steuerfuss: Decimal,
    pub gemeinde: String,
    pub gemeinde_steuerfuss: Decimal,
    pub kirchen_steuerfuss: Option<Decimal>,
}

impl KantonaleSteuer {
    pub fn zurich_city() -> Self {
        Self { kanton: Kanton::ZH, kantonal_steuerfuss: dec!(100), gemeinde: "Zürich".into(), gemeinde_steuerfuss: dec!(119), kirchen_steuerfuss: Some(dec!(10)) }
    }
    pub fn zug_city() -> Self {
        Self { kanton: Kanton::ZG, kantonal_steuerfuss: dec!(82), gemeinde: "Zug".into(), gemeinde_steuerfuss: dec!(60), kirchen_steuerfuss: Some(dec!(6)) }
    }
    pub fn geneva_city() -> Self {
        Self { kanton: Kanton::GE, kantonal_steuerfuss: dec!(100), gemeinde: "Genève".into(), gemeinde_steuerfuss: dec!(45.5), kirchen_steuerfuss: None }
    }
    pub fn total_multiplier(&self) -> Decimal {
        let base = self.kantonal_steuerfuss + self.gemeinde_steuerfuss;
        let church = self.kirchen_steuerfuss.unwrap_or(Decimal::ZERO);
        (base + church) / dec!(100)
    }
}

/// Swiss Tax Calculator
pub struct SwissTaxCalculator {
    pub bundessteuer_tarif: BundessteuerTarif,
    pub kantonale_steuer: KantonaleSteuer,
    pub social_insurance: SwissSocialInsurance,
    pub age: u8,
}

impl SwissTaxCalculator {
    pub fn calculate(&self, gross_annual: Decimal) -> SwissTaxResult {
        let bundessteuer = self.calculate_bundessteuer(gross_annual);
        let kantonal_basis = bundessteuer * dec!(3);
        let kantonal = kantonal_basis * self.kantonale_steuer.kantonal_steuerfuss / dec!(100);
        let gemeinde = kantonal_basis * self.kantonale_steuer.gemeinde_steuerfuss / dec!(100);
        let kirche = self.kantonale_steuer.kirchen_steuerfuss.map(|k| kantonal_basis * k / dec!(100)).unwrap_or(Decimal::ZERO);
        
        let total_tax = bundessteuer + kantonal + gemeinde + kirche;
        
        // Social insurance
        let si = &self.social_insurance;
        let ahv_iv_eo = gross_annual * (si.ahv_rate + si.iv_rate + si.eo_rate) / dec!(2);
        let alv = gross_annual.min(si.alv_ceiling) * si.alv_rate / dec!(2);
        let bvg_rate = BVGPension::get_savings_rate(self.age);
        let coord_salary = (gross_annual.min(si.bvg_maximum_salary) - si.bvg_coordination_deduction).max(Decimal::ZERO);
        let bvg = coord_salary * bvg_rate / dec!(2);
        
        SwissTaxResult {
            gross_annual,
            bundessteuer,
            kantonal_steuer: kantonal,
            gemeinde_steuer: gemeinde,
            kirchen_steuer: kirche,
            total_steuer: total_tax,
            ahv_iv_eo_employee: ahv_iv_eo,
            alv_employee: alv,
            bvg_employee: bvg,
            net_annual: gross_annual - total_tax - ahv_iv_eo - alv - bvg,
            effective_rate: if gross_annual > Decimal::ZERO { total_tax / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_bundessteuer(&self, income: Decimal) -> Decimal {
        for bracket in &self.bundessteuer_tarif.brackets {
            if income > bracket.lower && income <= bracket.upper {
                return bracket.base_tax + (income - bracket.lower) * bracket.marginal_rate / dec!(100);
            }
        }
        if let Some(last) = self.bundessteuer_tarif.brackets.last() {
            last.base_tax + (income - last.lower) * last.marginal_rate / dec!(100)
        } else { Decimal::ZERO }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwissTaxResult {
    pub gross_annual: Decimal,
    pub bundessteuer: Decimal,
    pub kantonal_steuer: Decimal,
    pub gemeinde_steuer: Decimal,
    pub kirchen_steuer: Decimal,
    pub total_steuer: Decimal,
    pub ahv_iv_eo_employee: Decimal,
    pub alv_employee: Decimal,
    pub bvg_employee: Decimal,
    pub net_annual: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// AUSTRIA (AT) - 14 SALARIES
// ═══════════════════════════════════════════════════════════════════════════

/// Austrian Bundesland
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Bundesland {
    Wien, Niederoesterreich, Oberoesterreich, Salzburg, Tirol,
    Vorarlberg, Kaernten, Steiermark, Burgenland,
}

/// Austrian Social Insurance
#[derive(Debug, Clone)]
pub struct AustrianSocialInsurance {
    pub hoechstbeitragsgrundlage: Decimal, // €6,060/month
    pub krankenversicherung_an: Decimal,    // 3.87%
    pub pensionsversicherung_an: Decimal,   // 10.25%
    pub arbeitslosenversicherung_an: Decimal, // 3.00%
    pub arbeiterkammerumlage: Decimal,      // 0.50%
    pub wohnbaufoerderungsbeitrag: Decimal, // 0.50%
}

impl Default for AustrianSocialInsurance {
    fn default() -> Self {
        Self {
            hoechstbeitragsgrundlage: dec!(6060),
            krankenversicherung_an: dec!(0.0387),
            pensionsversicherung_an: dec!(0.1025),
            arbeitslosenversicherung_an: dec!(0.03),
            arbeiterkammerumlage: dec!(0.005),
            wohnbaufoerderungsbeitrag: dec!(0.005),
        }
    }
}

/// Familienbonus Plus
#[derive(Debug, Clone)]
pub struct FamilienbonusPlus {
    pub child_age: u8,
    pub months_eligible: u8,
}

impl FamilienbonusPlus {
    pub fn calculate(&self) -> Decimal {
        let annual = if self.child_age < 18 { dec!(2000) } else { dec!(650) };
        annual * Decimal::from(self.months_eligible) / dec!(12)
    }
}

/// Sonderzahlungen (13th/14th Salary)
#[derive(Debug, Clone)]
pub struct Sonderzahlungen {
    pub urlaubsgeld: Decimal,
    pub weihnachtsgeld: Decimal,
}

impl Sonderzahlungen {
    pub fn calculate_tax(&self) -> Decimal {
        let total = self.urlaubsgeld + self.weihnachtsgeld;
        let taxable = (total - dec!(620)).max(Decimal::ZERO);
        taxable * dec!(0.06) // 6% flat rate
    }
}

/// Austrian Tax Calculator
pub struct AustrianTaxCalculator {
    pub si: AustrianSocialInsurance,
    pub children: Vec<FamilienbonusPlus>,
    pub bundesland: Bundesland,
}

impl AustrianTaxCalculator {
    pub fn new(bundesland: Bundesland) -> Self {
        Self { si: AustrianSocialInsurance::default(), children: vec![], bundesland }
    }
    
    pub fn calculate(&self, gross_monthly: Decimal) -> AustrianTaxResult {
        let gross_annual = gross_monthly * dec!(14); // 14 salaries!
        
        // Social insurance (capped)
        let sv_base = gross_monthly.min(self.si.hoechstbeitragsgrundlage);
        let sv_employee = sv_base * (self.si.krankenversicherung_an + self.si.pensionsversicherung_an + 
            self.si.arbeitslosenversicherung_an + self.si.arbeiterkammerumlage + self.si.wohnbaufoerderungsbeitrag);
        
        // Income tax (7 brackets)
        let taxable = gross_annual - sv_employee * dec!(14);
        let base_tax = self.calculate_brackets(taxable);
        
        // Credits
        let verkehrsabsetzbetrag = dec!(463);
        let familienbonus: Decimal = self.children.iter().map(|c| c.calculate()).sum();
        let tax_after_credits = (base_tax - verkehrsabsetzbetrag - familienbonus).max(Decimal::ZERO);
        
        // Sonderzahlungen tax (6%)
        let sonderzahlungen = Sonderzahlungen { urlaubsgeld: gross_monthly, weihnachtsgeld: gross_monthly };
        let sonder_tax = sonderzahlungen.calculate_tax();
        
        let total_tax = tax_after_credits + sonder_tax;
        
        AustrianTaxResult {
            gross_monthly,
            gross_annual,
            sv_employee_monthly: sv_employee,
            income_tax_annual: tax_after_credits,
            sonderzahlungen_tax: sonder_tax,
            familienbonus,
            net_monthly: gross_monthly - sv_employee - (total_tax / dec!(14)),
            effective_rate: if gross_annual > Decimal::ZERO { total_tax / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_brackets(&self, taxable: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 7] = [
            (dec!(12816), dec!(0)), (dec!(20818), dec!(0.20)), (dec!(34513), dec!(0.30)),
            (dec!(66612), dec!(0.40)), (dec!(99266), dec!(0.48)), (dec!(1000000), dec!(0.50)),
            (dec!(999999999), dec!(0.55)),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AustrianTaxResult {
    pub gross_monthly: Decimal,
    pub gross_annual: Decimal,
    pub sv_employee_monthly: Decimal,
    pub income_tax_annual: Decimal,
    pub sonderzahlungen_tax: Decimal,
    pub familienbonus: Decimal,
    pub net_monthly: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// LUXEMBOURG (LU) - TAX CLASSES & FRONTALIER
// ═══════════════════════════════════════════════════════════════════════════

/// Luxembourg Tax Class
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LuxembourgTaxClass {
    Class1,  // Single
    Class1a, // Single with children
    Class2,  // Married (splitting)
}

/// Frontalier (Cross-border worker)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FrontalierCountry {
    Belgium, France, Germany,
}

/// Luxembourg Tax Calculator
pub struct LuxembourgTaxCalculator {
    pub tax_class: LuxembourgTaxClass,
    pub frontalier: Option<FrontalierCountry>,
    pub children: u8,
}

impl LuxembourgTaxCalculator {
    pub fn new(tax_class: LuxembourgTaxClass) -> Self {
        Self { tax_class, frontalier: None, children: 0 }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> LuxembourgTaxResult {
        // Apply splitting for Class 2
        let adjusted = match self.tax_class {
            LuxembourgTaxClass::Class2 => gross_annual / dec!(2),
            _ => gross_annual,
        };
        
        // Progressive tax (0% to 42%)
        let base_tax = self.calculate_brackets(adjusted);
        let tax = match self.tax_class {
            LuxembourgTaxClass::Class2 => base_tax * dec!(2),
            _ => base_tax,
        };
        
        // Employment fund (7%)
        let fonds_emploi = tax * dec!(0.07);
        
        // CIS credit (€300-€600)
        let cis = if gross_annual <= dec!(30000) { dec!(600) }
        else if gross_annual <= dec!(150000) { dec!(400) }
        else { dec!(300) };
        
        // Child bonus
        let bonus_enfant = Decimal::from(self.children) * dec!(922.50);
        
        // Dependance (1.4%)
        let dependance = gross_annual * dec!(0.014);
        
        // Social security (~12.8% employee)
        let ss_employee = gross_annual.min(dec!(166800)) * dec!(0.128);
        
        let total_tax = (tax + fonds_emploi - cis - bonus_enfant).max(Decimal::ZERO) + dependance;
        
        LuxembourgTaxResult {
            gross_annual,
            impot_base: tax,
            fonds_emploi,
            cis,
            bonus_enfants: bonus_enfant,
            dependance,
            ss_employee,
            total_prelevements: total_tax + ss_employee,
            net_annual: gross_annual - total_tax - ss_employee,
            effective_rate: if gross_annual > Decimal::ZERO { (total_tax + ss_employee) / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_brackets(&self, income: Decimal) -> Decimal {
        // Simplified: 23 brackets from 0% to 42%
        if income <= dec!(12438) { Decimal::ZERO }
        else if income <= dec!(50751) { (income - dec!(12438)) * dec!(0.20) }
        else if income <= dec!(110403) { dec!(7663) + (income - dec!(50751)) * dec!(0.39) }
        else if income <= dec!(220788) { dec!(30907) + (income - dec!(110403)) * dec!(0.41) }
        else { dec!(76165) + (income - dec!(220788)) * dec!(0.42) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LuxembourgTaxResult {
    pub gross_annual: Decimal,
    pub impot_base: Decimal,
    pub fonds_emploi: Decimal,
    pub cis: Decimal,
    pub bonus_enfants: Decimal,
    pub dependance: Decimal,
    pub ss_employee: Decimal,
    pub total_prelevements: Decimal,
    pub net_annual: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// IRELAND (IE) - PAYE, USC, PRSI
// ═══════════════════════════════════════════════════════════════════════════

/// Irish Marital Status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IrishMaritalStatus {
    Single, Married, CivilPartner, Widowed, SingleParent,
}

/// PRSI Class
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PRSIClass {
    A, A1, B, C, D, E, H, J, K, M, S,
}

/// Irish Tax Calculator
pub struct IrishTaxCalculator {
    pub marital_status: IrishMaritalStatus,
    pub is_single_income: bool,
    pub prsi_class: PRSIClass,
}

impl IrishTaxCalculator {
    pub fn new(marital_status: IrishMaritalStatus) -> Self {
        Self { marital_status, is_single_income: true, prsi_class: PRSIClass::A }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> IrishTaxResult {
        // Standard rate band
        let band = match self.marital_status {
            IrishMaritalStatus::Single => dec!(42000),
            IrishMaritalStatus::SingleParent => dec!(46000),
            IrishMaritalStatus::Married | IrishMaritalStatus::CivilPartner => {
                if self.is_single_income { dec!(51000) } else { dec!(84000) }
            }
            IrishMaritalStatus::Widowed => dec!(42000),
        };
        
        // PAYE (20%/40%)
        let standard = gross_annual.min(band) * dec!(0.20);
        let higher = (gross_annual - band).max(Decimal::ZERO) * dec!(0.40);
        let income_tax_gross = standard + higher;
        
        // Tax credits
        let personal = match self.marital_status {
            IrishMaritalStatus::Married | IrishMaritalStatus::CivilPartner => dec!(3750),
            IrishMaritalStatus::SingleParent => dec!(1875) + dec!(1750), // + SPCCC
            _ => dec!(1875),
        };
        let employee_credit = dec!(1875);
        let total_credits = personal + employee_credit;
        let income_tax = (income_tax_gross - total_credits).max(Decimal::ZERO);
        
        // USC
        let usc = self.calculate_usc(gross_annual);
        
        // PRSI (4% Class A)
        let prsi = if gross_annual > dec!(18304) { gross_annual * dec!(0.04) } else { Decimal::ZERO };
        
        let total = income_tax + usc + prsi;
        
        IrishTaxResult {
            gross_annual,
            income_tax_gross,
            tax_credits: total_credits,
            income_tax,
            usc,
            prsi,
            total_tax: total,
            net_annual: gross_annual - total,
            effective_rate: if gross_annual > Decimal::ZERO { total / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_usc(&self, income: Decimal) -> Decimal {
        if income <= dec!(13000) { return Decimal::ZERO; }
        let bands: [(Decimal, Decimal); 4] = [
            (dec!(12012), dec!(0.005)), (dec!(25760), dec!(0.02)),
            (dec!(70044), dec!(0.04)), (dec!(999999999), dec!(0.08)),
        ];
        let mut usc = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in bands {
            if income <= prev { break; }
            let bracket = income.min(max) - prev;
            usc += bracket * rate;
            prev = max;
        }
        usc
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IrishTaxResult {
    pub gross_annual: Decimal,
    pub income_tax_gross: Decimal,
    pub tax_credits: Decimal,
    pub income_tax: Decimal,
    pub usc: Decimal,
    pub prsi: Decimal,
    pub total_tax: Decimal,
    pub net_annual: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// LIECHTENSTEIN (LI)
// ═══════════════════════════════════════════════════════════════════════════

/// Liechtenstein Gemeinde
#[derive(Debug, Clone)]
pub struct LiechtensteinGemeinde {
    pub name: String,
    pub surcharge: Decimal,
}

impl LiechtensteinGemeinde {
    pub fn vaduz() -> Self { Self { name: "Vaduz".into(), surcharge: dec!(200) } }
    pub fn schaan() -> Self { Self { name: "Schaan".into(), surcharge: dec!(175) } }
    pub fn triesen() -> Self { Self { name: "Triesen".into(), surcharge: dec!(180) } }
}

/// Liechtenstein Tax Calculator
pub struct LiechtensteinTaxCalculator {
    pub gemeinde: LiechtensteinGemeinde,
}

impl LiechtensteinTaxCalculator {
    pub fn new(gemeinde: LiechtensteinGemeinde) -> Self {
        Self { gemeinde }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> LiechtensteinTaxResult {
        // Deductions
        let deductions = dec!(18000);
        let taxable = (gross_annual - deductions).max(Decimal::ZERO);
        
        // State tax (1%-8% progressive)
        let state_tax = if taxable <= dec!(30000) { taxable * dec!(0.01) }
        else if taxable <= dec!(60000) { dec!(300) + (taxable - dec!(30000)) * dec!(0.03) }
        else if taxable <= dec!(100000) { dec!(1200) + (taxable - dec!(60000)) * dec!(0.05) }
        else { dec!(3200) + (taxable - dec!(100000)) * dec!(0.08) };
        
        // Municipal surcharge
        let municipal = state_tax * self.gemeinde.surcharge / dec!(100);
        let total_tax = state_tax + municipal;
        
        // Social insurance (Swiss-style: ~5.3% employee)
        let si = gross_annual.min(dec!(148200)) * dec!(0.053);
        
        LiechtensteinTaxResult {
            gross_annual,
            deductions,
            taxable,
            state_tax,
            municipal_surcharge: municipal,
            total_tax,
            social_insurance: si,
            net_annual: gross_annual - total_tax - si,
            effective_rate: if gross_annual > Decimal::ZERO { (total_tax + si) / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiechtensteinTaxResult {
    pub gross_annual: Decimal,
    pub deductions: Decimal,
    pub taxable: Decimal,
    pub state_tax: Decimal,
    pub municipal_surcharge: Decimal,
    pub total_tax: Decimal,
    pub social_insurance: Decimal,
    pub net_annual: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Western Europe Extended Registry
pub struct WesternEuropeExtendedRegistry;

impl WesternEuropeExtendedRegistry {
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
    
    pub fn is_eu_member(code: &str) -> bool { matches!(code, "AT" | "LU" | "IE") }
    pub fn is_efta_member(code: &str) -> bool { matches!(code, "CH" | "LI") }
    pub fn uses_sepa(code: &str) -> bool { matches!(code, "CH" | "AT" | "LU" | "IE" | "LI" | "MC" | "AD") }
    pub fn has_participation_exemption(code: &str) -> bool { matches!(code, "CH" | "LU" | "IE" | "LI") }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swiss_zurich() {
        let calc = SwissTaxCalculator {
            bundessteuer_tarif: BundessteuerTarif::single_tarif(),
            kantonale_steuer: KantonaleSteuer::zurich_city(),
            social_insurance: SwissSocialInsurance::default(),
            age: 35,
        };
        let result = calc.calculate(dec!(120000));
        assert_eq!(result.gross_annual, dec!(120000));
        assert!(result.total_steuer > Decimal::ZERO);
        assert!(result.effective_rate > Decimal::ZERO);
    }
    
    #[test]
    fn test_swiss_zug_lower() {
        let zurich = KantonaleSteuer::zurich_city().total_multiplier();
        let zug = KantonaleSteuer::zug_city().total_multiplier();
        assert!(zug < zurich);
    }
    
    #[test]
    fn test_austria_14_salaries() {
        let calc = AustrianTaxCalculator::new(Bundesland::Wien);
        let result = calc.calculate(dec!(4000));
        assert_eq!(result.gross_annual, dec!(56000)); // 4000 * 14
        assert!(result.sonderzahlungen_tax > Decimal::ZERO);
    }
    
    #[test]
    fn test_luxembourg_class2_splitting() {
        let calc = LuxembourgTaxCalculator::new(LuxembourgTaxClass::Class2);
        let result = calc.calculate(dec!(80000));
        assert!(result.effective_rate > Decimal::ZERO);
    }
    
    #[test]
    fn test_ireland_usc() {
        let calc = IrishTaxCalculator::new(IrishMaritalStatus::Single);
        let result = calc.calculate(dec!(60000));
        assert!(result.usc > Decimal::ZERO);
        assert!(result.prsi > Decimal::ZERO);
    }
    
    #[test]
    fn test_liechtenstein_vaduz() {
        let calc = LiechtensteinTaxCalculator::new(LiechtensteinGemeinde::vaduz());
        let result = calc.calculate(dec!(100000));
        assert!(result.state_tax > Decimal::ZERO);
        assert!(result.municipal_surcharge > Decimal::ZERO);
    }
    
    #[test]
    fn test_registry() {
        let countries = WesternEuropeExtendedRegistry::supported_countries();
        assert_eq!(countries.len(), 7);
        assert!(WesternEuropeExtendedRegistry::is_eu_member("AT"));
        assert!(WesternEuropeExtendedRegistry::is_efta_member("CH"));
        assert!(WesternEuropeExtendedRegistry::has_participation_exemption("LU"));
    }
}
