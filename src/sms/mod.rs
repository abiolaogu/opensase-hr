//! SMS and USSD Service Module
//! 
//! Fallback communication channels for emerging markets where:
//! - Connectivity is intermittent or expensive
//! - Feature phones are still common
//! - USSD provides reliable offline access

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// USSD codes by country for HR operations
#[derive(Debug, Clone)]
pub struct UssdCodes {
    pub check_balance: String,
    pub check_attendance: String,
    pub request_leave: String,
    pub view_payslip: String,
    pub emergency_contact: String,
}

/// Country-specific USSD configurations
pub struct UssdRegistry {
    codes: HashMap<String, UssdCodes>,
}

impl UssdRegistry {
    pub fn new() -> Self {
        let mut codes = HashMap::new();
        
        // Nigeria
        codes.insert("NG".to_string(), UssdCodes {
            check_balance: "*400#".to_string(),
            check_attendance: "*400*1#".to_string(),
            request_leave: "*400*2#".to_string(),
            view_payslip: "*400*3#".to_string(),
            emergency_contact: "*400*9#".to_string(),
        });
        
        // Kenya
        codes.insert("KE".to_string(), UssdCodes {
            check_balance: "*401#".to_string(),
            check_attendance: "*401*1#".to_string(),
            request_leave: "*401*2#".to_string(),
            view_payslip: "*401*3#".to_string(),
            emergency_contact: "*401*9#".to_string(),
        });
        
        // Ghana
        codes.insert("GH".to_string(), UssdCodes {
            check_balance: "*402#".to_string(),
            check_attendance: "*402*1#".to_string(),
            request_leave: "*402*2#".to_string(),
            view_payslip: "*402*3#".to_string(),
            emergency_contact: "*402*9#".to_string(),
        });
        
        // Côte d'Ivoire
        codes.insert("CI".to_string(), UssdCodes {
            check_balance: "*403#".to_string(),
            check_attendance: "*403*1#".to_string(),
            request_leave: "*403*2#".to_string(),
            view_payslip: "*403*3#".to_string(),
            emergency_contact: "*403*9#".to_string(),
        });
        
        // Senegal
        codes.insert("SN".to_string(), UssdCodes {
            check_balance: "*404#".to_string(),
            check_attendance: "*404*1#".to_string(),
            request_leave: "*404*2#".to_string(),
            view_payslip: "*404*3#".to_string(),
            emergency_contact: "*404*9#".to_string(),
        });
        
        Self { codes }
    }
    
    pub fn get_codes(&self, country_code: &str) -> Option<&UssdCodes> {
        self.codes.get(country_code)
    }
    
    pub fn build_attendance_ussd(&self, country_code: &str, employee_id: &str) -> Option<String> {
        let codes = self.get_codes(country_code)?;
        Some(format!("{}*{}", codes.check_attendance.trim_end_matches('#'), employee_id))
    }
    
    pub fn build_leave_request_ussd(
        &self, 
        country_code: &str, 
        employee_id: &str,
        leave_type: u8,
        start_date: &str,
        end_date: &str,
    ) -> Option<String> {
        let codes = self.get_codes(country_code)?;
        Some(format!(
            "{}*{}*{}*{}*{}#", 
            codes.request_leave.trim_end_matches('#'),
            employee_id,
            leave_type,
            start_date,
            end_date
        ))
    }
}

impl Default for UssdRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// SMS notification templates
#[derive(Debug, Clone)]
pub struct SmsTemplates {
    pub payslip_ready: String,
    pub leave_approved: String,
    pub leave_rejected: String,
    pub attendance_reminder: String,
    pub salary_credit: String,
}

/// Localized SMS templates
pub struct SmsTemplateRegistry {
    templates: HashMap<String, SmsTemplates>,
}

