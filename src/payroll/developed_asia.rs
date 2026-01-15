//! Developed Asia Tax Engines
//! 
//! Comprehensive tax calculators for high-GDP Asian economies:
//! - Japan: 7 brackets (5%-45%), residence tax, bonus taxation
//! - South Korea: 8 brackets (6%-45%), 4 insurances
//! - Taiwan: 6 brackets (5%-40%), labor insurance
//! - Hong Kong: Progressive vs Standard rate (15%), MPF
//! - Singapore: 13 brackets (0%-24%), CPF by age

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// JAPAN (JP) - 所得税 SHOTOKU-ZEI
// ═══════════════════════════════════════════════════════════════════════════

/// Japan Social Insurance (社会保険)
#[derive(Debug, Clone)]
pub struct JapanSocialInsurance {
    pub health_rate: Decimal,              // ~10% (split 50/50)
    pub pension_rate: Decimal,             // 18.3% (split 50/50)
    pub employment_ee: Decimal,            // 0.6%
    pub employment_er: Decimal,            // 0.95%
    pub nursing_rate: Decimal,             // 1.8% (ages 40-64)
    pub max_standard_monthly: Decimal,     // ¥1,390,000
}

impl Default for JapanSocialInsurance {
    fn default() -> Self {
        Self {
            health_rate: dec!(0.10), pension_rate: dec!(0.183),
            employment_ee: dec!(0.006), employment_er: dec!(0.0095),
            nursing_rate: dec!(0.018), max_standard_monthly: dec!(1390000),
        }
    }
}

/// Japan Tax Calculator
pub struct JapanTaxCalculator {
    pub si: JapanSocialInsurance,
    pub num_dependents: u8,
    pub age: u8,
}

impl JapanTaxCalculator {
    pub fn new() -> Self {
        Self { si: JapanSocialInsurance::default(), num_dependents: 0, age: 35 }
    }
    
    /// Calculate monthly payroll (源泉徴収)
    pub fn calculate_monthly(&self, monthly_salary: Decimal, prev_year_income: Decimal) -> JapanPayrollResult {
        let si = &self.si;
        
        // Standard monthly remuneration (標準報酬月額)
        let standard = (monthly_salary / dec!(10000)).round() * dec!(10000);
        let capped = standard.min(si.max_standard_monthly);
        
        // Social insurance (employee portion = 50%)
        let health = capped * si.health_rate / dec!(2);
        let nursing = if self.age >= 40 && self.age <= 64 { capped * si.nursing_rate / dec!(2) } else { Decimal::ZERO };
        let pension = capped * si.pension_rate / dec!(2);
        let employment = monthly_salary * si.employment_ee;
        let si_employee = health + nursing + pension + employment;
        
        // Employer contributions
        let si_employer = health + nursing + pension + monthly_salary * si.employment_er;
        
        // Taxable income
        let annual_projection = (monthly_salary - si_employee) * dec!(12);
        let employment_deduction = self.employment_income_deduction(annual_projection);
        let basic_deduction = dec!(480000);
        let dependent_deduction = dec!(380000) * Decimal::from(self.num_dependents);
        let taxable = (annual_projection - employment_deduction - basic_deduction - dependent_deduction).max(Decimal::ZERO);
        
        // Income tax (7 brackets)
        let annual_tax = self.calculate_income_tax(taxable);
        let income_tax = annual_tax / dec!(12);
        
        // Reconstruction surtax (2.1%)
        let reconstruction = income_tax * dec!(0.021);
        
        // Residence tax (住民税 - based on previous year, 10%)
        let prev_taxable = (prev_year_income - basic_deduction).max(Decimal::ZERO);
        let residence_tax = (prev_taxable * dec!(0.10) + dec!(5000)) / dec!(12);
        
        let total_deductions = si_employee + income_tax + reconstruction + residence_tax;
        
        JapanPayrollResult {
            monthly_salary,
            standard_monthly: standard,
            health_pension_employee: health + nursing + pension,
            employment_insurance: employment,
            income_tax: income_tax.round_dp(0),
            reconstruction_tax: reconstruction.round_dp(0),
            residence_tax: residence_tax.round_dp(0),
            total_deductions: total_deductions.round_dp(0),
            net_pay: (monthly_salary - total_deductions).round_dp(0),
            employer_cost: (monthly_salary + si_employer).round_dp(0),
        }
    }
    
