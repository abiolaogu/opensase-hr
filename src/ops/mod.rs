//! Deployment & Operations Module
//!
//! Health checks, metrics, and deployment configuration for the HR platform.
//! Supports multi-region Kubernetes deployments with observability.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH CHECKS
// ═══════════════════════════════════════════════════════════════════════════

/// Health check status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Component health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub latency_ms: Option<u64>,
}

/// Overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub version: String,
    pub components: Vec<ComponentHealth>,
    pub uptime_seconds: u64,
}

impl SystemHealth {
    pub fn new(version: &str, uptime: u64) -> Self {
        Self {
            status: HealthStatus::Healthy,
            version: version.to_string(),
            components: vec![],
            uptime_seconds: uptime,
        }
    }
    
    pub fn add_component(&mut self, component: ComponentHealth) {
        // Update overall status based on component health
        match component.status {
            HealthStatus::Unhealthy => self.status = HealthStatus::Unhealthy,
            HealthStatus::Degraded if self.status == HealthStatus::Healthy => {
                self.status = HealthStatus::Degraded;
            }
            _ => {}
        }
        self.components.push(component);
    }
    
    pub fn is_ready(&self) -> bool {
        self.status != HealthStatus::Unhealthy
    }
    
    pub fn is_live(&self) -> bool {
        // Basic liveness - can respond
        true
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// METRICS
// ═══════════════════════════════════════════════════════════════════════════

/// Metrics registry
#[derive(Debug, Default)]
pub struct MetricsRegistry {
    counters: HashMap<String, u64>,
    gauges: HashMap<String, f64>,
    histograms: HashMap<String, Vec<f64>>,
}

impl MetricsRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn increment(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }
    
    pub fn set_gauge(&mut self, name: &str, value: f64) {
        self.gauges.insert(name.to_string(), value);
    }
    
    pub fn record_histogram(&mut self, name: &str, value: f64) {
        self.histograms
            .entry(name.to_string())
            .or_insert_with(Vec::new)
            .push(value);
    }
    
    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        
        for (name, value) in &self.counters {
            output.push_str(&format!("# TYPE {} counter\n", name));
            output.push_str(&format!("{} {}\n", name, value));
        }
        
        for (name, value) in &self.gauges {
            output.push_str(&format!("# TYPE {} gauge\n", name));
            output.push_str(&format!("{} {}\n", name, value));
        }
        
        for (name, values) in &self.histograms {
            if !values.is_empty() {
                output.push_str(&format!("# TYPE {} histogram\n", name));
                let sum: f64 = values.iter().sum();
                let count = values.len();
                output.push_str(&format!("{}_count {}\n", name, count));
                output.push_str(&format!("{}_sum {}\n", name, sum));
            }
        }
        
        output
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// DEPLOYMENT CONFIGURATION
// ═══════════════════════════════════════════════════════════════════════════

/// Deployment region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRegion {
    pub code: String,
    pub name: String,
    pub cloud_provider: CloudProvider,
    pub primary_az: String,
    pub secondary_azs: Vec<String>,
}

/// Cloud provider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CloudProvider {
    AWS,
    GCP,
    Azure,
    OnPremise,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub environment: Environment,
    pub regions: Vec<DeploymentRegion>,
    pub replicas: ReplicaConfig,
    pub resources: ResourceConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicaConfig {
    pub api_gateway: u32,
    pub tax_engine: u32,
    pub payment_processor: u32,
    pub calculation_workers: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub cpu_request: String,
    pub cpu_limit: String,
    pub memory_request: String,
    pub memory_limit: String,
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            regions: vec![
                DeploymentRegion {
                    code: "us-east-1".to_string(),
                    name: "US East (N. Virginia)".to_string(),
                    cloud_provider: CloudProvider::AWS,
                    primary_az: "us-east-1a".to_string(),
                    secondary_azs: vec!["us-east-1b".to_string(), "us-east-1c".to_string()],
                },
            ],
            replicas: ReplicaConfig {
                api_gateway: 3,
                tax_engine: 5,
                payment_processor: 3,
                calculation_workers: 10,
            },
            resources: ResourceConfig {
                cpu_request: "500m".to_string(),
                cpu_limit: "2000m".to_string(),
                memory_request: "1Gi".to_string(),
                memory_limit: "4Gi".to_string(),
            },
        }
    }
}

