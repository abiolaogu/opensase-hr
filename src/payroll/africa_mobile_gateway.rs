//! Africa Mobile Money Gateway
//! 
//! Unified payment gateway for 50+ African mobile money providers:
//! - Tier 1: M-PESA, MTN MoMo, Orange Money, Airtel Money, Wave
//! - Tier 2: EcoCash, Telebirr, Flutterwave, Paystack
//! - Smart routing by country and phone prefix

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transaction state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionState {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Reversed,
}

/// Payment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentRequest {
    pub id: String,
    pub external_id: String,
    pub amount: Decimal,
    pub currency: String,
    pub phone_number: String,
    pub recipient_name: String,
    pub country: String,
    pub provider: Option<String>,
    pub description: String,
    pub reference: String,
    pub callback_url: Option<String>,
    pub metadata: HashMap<String, String>,
}

/// Payment response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentResponse {
    pub transaction_id: String,
    pub provider_ref: String,
    pub status: TransactionState,
    pub amount: Decimal,
    pub currency: String,
    pub fees: Decimal,
    pub provider_message: String,
}

/// Provider routing table by country
pub struct ProviderRouter {
    country_providers: HashMap<String, Vec<String>>,
    prefix_routes: HashMap<String, HashMap<String, String>>,
}

impl ProviderRouter {
    pub fn new() -> Self {
        let mut country_providers = HashMap::new();
        let mut prefix_routes = HashMap::new();
        
        // East Africa
        country_providers.insert("KE".to_string(), vec!["MPESA_KE".to_string(), "AIRTEL_KE".to_string()]);
        country_providers.insert("TZ".to_string(), vec!["MPESA_TZ".to_string(), "TIGOPESA".to_string()]);
        country_providers.insert("UG".to_string(), vec!["MTN_UG".to_string(), "AIRTEL_UG".to_string()]);
        country_providers.insert("RW".to_string(), vec!["MTN_RW".to_string(), "AIRTEL_RW".to_string()]);
        country_providers.insert("ET".to_string(), vec!["TELEBIRR".to_string()]);
        
        // West Africa
        country_providers.insert("NG".to_string(), vec!["FLUTTERWAVE".to_string(), "PAYSTACK".to_string(), "OPAY".to_string()]);
        country_providers.insert("GH".to_string(), vec!["MTN_GH".to_string(), "VODAFONE_GH".to_string()]);
        country_providers.insert("SN".to_string(), vec!["WAVE".to_string(), "ORANGE_SN".to_string()]);
        country_providers.insert("CI".to_string(), vec!["WAVE".to_string(), "ORANGE_CI".to_string(), "MTN_CI".to_string()]);
        country_providers.insert("ML".to_string(), vec!["ORANGE_ML".to_string(), "MOOV_ML".to_string()]);
        country_providers.insert("BF".to_string(), vec!["ORANGE_BF".to_string(), "MOOV_BF".to_string()]);
        country_providers.insert("CM".to_string(), vec!["MTN_CM".to_string(), "ORANGE_CM".to_string()]);
        
        // Southern Africa
        country_providers.insert("ZA".to_string(), vec!["FLUTTERWAVE".to_string(), "PAYSTACK".to_string()]);
        country_providers.insert("ZW".to_string(), vec!["ECOCASH".to_string(), "ONEMONEY".to_string()]);
        country_providers.insert("ZM".to_string(), vec!["MTN_ZM".to_string(), "AIRTEL_ZM".to_string()]);
        country_providers.insert("MZ".to_string(), vec!["MPESA_MZ".to_string(), "EMOLA".to_string()]);
        
        // North Africa
        country_providers.insert("EG".to_string(), vec!["VODAFONE_EG".to_string(), "FAWRY".to_string()]);
        country_providers.insert("MA".to_string(), vec!["ORANGE_MA".to_string(), "INWI_MA".to_string()]);
        
        // Kenya prefix routing (example)
        let mut ke_prefixes = HashMap::new();
        for prefix in ["0700", "0701", "0702", "0703", "0704", "0705", "0706", "0707", "0708", 
                       "0709", "0710", "0711", "0712", "0713", "0714", "0715", "0716", "0717",
                       "0718", "0719", "0720", "0721", "0722", "0723", "0724", "0725", "0726",
                       "0727", "0728", "0729", "0790", "0791", "0792", "0793", "0794", "0795"] {
            ke_prefixes.insert(prefix.to_string(), "MPESA_KE".to_string());
        }
        for prefix in ["0730", "0731", "0732", "0733", "0734", "0735", "0736", "0737", "0738",
                       "0739", "0750", "0751", "0752", "0753", "0754", "0755", "0756", "0780"] {
            ke_prefixes.insert(prefix.to_string(), "AIRTEL_KE".to_string());
        }
        prefix_routes.insert("KE".to_string(), ke_prefixes);
        
        // Ghana prefix routing
        let mut gh_prefixes = HashMap::new();
        for prefix in ["024", "025", "053", "054", "055", "059"] {
            gh_prefixes.insert(prefix.to_string(), "MTN_GH".to_string());
        }
        for prefix in ["020", "050"] {
            gh_prefixes.insert(prefix.to_string(), "VODAFONE_GH".to_string());
        }
        for prefix in ["026", "056", "027", "057"] {
            gh_prefixes.insert(prefix.to_string(), "AIRTEL_GH".to_string());
        }
        prefix_routes.insert("GH".to_string(), gh_prefixes);
        
        Self { country_providers, prefix_routes }
    }
    