    /// Calculate bonus tax (賞与)
    pub fn calculate_bonus(&self, bonus: Decimal, prev_month_salary: Decimal) -> JapanBonusResult {
        let si = &self.si;
        
        // SI on bonus (capped at 3x max)
        let capped = bonus.min(si.max_standard_monthly * dec!(3));
        let si_employee = capped * (si.health_rate + si.pension_rate) / dec!(2) + bonus * si.employment_ee;
        
        // Bonus tax rate (simplified - based on previous month)
        let rate = if prev_month_salary < dec!(79000) { Decimal::ZERO }
        else if prev_month_salary < dec!(252000) { dec!(0.02042) }
        else if prev_month_salary < dec!(300000) { dec!(0.04084) }
        else { dec!(0.06126) };
        
        let taxable = bonus - si_employee;
        let income_tax = taxable * rate;
        let reconstruction = income_tax * dec!(0.021);
        
        JapanBonusResult {
            gross_bonus: bonus,
            social_insurance: si_employee.round_dp(0),
            income_tax: income_tax.round_dp(0),
            reconstruction_tax: reconstruction.round_dp(0),
            net_bonus: (bonus - si_employee - income_tax - reconstruction).round_dp(0),
        }
    }
    
    fn employment_income_deduction(&self, annual: Decimal) -> Decimal {
        if annual <= dec!(1625000) { dec!(550000) }
        else if annual <= dec!(1800000) { annual * dec!(0.40) - dec!(100000) }
        else if annual <= dec!(3600000) { annual * dec!(0.30) + dec!(80000) }
        else if annual <= dec!(6600000) { annual * dec!(0.20) + dec!(440000) }
        else if annual <= dec!(8500000) { annual * dec!(0.10) + dec!(1100000) }
        else { dec!(1950000) }
    }
    
    fn calculate_income_tax(&self, taxable: Decimal) -> Decimal {
        // 7 brackets with deduction method
        let brackets: [(Decimal, Decimal, Decimal); 7] = [
            (dec!(1950000), dec!(0.05), Decimal::ZERO),
            (dec!(3300000), dec!(0.10), dec!(97500)),
            (dec!(6950000), dec!(0.20), dec!(427500)),
            (dec!(9000000), dec!(0.23), dec!(636000)),
            (dec!(18000000), dec!(0.33), dec!(1536000)),
            (dec!(40000000), dec!(0.40), dec!(2796000)),
            (dec!(999999999999), dec!(0.45), dec!(4796000)),
        ];
        
        for (max, rate, deduction) in brackets {
            if taxable <= max {
                return (taxable * rate - deduction).max(Decimal::ZERO);
            }
        }
        Decimal::ZERO
    }
}

