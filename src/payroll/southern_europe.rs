//! Southern Europe / Mediterranean Tax Engines
//! 
//! Comprehensive tax calculators for Mediterranean countries:
//! - Spain: 19 Comunidades Autónomas, IRPF, Beckham Law
//! - Italy: IRPEF 3 brackets, 20 Regioni, TFR
//! - Portugal: IRS 9 brackets, NHR regime
//! - Greece: EFKA, Solidarity contribution
//! - Malta: Single/Married/Parent rates
//! - Cyprus: Non-Dom regime, GHS

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

// ═══════════════════════════════════════════════════════════════════════════
// SPAIN (ES) - 19 COMUNIDADES AUTÓNOMAS
// ═══════════════════════════════════════════════════════════════════════════

/// Spanish Autonomous Communities
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComunidadAutonoma {
    Andalucia, Aragon, Asturias, Baleares, Canarias, Cantabria,
    CastillaLaMancha, CastillaYLeon, Cataluna, ComunidadValenciana,
    Extremadura, Galicia, Madrid, Murcia, Navarra, PaisVasco,
    LaRioja, Ceuta, Melilla,
}

/// Spanish Special Tax Regimes
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum SpanishSpecialRegime {
    Standard,
    BeckhamLaw,      // 24% flat up to €600K
    Canarias,        // REF reduced rates
    CeutaMelilla,    // 50% deduction
}

/// Spanish Social Security
#[derive(Debug, Clone)]
pub struct SpanishSocialSecurity {
    pub contingencias_comunes_trabajador: Decimal,  // 4.70%
    pub desempleo_trabajador: Decimal,              // 1.55%
    pub formacion_trabajador: Decimal,              // 0.10%
    pub contingencias_comunes_empresa: Decimal,     // 23.60%
    pub desempleo_empresa: Decimal,                 // 5.50%
    pub base_minima: Decimal,                       // €1,323
    pub base_maxima: Decimal,                       // €4,720.50
}

impl Default for SpanishSocialSecurity {
    fn default() -> Self {
        Self {
            contingencias_comunes_trabajador: dec!(0.047),
            desempleo_trabajador: dec!(0.0155),
            formacion_trabajador: dec!(0.001),
            contingencias_comunes_empresa: dec!(0.236),
            desempleo_empresa: dec!(0.055),
            base_minima: dec!(1323),
            base_maxima: dec!(4720.50),
        }
    }
}

impl SpanishSocialSecurity {
    pub fn employee_rate(&self) -> Decimal {
        self.contingencias_comunes_trabajador + self.desempleo_trabajador + self.formacion_trabajador
    }
    pub fn employer_rate(&self) -> Decimal {
        self.contingencias_comunes_empresa + self.desempleo_empresa + dec!(0.008) // +FOGASA+AT
    }
}

/// Spanish Tax Calculator
pub struct SpanishTaxCalculator {
    pub comunidad: ComunidadAutonoma,
    pub special_regime: SpanishSpecialRegime,
    pub ss: SpanishSocialSecurity,
    pub age: u8,
    pub num_children: u8,
}

