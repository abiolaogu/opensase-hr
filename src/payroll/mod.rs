//! Payroll Module
//!
//! Nigerian payroll processing with PAYE tax, PenCom pension, and NHF deductions.

pub mod models;
pub mod service;
pub mod handlers;
pub mod tax_calculator;
pub mod pension;

pub use models::*;
pub use service::PayrollService;
pub use tax_calculator::NigerianTaxCalculator;
pub use pension::PensionCalculator;
