//! JWT Authentication

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::rbac::Role;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,           // User ID
    pub tenant_id: String,     // Tenant ID
    pub employee_id: Option<String>,
    pub role: Role,
    pub exp: i64,              // Expiration timestamp
    pub iat: i64,              // Issued at timestamp
    pub jti: String,           // JWT ID (for revocation)
}

impl Claims {
    pub fn new(
        user_id: Uuid,
        tenant_id: Uuid,
        employee_id: Option<Uuid>,
        role: Role,
        expires_in_hours: i64,
    ) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(expires_in_hours);
        
        Self {
            sub: user_id.to_string(),
            tenant_id: tenant_id.to_string(),
            employee_id: employee_id.map(|e| e.to_string()),
            role,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }

    pub fn tenant_uuid(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.tenant_id)
    }

    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

/// Token pair (access + refresh)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

/// JWT Service (mock - in production use jsonwebtoken crate)
#[derive(Debug, Clone)]
pub struct JwtService {
    secret: String,
    access_token_expiry_hours: i64,
    refresh_token_expiry_hours: i64,
}

impl JwtService {
    pub fn new(secret: String) -> Self {
        Self {
            secret,
            access_token_expiry_hours: 1,
            refresh_token_expiry_hours: 24 * 7, // 1 week
        }
    }

    /// Generate token pair (mock implementation)
    /// 
    /// In production, use jsonwebtoken crate:
    /// ```ignore
    /// use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
    /// let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))?;
    /// ```
    pub fn generate_tokens(
        &self,
        user_id: Uuid,
        tenant_id: Uuid,
        employee_id: Option<Uuid>,
        role: Role,
    ) -> TokenPair {
        let access_claims = Claims::new(
            user_id,
            tenant_id,
            employee_id,
            role,
            self.access_token_expiry_hours,
        );
        
        let refresh_claims = Claims::new(
            user_id,
            tenant_id,
            employee_id,
            role,
            self.refresh_token_expiry_hours,
        );

        // Mock tokens - in production, sign with jsonwebtoken
        let access_token = format!("mock_access_{}", access_claims.jti);
        let refresh_token = format!("mock_refresh_{}", refresh_claims.jti);

        TokenPair {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_token_expiry_hours * 3600,
        }
    }

    /// Validate token (mock - returns None for invalid)
    pub fn validate_token(&self, _token: &str) -> Option<Claims> {
        // In production, decode and validate JWT
        // For mock, return None (not authenticated)
        None
    }
}

/// API Key for integrations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub key_hash: String,  // SHA256 hash of the key
    pub permissions: Vec<String>,
    pub last_used_at: Option<chrono::DateTime<Utc>>,
    pub expires_at: Option<chrono::DateTime<Utc>>,
    pub created_at: chrono::DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let claims = Claims::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Role::Employee,
            1,
        );

        assert!(!claims.is_expired());
        assert!(claims.user_id().is_ok());
    }

    #[test]
    fn test_token_generation() {
        let service = JwtService::new("test_secret".to_string());
        
        let tokens = service.generate_tokens(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Some(Uuid::new_v4()),
            Role::Employee,
        );

        assert!(tokens.access_token.starts_with("mock_access_"));
        assert!(tokens.refresh_token.starts_with("mock_refresh_"));
        assert_eq!(tokens.token_type, "Bearer");
    }
}
