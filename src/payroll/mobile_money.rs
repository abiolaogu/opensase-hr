//! West Africa Mobile Money Providers
//! 
//! Payment integrations for:
//! - Nigeria: OPay, PalmPay, Moniepoint, Kuda
//! - Ghana: MTN MoMo, Vodafone Cash, AirtelTigo Money
//! - Francophone: Orange Money, Wave, MTN MoMo

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Mobile money provider trait
pub trait MobileMoneyProvider {
    fn name(&self) -> &str;
    fn country_codes(&self) -> &[&str];
    fn currency(&self) -> &str;
    fn max_transaction_limit(&self) -> Decimal;
    fn fee_structure(&self) -> FeeStructure;
}

/// Fee structure for mobile money
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeeStructure {
    pub flat_fee: Decimal,
    pub percentage_fee: Decimal,
    pub min_fee: Decimal,
    pub max_fee: Decimal,
}

/// West African mobile money provider registry
#[derive(Debug, Clone)]
pub struct WestAfricaMobileMoneyRegistry {
    pub providers: Vec<MobileMoneyProviderInfo>,
}

/// Mobile money provider information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileMoneyProviderInfo {
    pub id: String,
    pub name: String,
    pub countries: Vec<String>,
    pub currency: String,
    pub max_limit: Decimal,
    pub fee_structure: FeeStructure,
    pub ussd_code: Option<String>,
    pub api_available: bool,
}

impl WestAfricaMobileMoneyRegistry {
    pub fn new() -> Self {
        Self {
            providers: Self::initialize_providers(),
        }
    }
    
    fn initialize_providers() -> Vec<MobileMoneyProviderInfo> {
        vec![
            // Nigeria
            MobileMoneyProviderInfo {
                id: "opay_ng".to_string(),
                name: "OPay".to_string(),
                countries: vec!["NG".to_string()],
                currency: "NGN".to_string(),
                max_limit: Decimal::from(5_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::ZERO,
                    percentage_fee: Decimal::from_str_exact("0.005").unwrap(), // 0.5%
                    min_fee: Decimal::from(10),
                    max_fee: Decimal::from(100),
                },
                ussd_code: Some("*955#".to_string()),
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "palmpay_ng".to_string(),
                name: "PalmPay".to_string(),
                countries: vec!["NG".to_string()],
                currency: "NGN".to_string(),
                max_limit: Decimal::from(5_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::ZERO,
                    percentage_fee: Decimal::from_str_exact("0.005").unwrap(),
                    min_fee: Decimal::from(10),
                    max_fee: Decimal::from(100),
                },
                ussd_code: None,
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "moniepoint_ng".to_string(),
                name: "Moniepoint".to_string(),
                countries: vec!["NG".to_string()],
                currency: "NGN".to_string(),
                max_limit: Decimal::from(10_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::from(10),
                    percentage_fee: Decimal::ZERO,
                    min_fee: Decimal::from(10),
                    max_fee: Decimal::from(50),
                },
                ussd_code: None,
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "kuda_ng".to_string(),
                name: "Kuda Bank".to_string(),
                countries: vec!["NG".to_string()],
                currency: "NGN".to_string(),
                max_limit: Decimal::from(5_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::from(10),
                    percentage_fee: Decimal::ZERO,
                    min_fee: Decimal::from(10),
                    max_fee: Decimal::from(25),
                },
                ussd_code: Some("*5555#".to_string()),
                api_available: true,
            },
            
            // Ghana
            MobileMoneyProviderInfo {
                id: "mtn_momo_gh".to_string(),
                name: "MTN Mobile Money".to_string(),
                countries: vec!["GH".to_string()],
                currency: "GHS".to_string(),
                max_limit: Decimal::from(50_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::ZERO,
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(), // 1%
                    min_fee: Decimal::from_str_exact("0.05").unwrap(),
                    max_fee: Decimal::from(50),
                },
                ussd_code: Some("*170#".to_string()),
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "vodafone_cash_gh".to_string(),
                name: "Vodafone Cash".to_string(),
                countries: vec!["GH".to_string()],
                currency: "GHS".to_string(),
                max_limit: Decimal::from(50_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::ZERO,
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(),
                    min_fee: Decimal::from_str_exact("0.05").unwrap(),
                    max_fee: Decimal::from(50),
                },
                ussd_code: Some("*110#".to_string()),
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "airteltigo_gh".to_string(),
                name: "AirtelTigo Money".to_string(),
                countries: vec!["GH".to_string()],
                currency: "GHS".to_string(),
                max_limit: Decimal::from(50_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::ZERO,
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(),
                    min_fee: Decimal::from_str_exact("0.05").unwrap(),
                    max_fee: Decimal::from(50),
                },
                ussd_code: Some("*500#".to_string()),
                api_available: true,
            },
            
            // Francophone West Africa (UEMOA)
            MobileMoneyProviderInfo {
                id: "orange_money".to_string(),
                name: "Orange Money".to_string(),
                countries: vec![
                    "CI".to_string(), "SN".to_string(), "ML".to_string(), 
                    "BF".to_string(), "NE".to_string(), "GN".to_string(),
                ],
                currency: "XOF".to_string(),
                max_limit: Decimal::from(2_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::from(200),
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(),
                    min_fee: Decimal::from(200),
                    max_fee: Decimal::from(5000),
                },
                ussd_code: Some("#144#".to_string()),
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "mtn_momo_ci".to_string(),
                name: "MTN MoMo".to_string(),
                countries: vec!["CI".to_string()],
                currency: "XOF".to_string(),
                max_limit: Decimal::from(2_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::from(200),
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(),
                    min_fee: Decimal::from(200),
                    max_fee: Decimal::from(5000),
                },
                ussd_code: Some("*133#".to_string()),
                api_available: true,
            },
            MobileMoneyProviderInfo {
                id: "wave".to_string(),
                name: "Wave".to_string(),
                countries: vec!["SN".to_string(), "CI".to_string(), "ML".to_string(), "BF".to_string()],
                currency: "XOF".to_string(),
                max_limit: Decimal::from(3_000_000),
                fee_structure: FeeStructure {
                    flat_fee: Decimal::from(100),
                    percentage_fee: Decimal::from_str_exact("0.01").unwrap(),
                    min_fee: Decimal::from(100),
                    max_fee: Decimal::from(3000),
                },
                ussd_code: None, // App-only
                api_available: true,
            },
        ]
    }
    