impl Default for JapanTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JapanPayrollResult {
    pub monthly_salary: Decimal,
    pub standard_monthly: Decimal,
    pub health_pension_employee: Decimal,
    pub employment_insurance: Decimal,
    pub income_tax: Decimal,
    pub reconstruction_tax: Decimal,
    pub residence_tax: Decimal,
    pub total_deductions: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JapanBonusResult {
    pub gross_bonus: Decimal,
    pub social_insurance: Decimal,
    pub income_tax: Decimal,
    pub reconstruction_tax: Decimal,
    pub net_bonus: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// SOUTH KOREA (KR) - 소득세 SODEUKSE
// ═══════════════════════════════════════════════════════════════════════════

/// Korean 4 Insurances (4대보험)
#[derive(Debug, Clone)]
pub struct KoreanFourInsurances {
    pub national_pension_ee: Decimal,   // 4.5% (국민연금)
    pub national_pension_er: Decimal,   // 4.5%
    pub health_insurance_ee: Decimal,   // 3.545% (건강보험)
    pub health_insurance_er: Decimal,   // 3.545%
    pub long_term_care_ee: Decimal,     // 12.95% of health (장기요양보험)
    pub employment_insurance_ee: Decimal, // 0.9% (고용보험)
    pub employment_insurance_er: Decimal, // 0.9%-1.65%
    pub industrial_accident_er: Decimal,  // Varies (산재보험)
}

impl Default for KoreanFourInsurances {
    fn default() -> Self {
        Self {
            national_pension_ee: dec!(0.045), national_pension_er: dec!(0.045),
            health_insurance_ee: dec!(0.03545), health_insurance_er: dec!(0.03545),
            long_term_care_ee: dec!(0.1295),
            employment_insurance_ee: dec!(0.009), employment_insurance_er: dec!(0.0135),
            industrial_accident_er: dec!(0.01), // Average
        }
    }
}

/// Korean Tax Calculator
pub struct KoreanTaxCalculator {
    pub insurances: KoreanFourInsurances,
}

impl KoreanTaxCalculator {
    pub fn new() -> Self { Self { insurances: KoreanFourInsurances::default() } }
    
    pub fn calculate(&self, gross_annual: Decimal) -> KoreanTaxResult {
        let ins = &self.insurances;
        
        // 4 Insurances (employee portions)
        let pension = gross_annual * ins.national_pension_ee;
        let health = gross_annual * ins.health_insurance_ee;
        let long_term = health * ins.long_term_care_ee;
        let employment = gross_annual * ins.employment_insurance_ee;
        let social_total = pension + health + long_term + employment;
        
        // Income tax (8 brackets: 6%-45%)
        let taxable = (gross_annual - social_total).max(Decimal::ZERO);
        let income_tax = self.calculate_income_tax(taxable);
        
        // Local income tax (10% of income tax)
        let local_tax = income_tax * dec!(0.10);
        
        KoreanTaxResult {
            geup_yeo: gross_annual,
            gukmin_yeonkeum: pension.round_dp(0),
            geongang_boheom: health.round_dp(0),
            janggi_yoyang: long_term.round_dp(0),
            goyong_boheom: employment.round_dp(0),
            sodeuk_se: income_tax.round_dp(0),
            jibangsodeuk_se: local_tax.round_dp(0),
            silsu_ryeong: (gross_annual - social_total - income_tax - local_tax).round_dp(0),
        }
    }
    
    fn calculate_income_tax(&self, taxable: Decimal) -> Decimal {
        // 8 brackets
        let brackets: [(Decimal, Decimal, Decimal); 8] = [
            (dec!(14000000), dec!(0.06), Decimal::ZERO),
            (dec!(50000000), dec!(0.15), dec!(1260000)),
            (dec!(88000000), dec!(0.24), dec!(5760000)),
            (dec!(150000000), dec!(0.35), dec!(15440000)),
            (dec!(300000000), dec!(0.38), dec!(19940000)),
            (dec!(500000000), dec!(0.40), dec!(25940000)),
            (dec!(1000000000), dec!(0.42), dec!(35940000)),
            (dec!(999999999999), dec!(0.45), dec!(65940000)),
        ];
        
        for (max, rate, deduction) in brackets {
            if taxable <= max {
                return (taxable * rate - deduction).max(Decimal::ZERO);
            }
        }
        Decimal::ZERO
    }
}

impl Default for KoreanTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KoreanTaxResult {
    pub geup_yeo: Decimal,            // 급여 (salary)
    pub gukmin_yeonkeum: Decimal,     // 국민연금 (pension)
    pub geongang_boheom: Decimal,     // 건강보험 (health)
    pub janggi_yoyang: Decimal,       // 장기요양 (long-term care)
    pub goyong_boheom: Decimal,       // 고용보험 (employment)
    pub sodeuk_se: Decimal,           // 소득세 (income tax)
    pub jibangsodeuk_se: Decimal,     // 지방소득세 (local tax)
    pub silsu_ryeong: Decimal,        // 실수령액 (net pay)
}

// ═══════════════════════════════════════════════════════════════════════════
// TAIWAN (TW) - 所得稅 SUODE SHUI
// ═══════════════════════════════════════════════════════════════════════════

/// Taiwan Tax Calculator
pub struct TaiwanTaxCalculator {
    pub num_dependents: u8,
}

impl TaiwanTaxCalculator {
    pub fn new() -> Self { Self { num_dependents: 0 } }
    
    pub fn calculate(&self, gross_annual: Decimal) -> TaiwanTaxResult {
        // Labor insurance (勞保) 11.5% (employee 20% = 2.3%)
        let labor_insurance = gross_annual * dec!(0.023);
        
        // Health insurance (健保) 5.17% (employee 30% = 1.55%)
        let health_insurance = gross_annual * dec!(0.0155);
        
        // Standard deduction NT$124,000 single / NT$248,000 married
        let standard_deduction = dec!(124000);
        let personal_exemption = dec!(92000) * (Decimal::ONE + Decimal::from(self.num_dependents));
        
        let taxable = (gross_annual - labor_insurance - health_insurance - standard_deduction - personal_exemption).max(Decimal::ZERO);
        
        // 6 brackets (5%-40%)
        let income_tax = self.calculate_income_tax(taxable);
        
        TaiwanTaxResult {
            nian_shou_ru: gross_annual,
            lao_bao: labor_insurance.round_dp(0),
            jian_bao: health_insurance.round_dp(0),
            suo_de_shui: income_tax.round_dp(0),
            shi_ling: (gross_annual - labor_insurance - health_insurance - income_tax).round_dp(0),
        }
    }
    
    fn calculate_income_tax(&self, taxable: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 6] = [
            (dec!(560000), dec!(0.05)),
            (dec!(1260000), dec!(0.12)),
            (dec!(2520000), dec!(0.20)),
            (dec!(4720000), dec!(0.30)),
            (dec!(10310000), dec!(0.40)),
            (dec!(999999999999), dec!(0.40)),
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

impl Default for TaiwanTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaiwanTaxResult {
    pub nian_shou_ru: Decimal,  // 年收入 (annual income)
    pub lao_bao: Decimal,       // 勞保 (labor insurance)
    pub jian_bao: Decimal,      // 健保 (health insurance)
    pub suo_de_shui: Decimal,   // 所得稅 (income tax)
    pub shi_ling: Decimal,      // 實領 (net pay)
}

// ═══════════════════════════════════════════════════════════════════════════
// HONG KONG (HK) - SALARIES TAX
// ═══════════════════════════════════════════════════════════════════════════

/// Hong Kong Marital Status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HkMaritalStatus { Single, Married }

/// Hong Kong Tax Calculator
pub struct HongKongTaxCalculator {
    pub marital_status: HkMaritalStatus,
    pub num_children: u8,
}

impl HongKongTaxCalculator {
    pub fn new() -> Self { Self { marital_status: HkMaritalStatus::Single, num_children: 0 } }
    
    pub fn calculate(&self, gross_annual: Decimal, mpf_contributions: Decimal) -> HongKongTaxResult {
        // Allowances
        let personal = match self.marital_status {
            HkMaritalStatus::Single => dec!(132000),
            HkMaritalStatus::Married => dec!(264000),
        };
        let child = dec!(130000) * Decimal::from(self.num_children);
        let mpf_relief = mpf_contributions.min(dec!(18000));
        let total_allowances = personal + child + mpf_relief;
        
        // Progressive tax (5 bands)
        let net_chargeable = (gross_annual - total_allowances).max(Decimal::ZERO);
        let progressive = self.calculate_progressive(net_chargeable);
        
        // Standard rate (15% on net income)
        let standard = (gross_annual - mpf_relief) * dec!(0.15);
        
        // Pay the lower
        let final_tax = progressive.min(standard);
        
        HongKongTaxResult {
            annual_income: gross_annual,
            total_allowances,
            net_chargeable_income: net_chargeable,
            progressive_tax: progressive.round_dp(0),
            standard_tax: standard.round_dp(0),
            final_tax: final_tax.round_dp(0),
            effective_rate: if gross_annual > Decimal::ZERO { final_tax / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_progressive(&self, net_chargeable: Decimal) -> Decimal {
        // 5 bands: 2%, 6%, 10%, 14%, 17%
        let bands: [(Decimal, Decimal); 5] = [
            (dec!(50000), dec!(0.02)),
            (dec!(50000), dec!(0.06)),
            (dec!(50000), dec!(0.10)),
            (dec!(50000), dec!(0.14)),
            (dec!(999999999999), dec!(0.17)),
        ];
        
        let mut tax = Decimal::ZERO;
        let mut remaining = net_chargeable;
        for (width, rate) in bands {
            if remaining <= Decimal::ZERO { break; }
            let in_band = remaining.min(width);
            tax += in_band * rate;
            remaining -= in_band;
        }
        tax
    }
    
    /// Calculate MPF (強積金)
    pub fn calculate_mpf(&self, monthly_income: Decimal) -> HkMpfResult {
        let min_income = dec!(7100);
        let max_income = dec!(30000);
        
        let employee = if monthly_income < min_income { Decimal::ZERO }
        else { (monthly_income.min(max_income) * dec!(0.05)).min(dec!(1500)) };
        
        let employer = (monthly_income.min(max_income) * dec!(0.05)).min(dec!(1500));
        
        HkMpfResult { employee_contribution: employee, employer_contribution: employer, total: employee + employer }
    }
}

impl Default for HongKongTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HongKongTaxResult {
    pub annual_income: Decimal,
    pub total_allowances: Decimal,
    pub net_chargeable_income: Decimal,
    pub progressive_tax: Decimal,
    pub standard_tax: Decimal,
    pub final_tax: Decimal,
    pub effective_rate: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HkMpfResult {
    pub employee_contribution: Decimal,
    pub employer_contribution: Decimal,
    pub total: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// SINGAPORE (SG) - IRAS + CPF
// ═══════════════════════════════════════════════════════════════════════════

/// Singapore CPF Rates by Age
#[derive(Debug, Clone)]
pub struct CpfRatesByAge {
    pub employee_rate: Decimal,
    pub employer_rate: Decimal,
}

impl CpfRatesByAge {
    pub fn for_age(age: u8) -> Self {
        match age {
            0..=55 => Self { employee_rate: dec!(0.20), employer_rate: dec!(0.17) },
            56..=60 => Self { employee_rate: dec!(0.16), employer_rate: dec!(0.145) },
            61..=65 => Self { employee_rate: dec!(0.105), employer_rate: dec!(0.11) },
            66..=70 => Self { employee_rate: dec!(0.075), employer_rate: dec!(0.085) },
            _ => Self { employee_rate: dec!(0.05), employer_rate: dec!(0.075) },
        }
    }
}

/// Singapore Tax Calculator
pub struct SingaporeTaxCalculator {
    pub age: u8,
    pub is_pr_or_citizen: bool,
}

impl SingaporeTaxCalculator {
    pub fn new() -> Self { Self { age: 35, is_pr_or_citizen: true } }
    
    pub fn calculate_monthly(&self, gross_monthly: Decimal, bonus: Decimal) -> SingaporePayrollResult {
        // CPF ceiling: $6,800/month OW
        let ow_ceiling = dec!(6800);
        let ordinary_wages = gross_monthly.min(ow_ceiling);
        
        let cpf_rates = CpfRatesByAge::for_age(self.age);
        
        // CPF contributions (only for PR/Citizens)
        let (cpf_ee, cpf_er) = if self.is_pr_or_citizen {
            ((ordinary_wages + bonus) * cpf_rates.employee_rate,
             (ordinary_wages + bonus) * cpf_rates.employer_rate)
        } else { (Decimal::ZERO, Decimal::ZERO) };
        
        // Estimate annual tax
        let annual_gross = gross_monthly * dec!(12) + bonus;
        let annual_cpf = cpf_ee * dec!(12);
        let taxable = annual_gross - annual_cpf; // CPF relief
        let annual_tax = self.calculate_income_tax(taxable);
        let monthly_tax = annual_tax / dec!(12);
        
        SingaporePayrollResult {
            gross_salary: gross_monthly,
            bonus,
            cpf_employee: cpf_ee.round_dp(2),
            cpf_employer: cpf_er.round_dp(2),
            estimated_tax: monthly_tax.round_dp(2),
            net_pay: (gross_monthly + bonus - cpf_ee - monthly_tax).round_dp(2),
            employer_cost: gross_monthly + bonus + cpf_er,
        }
    }
    
    fn calculate_income_tax(&self, taxable: Decimal) -> Decimal {
        // 13 brackets (0%-24%)
        let brackets: [(Decimal, Decimal, Decimal); 13] = [
            (dec!(20000), dec!(0), Decimal::ZERO),
            (dec!(30000), dec!(0.02), Decimal::ZERO),
            (dec!(40000), dec!(0.035), dec!(200)),
            (dec!(80000), dec!(0.07), dec!(550)),
            (dec!(120000), dec!(0.115), dec!(3350)),
            (dec!(160000), dec!(0.15), dec!(7950)),
            (dec!(200000), dec!(0.18), dec!(13950)),
            (dec!(240000), dec!(0.19), dec!(21150)),
            (dec!(280000), dec!(0.195), dec!(28750)),
            (dec!(320000), dec!(0.20), dec!(36550)),
            (dec!(500000), dec!(0.22), dec!(44550)),
            (dec!(1000000), dec!(0.23), dec!(84150)),
            (dec!(999999999999), dec!(0.24), dec!(199150)),
        ];
        
        for (max, rate, base) in brackets {
            if taxable <= max {
                let excess = (taxable - brackets.iter().find(|b| b.0 < max).map(|b| b.0).unwrap_or(Decimal::ZERO)).max(Decimal::ZERO);
                return base + excess * rate;
            }
        }
        Decimal::ZERO
    }
}

impl Default for SingaporeTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingaporePayrollResult {
    pub gross_salary: Decimal,
    pub bonus: Decimal,
    pub cpf_employee: Decimal,
    pub cpf_employer: Decimal,
    pub estimated_tax: Decimal,
    pub net_pay: Decimal,
    pub employer_cost: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Developed Asia Registry
pub struct DevelopedAsiaRegistry;

impl DevelopedAsiaRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("JP", "Japan", "JPY"),
            ("KR", "South Korea", "KRW"),
            ("TW", "Taiwan", "TWD"),
            ("HK", "Hong Kong", "HKD"),
            ("SG", "Singapore", "SGD"),
        ]
    }
    
    pub fn has_progressive_tax(code: &str) -> bool { matches!(code, "JP" | "KR" | "TW" | "SG") }
    pub fn has_flat_tax_option(code: &str) -> bool { matches!(code, "HK") } // Standard rate option
    pub fn max_tax_rate(code: &str) -> Decimal {
        match code {
            "JP" | "KR" => dec!(45),
            "TW" => dec!(40),
            "SG" => dec!(24),
            "HK" => dec!(17),
            _ => Decimal::ZERO,
        }
    }
    pub fn uses_mandatory_pension(code: &str) -> bool { matches!(code, "JP" | "KR" | "TW" | "SG" | "HK") }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_japan_monthly() {
        let calc = JapanTaxCalculator::new();
        let result = calc.calculate_monthly(dec!(400000), dec!(5000000));
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.health_pension_employee > Decimal::ZERO);
    }
    
    #[test]
    fn test_japan_bonus() {
        let calc = JapanTaxCalculator::new();
        let result = calc.calculate_bonus(dec!(1000000), dec!(400000));
        assert!(result.income_tax > Decimal::ZERO);
        assert!(result.net_bonus < result.gross_bonus);
    }
    
    #[test]
    fn test_korea() {
        let calc = KoreanTaxCalculator::new();
        let result = calc.calculate(dec!(50000000));
        assert!(result.sodeuk_se > Decimal::ZERO);
        assert!(result.gukmin_yeonkeum > Decimal::ZERO);
    }
    
    #[test]
    fn test_taiwan() {
        let calc = TaiwanTaxCalculator::new();
        let result = calc.calculate(dec!(1000000));
        assert!(result.suo_de_shui > Decimal::ZERO);
    }
    
    #[test]
    fn test_hong_kong_progressive_vs_standard() {
        let calc = HongKongTaxCalculator::new();
        let result = calc.calculate(dec!(500000), dec!(18000));
        // Should choose lower of progressive/standard
        assert!(result.final_tax <= result.progressive_tax);
        assert!(result.final_tax <= result.standard_tax);
    }
    
    #[test]
    fn test_hong_kong_mpf() {
        let calc = HongKongTaxCalculator::new();
        let result = calc.calculate_mpf(dec!(25000));
        assert_eq!(result.employee_contribution, dec!(1250)); // 5% of 25k
        assert_eq!(result.employer_contribution, dec!(1250));
    }
    
    #[test]
    fn test_singapore_cpf() {
        let calc = SingaporeTaxCalculator::new();
        let result = calc.calculate_monthly(dec!(6000), Decimal::ZERO);
        assert!(result.cpf_employee > Decimal::ZERO); // 20% for age <= 55
    }
    
    #[test]
    fn test_singapore_foreigner() {
        let mut calc = SingaporeTaxCalculator::new();
        calc.is_pr_or_citizen = false;
        let result = calc.calculate_monthly(dec!(6000), Decimal::ZERO);
        assert_eq!(result.cpf_employee, Decimal::ZERO); // No CPF for foreigners
    }
    
    #[test]
    fn test_registry() {
        let countries = DevelopedAsiaRegistry::supported_countries();
        assert_eq!(countries.len(), 5);
        assert_eq!(DevelopedAsiaRegistry::max_tax_rate("JP"), dec!(45));
        assert!(DevelopedAsiaRegistry::has_flat_tax_option("HK"));
    }
}