    /// Route payment to optimal provider
    pub fn route(&self, country: &str, phone: &str, preferred: Option<&str>) -> Result<String, String> {
        // If specific provider requested, use it
        if let Some(p) = preferred {
            return Ok(p.to_string());
        }
        
        // Try prefix-based routing
        if let Some(prefixes) = self.prefix_routes.get(country) {
            let normalized = self.normalize_phone(phone, country);
            for (prefix, provider) in prefixes {
                if normalized.starts_with(prefix) {
                    return Ok(provider.clone());
                }
            }
        }
        
        // Fallback to first available provider
        if let Some(providers) = self.country_providers.get(country) {
            if let Some(first) = providers.first() {
                return Ok(first.clone());
            }
        }
        
        Err(format!("No provider available for country: {}", country))
    }
    
    /// Get all providers for a country
    pub fn get_providers(&self, country: &str) -> Vec<&str> {
        self.country_providers
            .get(country)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    
    fn normalize_phone(&self, phone: &str, country: &str) -> String {
        let prefixes: HashMap<&str, &str> = [
            ("KE", "+254"), ("GH", "+233"), ("NG", "+234"),
            ("TZ", "+255"), ("UG", "+256"), ("RW", "+250"),
            ("SN", "+221"), ("CI", "+225"), ("ZA", "+27"),
        ].into_iter().collect();
        
        if let Some(prefix) = prefixes.get(country) {
            if phone.starts_with(prefix) {
                return format!("0{}", &phone[prefix.len()..]);
            }
        }
        phone.to_string()
    }
}

impl Default for ProviderRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Supported countries registry
pub struct AfricaMobileMoneyRegistry;

impl AfricaMobileMoneyRegistry {
    /// Get all supported countries with providers
    pub fn supported_countries() -> Vec<(&'static str, &'static str, Vec<&'static str>)> {
        vec![
            // East Africa
            ("KE", "Kenya", vec!["M-PESA", "Airtel Money"]),
            ("TZ", "Tanzania", vec!["M-PESA", "TigoPesa", "Airtel Money"]),
            ("UG", "Uganda", vec!["MTN MoMo", "Airtel Money"]),
            ("RW", "Rwanda", vec!["MTN MoMo", "Airtel Money"]),
            ("ET", "Ethiopia", vec!["Telebirr"]),
            // West Africa
            ("NG", "Nigeria", vec!["Flutterwave", "Paystack", "OPay"]),
            ("GH", "Ghana", vec!["MTN MoMo", "Vodafone Cash", "AirtelTigo"]),
            ("SN", "Senegal", vec!["Wave", "Orange Money"]),
            ("CI", "Côte d'Ivoire", vec!["Wave", "Orange Money", "MTN MoMo"]),
            ("CM", "Cameroon", vec!["MTN MoMo", "Orange Money"]),
            // Southern Africa
            ("ZA", "South Africa", vec!["Flutterwave", "Paystack"]),
            ("ZW", "Zimbabwe", vec!["EcoCash", "OneMoney"]),
            ("ZM", "Zambia", vec!["MTN MoMo", "Airtel Money"]),
            ("MZ", "Mozambique", vec!["M-PESA", "e-Mola"]),
            // North Africa
            ("EG", "Egypt", vec!["Vodafone Cash", "Fawry"]),
            ("MA", "Morocco", vec!["Orange Money", "Inwi"]),
        ]
    }
    
    /// Check if country uses aggregator (no direct mobile money)
    pub fn uses_aggregator(country_code: &str) -> bool {
        matches!(country_code, "ZA" | "NG")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_provider_router() {
        let router = ProviderRouter::new();
        
        // Kenya M-PESA routing
        let provider = router.route("KE", "+254712345678", None).unwrap();
        assert_eq!(provider, "MPESA_KE");
        
        // Ghana MTN routing
        let provider = router.route("GH", "+233241234567", None).unwrap();
        assert_eq!(provider, "MTN_GH");
        
        // Nigeria (aggregator)
        let provider = router.route("NG", "+2348012345678", None).unwrap();
        assert_eq!(provider, "FLUTTERWAVE");
    }
    
    #[test]
    fn test_preferred_provider() {
        let router = ProviderRouter::new();
        
        // Explicit provider selection
        let provider = router.route("KE", "+254712345678", Some("AIRTEL_KE")).unwrap();
        assert_eq!(provider, "AIRTEL_KE");
    }
    
    #[test]
    fn test_registry() {
        let countries = AfricaMobileMoneyRegistry::supported_countries();
        assert!(countries.len() >= 16);
        
        assert!(AfricaMobileMoneyRegistry::uses_aggregator("ZA"));
        assert!(AfricaMobileMoneyRegistry::uses_aggregator("NG"));
        assert!(!AfricaMobileMoneyRegistry::uses_aggregator("KE"));
    }
}