impl SmsTemplateRegistry {
    pub fn new() -> Self {
        let mut templates = HashMap::new();
        
        // English
        templates.insert("en".to_string(), SmsTemplates {
            payslip_ready: "OpenSASE: Your payslip for {period} is ready. Net: {currency}{amount}. View in app or dial {ussd}".to_string(),
            leave_approved: "OpenSASE: Your {leave_type} leave from {start_date} to {end_date} has been APPROVED.".to_string(),
            leave_rejected: "OpenSASE: Your {leave_type} leave request has been REJECTED. Reason: {reason}".to_string(),
            attendance_reminder: "OpenSASE: Reminder to check in. Dial {ussd} or open the app.".to_string(),
            salary_credit: "OpenSASE: Salary of {currency}{amount} credited for {period}.".to_string(),
        });
        
        // French
        templates.insert("fr".to_string(), SmsTemplates {
            payslip_ready: "OpenSASE: Bulletin de paie pour {period} prêt. Net: {currency}{amount}. Composez {ussd}".to_string(),
            leave_approved: "OpenSASE: Votre congé {leave_type} du {start_date} au {end_date} a été APPROUVÉ.".to_string(),
            leave_rejected: "OpenSASE: Demande de congé refusée. Raison: {reason}".to_string(),
            attendance_reminder: "OpenSASE: Rappel pointage. Composez {ussd}.".to_string(),
            salary_credit: "OpenSASE: Salaire de {currency}{amount} crédité pour {period}.".to_string(),
        });
        
        // Hausa
        templates.insert("ha".to_string(), SmsTemplates {
            payslip_ready: "OpenSASE: Takardar albashin {period} ya shirya. Net: {currency}{amount}. Danna {ussd}".to_string(),
            leave_approved: "OpenSASE: An amince da hutu {leave_type} daga {start_date} zuwa {end_date}.".to_string(),
            leave_rejected: "OpenSASE: An ki bukatar hutu. Dalili: {reason}".to_string(),
            attendance_reminder: "OpenSASE: Tunatarwa don shiga aiki. Danna {ussd}.".to_string(),
            salary_credit: "OpenSASE: An saka albashi na {currency}{amount} domin {period}.".to_string(),
        });
        
        // Yoruba
        templates.insert("yo".to_string(), SmsTemplates {
            payslip_ready: "OpenSASE: Owo isẹ rẹ fun {period} ti ṣetan. Net: {currency}{amount}. Tẹ {ussd}".to_string(),
            leave_approved: "OpenSASE: Isinmi {leave_type} rẹ lati {start_date} si {end_date} ti FỌWỌSI.".to_string(),
            leave_rejected: "OpenSASE: A ko ibeere isinmi. Idi: {reason}".to_string(),
            attendance_reminder: "OpenSASE: Ranti lati check in. Tẹ {ussd}.".to_string(),
            salary_credit: "OpenSASE: Owo oya {currency}{amount} ti wọle fun {period}.".to_string(),
        });
        
        // Swahili
        templates.insert("sw".to_string(), SmsTemplates {
            payslip_ready: "OpenSASE: Sliipu ya {period} ipo tayari. Net: {currency}{amount}. Piga {ussd}".to_string(),
            leave_approved: "OpenSASE: Likizo {leave_type} kuanzia {start_date} hadi {end_date} IMEKUBALIWA.".to_string(),
            leave_rejected: "OpenSASE: Ombi la likizo LIMEKATALIWA. Sababu: {reason}".to_string(),
            attendance_reminder: "OpenSASE: Ukumbusho kuingia kazini. Piga {ussd}.".to_string(),
            salary_credit: "OpenSASE: Mshahara wa {currency}{amount} umewekwa kwa {period}.".to_string(),
        });
        
        Self { templates }
    }
    
    pub fn get_templates(&self, language: &str) -> &SmsTemplates {
        self.templates.get(language).unwrap_or_else(|| self.templates.get("en").unwrap())
    }
    
    pub fn format_payslip_sms(
        &self, 
        language: &str, 
        period: &str, 
        currency: &str, 
        amount: &str,
        ussd: &str,
    ) -> String {
        self.get_templates(language)
            .payslip_ready
            .replace("{period}", period)
            .replace("{currency}", currency)
            .replace("{amount}", amount)
            .replace("{ussd}", ussd)
    }
    
    pub fn format_leave_approved_sms(
        &self,
        language: &str,
        leave_type: &str,
        start_date: &str,
        end_date: &str,
    ) -> String {
        self.get_templates(language)
            .leave_approved
            .replace("{leave_type}", leave_type)
            .replace("{start_date}", start_date)
            .replace("{end_date}", end_date)
    }
    
    pub fn format_salary_credit_sms(
        &self,
        language: &str,
        currency: &str,
        amount: &str,
        period: &str,
    ) -> String {
        self.get_templates(language)
            .salary_credit
            .replace("{currency}", currency)
            .replace("{amount}", amount)
            .replace("{period}", period)
    }
}

impl Default for SmsTemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Sync status for offline-first operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    Synced,
    Pending,
    Error,
    Conflict,
}

/// Pending operation for offline queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingOperation {
    pub id: String,
    pub operation_type: OperationType,
    pub entity_type: String,
    pub entity_id: String,
    pub payload: serde_json::Value,
    pub created_at: i64,
    pub retry_count: u32,
    pub last_error: Option<String>,
}

/// Operation types for sync queue
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OperationType {
    Create,
    Update,
    Delete,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ussd_registry() {
        let registry = UssdRegistry::new();
        
        // Nigeria codes should exist
        let ng_codes = registry.get_codes("NG").unwrap();
        assert_eq!(ng_codes.check_balance, "*400#");
        
        // Build attendance USSD
        let ussd = registry.build_attendance_ussd("NG", "EMP001").unwrap();
        assert!(ussd.contains("EMP001"));
        
        // Build leave request USSD
        let leave_ussd = registry.build_leave_request_ussd("GH", "EMP002", 1, "20240201", "20240205").unwrap();
        assert!(leave_ussd.contains("EMP002"));
    }
    
    #[test]
    fn test_sms_templates() {
        let registry = SmsTemplateRegistry::new();
        
        // English payslip SMS
        let sms = registry.format_payslip_sms("en", "Jan 2024", "₦", "500,000", "*400*3#");
        assert!(sms.contains("Jan 2024"));
        assert!(sms.contains("₦500,000"));
        
        // French leave approval
        let sms = registry.format_leave_approved_sms("fr", "Annuel", "01/02/2024", "05/02/2024");
        assert!(sms.contains("APPROUVÉ"));
        
        // Fallback to English
        let sms = registry.format_payslip_sms("unknown", "Jan 2024", "₦", "500,000", "*400*3#");
        assert!(sms.contains("OpenSASE"));
    }
}
