//! Payroll Module
//!
//! Multi-region payroll processing with tax engines for 70+ countries.
//! Supports Africa, South America, Middle East, and Western Europe.

pub mod models;
pub mod service;
pub mod handlers;
pub mod tax_calculator;
pub mod pension;
pub mod west_africa;
pub mod west_africa_enhanced;
pub mod mobile_money;
pub mod south_africa;
pub mod africa_mobile_gateway;
pub mod south_america;
pub mod middle_east;
pub mod western_europe;

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
pub use africa_mobile_gateway::{ProviderRouter, AfricaMobileMoneyRegistry};
pub use south_america::{
    BrazilTaxCalculator, ArgentinaTaxCalculator,
    ColombiaTaxCalculator, PeruTaxCalculator, SouthAmericaRegistry
};
pub use middle_east::{
    UAETaxCalculator, SaudiTaxCalculator, 
    IsraelTaxCalculator, MiddleEastRegistry
};
pub use western_europe::{
    SwitzerlandTaxCalculator, AustriaTaxCalculator,
    IrelandTaxCalculator, WesternEuropeRegistry
};






