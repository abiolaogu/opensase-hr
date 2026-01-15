//! Payroll Module
//!
//! Nigerian payroll processing with PAYE tax, PenCom pension, and NHF deductions.
//! Extended with West Africa and Southern Africa tax calculators and mobile money.

pub mod models;
pub mod service;
pub mod handlers;
pub mod tax_calculator;
pub mod pension;
pub mod west_africa;
pub mod west_africa_enhanced;
pub mod mobile_money;
pub mod south_africa;

pub use models::*;
pub use service::PayrollService;
pub use tax_calculator::NigerianTaxCalculator;
pub use pension::PensionCalculator;
pub use west_africa::{GhanaTaxCalculator, UemoaTaxCalculator, WestAfricaTaxRegistry};
pub use west_africa_enhanced::{CFAZoneConfig, GhanaEnhancedConfig, LaborLawSummary};
pub use mobile_money::WestAfricaMobileMoneyRegistry;
pub use south_africa::{
    SouthAfricaTaxCalculator, ZimbabweTaxCalculator, 
    ZambiaTaxCalculator, AngolaTaxCalculator, SouthernAfricaRegistry
};