    /// Get providers for a specific country
    pub fn get_providers_for_country(&self, country_code: &str) -> Vec<&MobileMoneyProviderInfo> {
        self.providers
            .iter()
            .filter(|p| p.countries.iter().any(|c| c == country_code))
            .collect()
    }
    
    /// Get provider by ID
    pub fn get_provider(&self, provider_id: &str) -> Option<&MobileMoneyProviderInfo> {
        self.providers.iter().find(|p| p.id == provider_id)
    }
    
    /// Calculate fee for a transaction
    pub fn calculate_fee(&self, provider_id: &str, amount: Decimal) -> Option<Decimal> {
        let provider = self.get_provider(provider_id)?;
        let fee = &provider.fee_structure;
        
        let calculated = fee.flat_fee + (amount * fee.percentage_fee);
        Some(calculated.max(fee.min_fee).min(fee.max_fee))
    }
}

impl Default for WestAfricaMobileMoneyRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_provider_registry() {
        let registry = WestAfricaMobileMoneyRegistry::new();
        
        // Nigeria should have 4 providers
        let ng_providers = registry.get_providers_for_country("NG");
        assert_eq!(ng_providers.len(), 4);
        
        // Ghana should have 3 providers
        let gh_providers = registry.get_providers_for_country("GH");
        assert_eq!(gh_providers.len(), 3);
        
        // Senegal should have Orange Money and Wave
        let sn_providers = registry.get_providers_for_country("SN");
        assert!(sn_providers.len() >= 2);
    }
    
    #[test]
    fn test_fee_calculation() {
        let registry = WestAfricaMobileMoneyRegistry::new();
        
        // OPay fee for 100,000 NGN
        let fee = registry.calculate_fee("opay_ng", dec!(100_000)).unwrap();
        assert!(fee >= dec!(10) && fee <= dec!(100));
        
        // MTN MoMo Ghana fee for 1000 GHS
        let fee = registry.calculate_fee("mtn_momo_gh", dec!(1000)).unwrap();
        assert!(fee > Decimal::ZERO);
    }
}