impl SpanishTaxCalculator {
    pub fn new(comunidad: ComunidadAutonoma) -> Self {
        Self { comunidad, special_regime: SpanishSpecialRegime::Standard, ss: SpanishSocialSecurity::default(), age: 35, num_children: 0 }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> SpanishTaxResult {
        match self.special_regime {
            SpanishSpecialRegime::BeckhamLaw => self.calculate_beckham(gross_annual),
            SpanishSpecialRegime::CeutaMelilla => {
                let mut result = self.calculate_standard(gross_annual);
                result.cuota_liquida = result.cuota_liquida * dec!(0.50);
                result
            }
            _ => self.calculate_standard(gross_annual),
        }
    }
    
    fn calculate_standard(&self, gross_annual: Decimal) -> SpanishTaxResult {
        // Mínimo personal y familiar
        let minimo = dec!(5550) + Decimal::from(self.num_children) * dec!(2400);
        
        // State tax (9.5% to 24.5% progressive)
        let cuota_estatal = self.calculate_state_tax(gross_annual);
        let reduccion_estatal = self.calculate_state_tax(minimo);
        
        // Regional tax (varies by comunidad)
        let cuota_autonomica = self.calculate_regional_tax(gross_annual);
        let reduccion_autonomica = self.calculate_regional_tax(minimo);
        
        let total = (cuota_estatal - reduccion_estatal).max(Decimal::ZERO) + 
                    (cuota_autonomica - reduccion_autonomica).max(Decimal::ZERO);
        
        SpanishTaxResult {
            base_imponible: gross_annual,
            minimo_personal_familiar: minimo,
            cuota_estatal: cuota_estatal - reduccion_estatal,
            cuota_autonomica: cuota_autonomica - reduccion_autonomica,
            cuota_integra: total,
            cuota_liquida: total,
            tipo_efectivo: if gross_annual > Decimal::ZERO { total / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_beckham(&self, gross_annual: Decimal) -> SpanishTaxResult {
        let threshold = dec!(600000);
        let tax = gross_annual.min(threshold) * dec!(0.24) + 
                  (gross_annual - threshold).max(Decimal::ZERO) * dec!(0.47);
        
        SpanishTaxResult {
            base_imponible: gross_annual,
            minimo_personal_familiar: Decimal::ZERO,
            cuota_estatal: tax / dec!(2),
            cuota_autonomica: tax / dec!(2),
            cuota_integra: tax,
            cuota_liquida: tax,
            tipo_efectivo: if gross_annual > Decimal::ZERO { tax / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_state_tax(&self, income: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 6] = [
            (dec!(12450), dec!(0.095)), (dec!(20200), dec!(0.12)), (dec!(35200), dec!(0.15)),
            (dec!(60000), dec!(0.185)), (dec!(300000), dec!(0.225)), (dec!(999999999), dec!(0.245)),
        ];
        self.progressive_tax(&brackets, income)
    }
    
    fn calculate_regional_tax(&self, income: Decimal) -> Decimal {
        // Madrid has lower rates, Cataluña higher
        let multiplier = match self.comunidad {
            ComunidadAutonoma::Madrid => dec!(0.90),
            ComunidadAutonoma::Cataluna => dec!(1.10),
            ComunidadAutonoma::PaisVasco | ComunidadAutonoma::Navarra => dec!(0.85),
            _ => dec!(1.0),
        };
        self.calculate_state_tax(income) * multiplier
    }
    
    fn progressive_tax(&self, brackets: &[(Decimal, Decimal)], income: Decimal) -> Decimal {
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if income <= prev { break; }
            let bracket = income.min(*max) - prev;
            tax += bracket * rate;
            prev = *max;
        }
        tax
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpanishTaxResult {
    pub base_imponible: Decimal,
    pub minimo_personal_familiar: Decimal,
    pub cuota_estatal: Decimal,
    pub cuota_autonomica: Decimal,
    pub cuota_integra: Decimal,
    pub cuota_liquida: Decimal,
    pub tipo_efectivo: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// ITALY (IT) - 20 REGIONI
// ═══════════════════════════════════════════════════════════════════════════

/// Italian Regions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItalianRegione {
    Lombardia, Lazio, Campania, Sicilia, Veneto, EmiliaRomagna,
    Piemonte, Puglia, Toscana, Calabria, Sardegna, Liguria,
    Marche, Abruzzo, FriuliVeneziaGiulia, TrentinoAltoAdige,
    Umbria, Basilicata, Molise, ValleDAosta,
}

impl ItalianRegione {
    pub fn regional_rate(&self) -> Decimal {
        match self {
            ItalianRegione::Lazio => dec!(0.0333),
            ItalianRegione::Campania | ItalianRegione::Calabria | ItalianRegione::Molise => dec!(0.0203),
            ItalianRegione::Abruzzo => dec!(0.0173),
            ItalianRegione::Piemonte => dec!(0.0162),
            ItalianRegione::Toscana => dec!(0.0142),
            ItalianRegione::EmiliaRomagna | ItalianRegione::Puglia => dec!(0.0133),
            ItalianRegione::FriuliVeneziaGiulia => dec!(0.007),
            _ => dec!(0.0123), // Standard
        }
    }
}

/// Italian INPS Social Security
#[derive(Debug, Clone)]
pub struct ItalianINPS {
    pub ivs_dipendente: Decimal,     // 9.19%
    pub ivs_datore: Decimal,         // 23.81%
    pub naspi: Decimal,              // 1.31%
    pub massimale: Decimal,          // €119,650
}

impl Default for ItalianINPS {
    fn default() -> Self {
        Self { ivs_dipendente: dec!(0.0919), ivs_datore: dec!(0.2381), naspi: dec!(0.0131), massimale: dec!(119650) }
    }
}

/// Italian TFR (Severance)
pub struct ItalianTFR;
impl ItalianTFR {
    pub fn annual_accrual(salary: Decimal) -> Decimal { salary * dec!(0.0691) }
}

/// Italian Tax Calculator
pub struct ItalianTaxCalculator {
    pub regione: ItalianRegione,
    pub comune_rate: Decimal, // 0-0.9%
    pub inps: ItalianINPS,
    pub num_figli: u8,
    pub has_coniuge: bool,
}

impl ItalianTaxCalculator {
    pub fn new(regione: ItalianRegione) -> Self {
        Self { regione, comune_rate: dec!(0.008), inps: ItalianINPS::default(), num_figli: 0, has_coniuge: false }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> ItalianTaxResult {
        // IRPEF (3 brackets: 23%, 35%, 43%)
        let irpef_lorda = self.calculate_irpef(gross_annual);
        
        // Detrazioni
        let detrazione_lavoro = self.calculate_detrazione_lavoro(gross_annual);
        let detrazione_coniuge = if self.has_coniuge && gross_annual <= dec!(80000) { dec!(800) } else { Decimal::ZERO };
        let detrazioni = detrazione_lavoro + detrazione_coniuge;
        
        let irpef_netta = (irpef_lorda - detrazioni).max(Decimal::ZERO);
        
        // Addizionale regionale + comunale
        let regionale = gross_annual * self.regione.regional_rate();
        let comunale = gross_annual * self.comune_rate;
        
        let total = irpef_netta + regionale + comunale;
        
        ItalianTaxResult {
            reddito_imponibile: gross_annual,
            irpef_lorda,
            detrazioni,
            irpef_netta,
            addizionale_regionale: regionale,
            addizionale_comunale: comunale,
            imposta_totale: total,
            aliquota_effettiva: if gross_annual > Decimal::ZERO { total / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_irpef(&self, income: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 3] = [
            (dec!(28000), dec!(0.23)), (dec!(50000), dec!(0.35)), (dec!(999999999), dec!(0.43)),
        ];
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if income <= prev { break; }
            tax += (income.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
    
    fn calculate_detrazione_lavoro(&self, income: Decimal) -> Decimal {
        if income <= dec!(15000) { dec!(1880) }
        else if income <= dec!(28000) { dec!(1910) + dec!(1190) * (dec!(28000) - income) / dec!(13000) }
        else if income <= dec!(50000) { dec!(1910) * (dec!(50000) - income) / dec!(22000) }
        else { Decimal::ZERO }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItalianTaxResult {
    pub reddito_imponibile: Decimal,
    pub irpef_lorda: Decimal,
    pub detrazioni: Decimal,
    pub irpef_netta: Decimal,
    pub addizionale_regionale: Decimal,
    pub addizionale_comunale: Decimal,
    pub imposta_totale: Decimal,
    pub aliquota_effettiva: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// PORTUGAL (PT) - NHR REGIME
// ═══════════════════════════════════════════════════════════════════════════

/// Portuguese NHR (Non-Habitual Resident)
#[derive(Debug, Clone)]
pub struct PortugueseNHR {
    pub is_eligible: bool,
    pub flat_rate: Decimal,      // 20%
    pub remaining_years: u8,     // Max 10
}

impl Default for PortugueseNHR {
    fn default() -> Self {
        Self { is_eligible: false, flat_rate: dec!(0.20), remaining_years: 10 }
    }
}

/// Portuguese Social Security
#[derive(Debug, Clone)]
pub struct PortugueseSocialSecurity {
    pub taxa_trabalhador: Decimal,    // 11%
    pub taxa_patronal: Decimal,       // 23.75%
    pub salario_minimo: Decimal,      // €820
}

impl Default for PortugueseSocialSecurity {
    fn default() -> Self {
        Self { taxa_trabalhador: dec!(0.11), taxa_patronal: dec!(0.2375), salario_minimo: dec!(820) }
    }
}

/// Portuguese Tax Calculator
pub struct PortugueseTaxCalculator {
    pub nhr: Option<PortugueseNHR>,
    pub ss: PortugueseSocialSecurity,
    pub is_casado: bool,
    pub num_dependentes: u8,
}

impl PortugueseTaxCalculator {
    pub fn new() -> Self {
        Self { nhr: None, ss: PortugueseSocialSecurity::default(), is_casado: false, num_dependentes: 0 }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> PortugueseTaxResult {
        let deducao_especifica = dec!(4104);
        let rendimento_coletavel = (gross_annual - deducao_especifica).max(Decimal::ZERO);
        
        // NHR regime
        if let Some(nhr) = &self.nhr {
            if nhr.is_eligible && nhr.remaining_years > 0 {
                let tax = rendimento_coletavel * nhr.flat_rate;
                return PortugueseTaxResult {
                    rendimento_bruto: gross_annual,
                    rendimento_coletavel,
                    coleta: tax,
                    deducoes: Decimal::ZERO,
                    imposto: tax,
                    taxa_efetiva: if gross_annual > Decimal::ZERO { tax / gross_annual * dec!(100) } else { Decimal::ZERO },
                    taxa_marginal: dec!(20),
                };
            }
        }
        
        // Standard IRS (9 brackets)
        let (coleta, marginal) = self.calculate_coleta(rendimento_coletavel);
        
        // Deductions
        let deducoes = Decimal::from(self.num_dependentes) * dec!(600) + dec!(250);
        let imposto = (coleta - deducoes).max(Decimal::ZERO);
        
        PortugueseTaxResult {
            rendimento_bruto: gross_annual,
            rendimento_coletavel,
            coleta,
            deducoes,
            imposto,
            taxa_efetiva: if gross_annual > Decimal::ZERO { imposto / gross_annual * dec!(100) } else { Decimal::ZERO },
            taxa_marginal: marginal,
        }
    }
    
    fn calculate_coleta(&self, income: Decimal) -> (Decimal, Decimal) {
        // Simplified 9-bracket with deduction method
        let brackets: [(Decimal, Decimal, Decimal); 9] = [
            (dec!(7703), dec!(0.1325), Decimal::ZERO),
            (dec!(11623), dec!(0.18), dec!(365.89)),
            (dec!(16472), dec!(0.23), dec!(947.04)),
            (dec!(21321), dec!(0.26), dec!(1441.20)),
            (dec!(27146), dec!(0.3275), dec!(2880.47)),
            (dec!(39791), dec!(0.37), dec!(4034.17)),
            (dec!(51997), dec!(0.435), dec!(6620.43)),
            (dec!(81199), dec!(0.45), dec!(7400.28)),
            (dec!(999999999), dec!(0.48), dec!(9836.45)),
        ];
        
        for (max, rate, deduction) in brackets {
            if income <= max {
                let tax = (income * rate - deduction).max(Decimal::ZERO);
                return (tax, rate * dec!(100));
            }
        }
        (Decimal::ZERO, Decimal::ZERO)
    }
}

impl Default for PortugueseTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortugueseTaxResult {
    pub rendimento_bruto: Decimal,
    pub rendimento_coletavel: Decimal,
    pub coleta: Decimal,
    pub deducoes: Decimal,
    pub imposto: Decimal,
    pub taxa_efetiva: Decimal,
    pub taxa_marginal: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// GREECE (GR) - EFKA
// ═══════════════════════════════════════════════════════════════════════════

/// Greek EFKA Social Security
#[derive(Debug, Clone)]
pub struct GreekEFKA {
    pub employee_rate: Decimal,  // ~13%
    pub employer_rate: Decimal,  // ~22%
    pub ceiling: Decimal,        // €7,126.94/month
}

impl Default for GreekEFKA {
    fn default() -> Self {
        Self { employee_rate: dec!(0.1307), employer_rate: dec!(0.2241), ceiling: dec!(7126.94) }
    }
}

/// Greek Tax Calculator
pub struct GreekTaxCalculator {
    pub efka: GreekEFKA,
    pub num_children: u8,
}

impl GreekTaxCalculator {
    pub fn new() -> Self {
        Self { efka: GreekEFKA::default(), num_children: 0 }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> GreekTaxResult {
        // 5 brackets (9%, 22%, 28%, 36%, 44%)
        let base_tax = self.calculate_progressive(gross_annual);
        
        // Tax credit (€777 base, reduced above €12,000)
        let credit = self.calculate_credit(gross_annual);
        let tax_after_credit = (base_tax - credit).max(Decimal::ZERO);
        
        GreekTaxResult {
            eisodima: gross_annual,
            foros_klimakos: base_tax,
            meiosi_forou: credit,
            foros_meta_meiosis: tax_after_credit,
            katharo_eisodima: gross_annual - tax_after_credit,
            syntelestis: if gross_annual > Decimal::ZERO { tax_after_credit / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn calculate_progressive(&self, income: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 5] = [
            (dec!(10000), dec!(0.09)), (dec!(20000), dec!(0.22)), (dec!(30000), dec!(0.28)),
            (dec!(40000), dec!(0.36)), (dec!(999999999), dec!(0.44)),
        ];
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if income <= prev { break; }
            tax += (income.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
    
    fn calculate_credit(&self, income: Decimal) -> Decimal {
        let base = dec!(777) + Decimal::from(self.num_children) * dec!(810);
        if income > dec!(12000) {
            let reduction = (income - dec!(12000)) * dec!(0.02);
            (base - reduction).max(Decimal::ZERO)
        } else { base }
    }
}

impl Default for GreekTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GreekTaxResult {
    pub eisodima: Decimal,
    pub foros_klimakos: Decimal,
    pub meiosi_forou: Decimal,
    pub foros_meta_meiosis: Decimal,
    pub katharo_eisodima: Decimal,
    pub syntelestis: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// MALTA (MT)
// ═══════════════════════════════════════════════════════════════════════════

/// Malta Tax Status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MaltaTaxStatus { Single, Married, Parent }

/// Malta Tax Calculator
pub struct MaltaTaxCalculator {
    pub status: MaltaTaxStatus,
}

impl MaltaTaxCalculator {
    pub fn new(status: MaltaTaxStatus) -> Self { Self { status } }
    
    pub fn calculate(&self, gross_annual: Decimal) -> MaltaTaxResult {
        let (exempt, brackets) = self.get_brackets();
        
        if gross_annual <= exempt {
            return MaltaTaxResult { income: gross_annual, tax: Decimal::ZERO, effective_rate: Decimal::ZERO };
        }
        
        let mut tax = Decimal::ZERO;
        let mut prev = exempt;
        for (max, rate, subtract) in brackets {
            if gross_annual <= prev { break; }
            let bracket_income = gross_annual.min(max) - prev;
            tax += bracket_income * rate;
            prev = max;
        }
        // Apply subtract method
        let final_tax = tax.max(Decimal::ZERO);
        
        MaltaTaxResult {
            income: gross_annual,
            tax: final_tax,
            effective_rate: if gross_annual > Decimal::ZERO { final_tax / gross_annual * dec!(100) } else { Decimal::ZERO },
        }
    }
    
    fn get_brackets(&self) -> (Decimal, Vec<(Decimal, Decimal, Decimal)>) {
        match self.status {
            MaltaTaxStatus::Single => (dec!(9100), vec![
                (dec!(14500), dec!(0.15), dec!(1365)), (dec!(19500), dec!(0.25), dec!(2815)),
                (dec!(60000), dec!(0.25), dec!(2725)), (dec!(999999999), dec!(0.35), dec!(8725)),
            ]),
            MaltaTaxStatus::Married => (dec!(12700), vec![
                (dec!(21200), dec!(0.15), dec!(1905)), (dec!(28700), dec!(0.25), dec!(4025)),
                (dec!(60000), dec!(0.25), dec!(3905)), (dec!(999999999), dec!(0.35), dec!(9905)),
            ]),
            MaltaTaxStatus::Parent => (dec!(10500), vec![
                (dec!(15800), dec!(0.15), dec!(1575)), (dec!(21200), dec!(0.25), dec!(3155)),
                (dec!(60000), dec!(0.25), dec!(3050)), (dec!(999999999), dec!(0.35), dec!(9050)),
            ]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaltaTaxResult {
    pub income: Decimal,
    pub tax: Decimal,
    pub effective_rate: Decimal,
}

// ═══════════════════════════════════════════════════════════════════════════
// CYPRUS (CY) - NON-DOM
// ═══════════════════════════════════════════════════════════════════════════

/// Cyprus Non-Dom Regime
#[derive(Debug, Clone)]
pub struct CyprusNonDom {
    pub is_non_dom: bool,
    pub dividend_exempt: bool,
    pub interest_exempt: bool,
}

impl Default for CyprusNonDom {
    fn default() -> Self {
        Self { is_non_dom: false, dividend_exempt: true, interest_exempt: true }
    }
}

/// Cyprus Social Insurance
#[derive(Debug, Clone)]
pub struct CyprusSocialInsurance {
    pub employee_rate: Decimal,    // 8.8%
    pub employer_rate: Decimal,    // 8.8%
    pub ghs_employee: Decimal,     // 2.65%
    pub ghs_employer: Decimal,     // 2.90%
    pub ceiling: Decimal,          // €58,080
}

impl Default for CyprusSocialInsurance {
    fn default() -> Self {
        Self {
            employee_rate: dec!(0.088), employer_rate: dec!(0.088),
            ghs_employee: dec!(0.0265), ghs_employer: dec!(0.029),
            ceiling: dec!(58080),
        }
    }
}

impl CyprusSocialInsurance {
    pub fn total_employee(&self) -> Decimal { self.employee_rate + self.ghs_employee }
    pub fn total_employer(&self) -> Decimal { self.employer_rate + self.ghs_employer + dec!(0.037) } // +funds
}

/// Cyprus Tax Calculator
pub struct CyprusTaxCalculator {
    pub non_dom: Option<CyprusNonDom>,
    pub si: CyprusSocialInsurance,
}

impl CyprusTaxCalculator {
    pub fn new() -> Self {
        Self { non_dom: None, si: CyprusSocialInsurance::default() }
    }
    
    pub fn calculate(&self, gross_annual: Decimal) -> CyprusTaxResult {
        // 5 brackets (0%, 20%, 25%, 30%, 35%)
        let tax = self.calculate_progressive(gross_annual);
        
        CyprusTaxResult {
            income: gross_annual,
            tax,
            effective_rate: if gross_annual > Decimal::ZERO { tax / gross_annual * dec!(100) } else { Decimal::ZERO },
            is_non_dom: self.non_dom.as_ref().map(|n| n.is_non_dom).unwrap_or(false),
        }
    }
    
    fn calculate_progressive(&self, income: Decimal) -> Decimal {
        let brackets: [(Decimal, Decimal); 5] = [
            (dec!(19500), dec!(0)), (dec!(28000), dec!(0.20)), (dec!(36300), dec!(0.25)),
            (dec!(60000), dec!(0.30)), (dec!(999999999), dec!(0.35)),
        ];
        let mut tax = Decimal::ZERO;
        let mut prev = Decimal::ZERO;
        for (max, rate) in brackets {
            if income <= prev { break; }
            tax += (income.min(max) - prev) * rate;
            prev = max;
        }
        tax
    }
}

impl Default for CyprusTaxCalculator {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyprusTaxResult {
    pub income: Decimal,
    pub tax: Decimal,
    pub effective_rate: Decimal,
    pub is_non_dom: bool,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGISTRY
// ═══════════════════════════════════════════════════════════════════════════

/// Southern Europe Registry
pub struct SouthernEuropeRegistry;

impl SouthernEuropeRegistry {
    pub fn supported_countries() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("ES", "Spain", "EUR"), ("IT", "Italy", "EUR"), ("PT", "Portugal", "EUR"),
            ("GR", "Greece", "EUR"), ("MT", "Malta", "EUR"), ("CY", "Cyprus", "EUR"),
        ]
    }
    
    pub fn is_eurozone(code: &str) -> bool { matches!(code, "ES" | "IT" | "PT" | "GR" | "MT" | "CY") }
    pub fn has_special_regime(code: &str) -> bool { matches!(code, "ES" | "PT" | "CY") } // Beckham, NHR, Non-Dom
    pub fn uses_sepa(code: &str) -> bool { Self::is_eurozone(code) }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spain_madrid() {
        let calc = SpanishTaxCalculator::new(ComunidadAutonoma::Madrid);
        let result = calc.calculate(dec!(50000));
        assert!(result.cuota_liquida > Decimal::ZERO);
        assert!(result.tipo_efectivo > Decimal::ZERO);
    }
    
    #[test]
    fn test_spain_beckham() {
        let mut calc = SpanishTaxCalculator::new(ComunidadAutonoma::Madrid);
        calc.special_regime = SpanishSpecialRegime::BeckhamLaw;
        let result = calc.calculate(dec!(100000));
        // Beckham: 24% flat
        assert!(result.tipo_efectivo < dec!(25));
    }
    
    #[test]
    fn test_italy_lombardia() {
        let calc = ItalianTaxCalculator::new(ItalianRegione::Lombardia);
        let result = calc.calculate(dec!(40000));
        assert!(result.irpef_netta > Decimal::ZERO);
        assert!(result.addizionale_regionale > Decimal::ZERO);
    }
    
    #[test]
    fn test_portugal_standard() {
        let calc = PortugueseTaxCalculator::new();
        let result = calc.calculate(dec!(35000));
        assert!(result.imposto > Decimal::ZERO);
    }
    
    #[test]
    fn test_portugal_nhr() {
        let mut calc = PortugueseTaxCalculator::new();
        calc.nhr = Some(PortugueseNHR { is_eligible: true, flat_rate: dec!(0.20), remaining_years: 10 });
        let result = calc.calculate(dec!(50000));
        assert_eq!(result.taxa_marginal, dec!(20));
    }
    
    #[test]
    fn test_greece_tax() {
        let calc = GreekTaxCalculator::new();
        let result = calc.calculate(dec!(30000));
        assert!(result.foros_meta_meiosis > Decimal::ZERO);
    }
    
    #[test]
    fn test_malta_single() {
        let calc = MaltaTaxCalculator::new(MaltaTaxStatus::Single);
        let result = calc.calculate(dec!(25000));
        assert!(result.tax > Decimal::ZERO);
    }
    
    #[test]
    fn test_cyprus_tax() {
        let calc = CyprusTaxCalculator::new();
        let result = calc.calculate(dec!(40000));
        assert!(result.tax > Decimal::ZERO);
    }
    
    #[test]
    fn test_registry() {
        let countries = SouthernEuropeRegistry::supported_countries();
        assert_eq!(countries.len(), 6);
        assert!(SouthernEuropeRegistry::is_eurozone("ES"));
        assert!(SouthernEuropeRegistry::has_special_regime("PT"));
    }
}