impl DeploymentConfig {
    pub fn production() -> Self {
        Self {
            environment: Environment::Production,
            regions: vec![
                DeploymentRegion {
                    code: "us-east-1".to_string(),
                    name: "US East".to_string(),
                    cloud_provider: CloudProvider::AWS,
                    primary_az: "us-east-1a".to_string(),
                    secondary_azs: vec!["us-east-1b".to_string(), "us-east-1c".to_string()],
                },
                DeploymentRegion {
                    code: "eu-west-1".to_string(),
                    name: "EU West".to_string(),
                    cloud_provider: CloudProvider::AWS,
                    primary_az: "eu-west-1a".to_string(),
                    secondary_azs: vec!["eu-west-1b".to_string(), "eu-west-1c".to_string()],
                },
                DeploymentRegion {
                    code: "ap-southeast-1".to_string(),
                    name: "Asia Pacific".to_string(),
                    cloud_provider: CloudProvider::AWS,
                    primary_az: "ap-southeast-1a".to_string(),
                    secondary_azs: vec!["ap-southeast-1b".to_string()],
                },
            ],
            replicas: ReplicaConfig {
                api_gateway: 5,
                tax_engine: 10,
                payment_processor: 5,
                calculation_workers: 50,
            },
            resources: ResourceConfig {
                cpu_request: "1000m".to_string(),
                cpu_limit: "4000m".to_string(),
                memory_request: "2Gi".to_string(),
                memory_limit: "8Gi".to_string(),
            },
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ALERT DEFINITIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Alert severity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

/// Alert definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertDefinition {
    pub name: String,
    pub severity: AlertSeverity,
    pub expr: String,
    pub duration: String,
    pub summary: String,
    pub description: String,
}

/// Standard payroll platform alerts
pub fn standard_alerts() -> Vec<AlertDefinition> {
    vec![
        AlertDefinition {
            name: "HighErrorRate".to_string(),
            severity: AlertSeverity::Critical,
            expr: r#"sum(rate(http_requests_total{status=~"5.."}[5m])) / sum(rate(http_requests_total[5m])) > 0.01"#.to_string(),
            duration: "5m".to_string(),
            summary: "High error rate detected".to_string(),
            description: "Error rate is above 1%".to_string(),
        },
        AlertDefinition {
            name: "SlowResponseTime".to_string(),
            severity: AlertSeverity::Warning,
            expr: r#"histogram_quantile(0.99, sum(rate(http_request_duration_seconds_bucket[5m])) by (le)) > 2"#.to_string(),
            duration: "5m".to_string(),
            summary: "Slow response times".to_string(),
            description: "P99 latency is above 2 seconds".to_string(),
        },
        AlertDefinition {
            name: "PayrollCalculationBacklog".to_string(),
            severity: AlertSeverity::Warning,
            expr: "payroll_pending_calculations > 10000".to_string(),
            duration: "10m".to_string(),
            summary: "Payroll calculation backlog".to_string(),
            description: "More than 10,000 calculations pending".to_string(),
        },
        AlertDefinition {
            name: "TaxEngineDown".to_string(),
            severity: AlertSeverity::Critical,
            expr: "up{job=\"tax-engine\"} == 0".to_string(),
            duration: "1m".to_string(),
            summary: "Tax engine is down".to_string(),
            description: "Tax calculation service is unavailable".to_string(),
        },
        AlertDefinition {
            name: "PaymentProcessorDown".to_string(),
            severity: AlertSeverity::Critical,
            expr: "up{job=\"payment-processor\"} == 0".to_string(),
            duration: "1m".to_string(),
            summary: "Payment processor is down".to_string(),
            description: "Payment processing service is unavailable".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_system_health() {
        let mut health = SystemHealth::new("1.0.0", 3600);
        
        health.add_component(ComponentHealth {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            latency_ms: Some(5),
        });
        
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.is_ready());
        assert!(health.is_live());
    }
    
    #[test]
    fn test_health_degradation() {
        let mut health = SystemHealth::new("1.0.0", 3600);
        
        health.add_component(ComponentHealth {
            name: "cache".to_string(),
            status: HealthStatus::Degraded,
            message: Some("High latency".to_string()),
            latency_ms: Some(500),
        });
        
        assert_eq!(health.status, HealthStatus::Degraded);
        assert!(health.is_ready()); // Degraded is still ready
    }
    
    #[test]
    fn test_metrics_registry() {
        let mut registry = MetricsRegistry::new();
        
        registry.increment("http_requests_total", 1);
        registry.increment("http_requests_total", 5);
        registry.set_gauge("active_connections", 42.0);
        registry.record_histogram("request_duration_seconds", 0.15);
        registry.record_histogram("request_duration_seconds", 0.25);
        
        let output = registry.export_prometheus();
        assert!(output.contains("http_requests_total 6"));
        assert!(output.contains("active_connections 42"));
    }
    
    #[test]
    fn test_deployment_config() {
        let config = DeploymentConfig::default();
        assert_eq!(config.environment, Environment::Development);
        assert_eq!(config.regions.len(), 1);
        
        let prod = DeploymentConfig::production();
        assert_eq!(prod.environment, Environment::Production);
        assert_eq!(prod.regions.len(), 3);
        assert_eq!(prod.replicas.tax_engine, 10);
    }
    
    #[test]
    fn test_standard_alerts() {
        let alerts = standard_alerts();
        assert!(alerts.len() >= 5);
        
        let critical_count = alerts.iter()
            .filter(|a| a.severity == AlertSeverity::Critical)
            .count();
        assert!(critical_count >= 3);
    }
}
