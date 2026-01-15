//! Central Controller Module
//! 
//! Control plane implementation for distributed SASE architecture:
//! - CentralController: Policy distribution, PoP management, routing
//! - RegionalController: Regional PoP coordination
//! - FailoverManager: High availability with lease-based failover
//! - HealthMonitor: Real-time PoP health tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

// ═══════════════════════════════════════════════════════════════════════════
// CORE TYPES
// ═══════════════════════════════════════════════════════════════════════════

/// Controller Role
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Role {
    Primary = 0,
    Secondary = 1,
    Observer = 2,
}

/// PoP Health Status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool { matches!(self, Self::Healthy | Self::Degraded) }
}

/// Policy Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyType {
    Firewall,
    Routing,
    QoS,
    Security,
    Compliance,
    Custom,
}

/// Region identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Region(pub String);

impl From<&str> for Region { fn from(s: &str) -> Self { Self(s.into()) } }

// ═══════════════════════════════════════════════════════════════════════════
// GLOBAL STATE
// ═══════════════════════════════════════════════════════════════════════════

/// Global controller state
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GlobalState {
    pub policies: HashMap<String, Policy>,
    pub pops: HashMap<String, PopInfo>,
    pub tunnels: HashMap<String, Tunnel>,
    pub routes: HashMap<String, Route>,
}

/// Versioned policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: String,
    pub name: String,
    pub version: u64,
    pub policy_type: PolicyType,
    pub rules: Vec<u8>,  // Serialized rules
    pub tenant_id: String,
    pub target_pops: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// PoP information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopInfo {
    pub id: String,
    pub location: String,
    pub region: String,
    pub health: HealthStatus,
    pub active_connections: u64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub bandwidth_mbps: f64,
    pub last_heartbeat: String,
}

impl PopInfo {
    pub fn latency_to(&self, _client_location: &str) -> f64 {
        // Simplified - would use geo-based calculation
        10.0
    }
}

/// Tunnel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tunnel {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub endpoints: Vec<String>,
    pub client_location: String,
    pub status: TunnelStatus,
    pub bandwidth_limit_mbps: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelStatus { Active, Inactive, Degraded, Failed }

/// Route entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: String,
    pub prefix: String,
    pub next_hop: String,
    pub metric: u32,
    pub pop_id: String,
}

// ═══════════════════════════════════════════════════════════════════════════
// CONTROLLER CONFIG
// ═══════════════════════════════════════════════════════════════════════════

/// Controller configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControllerConfig {
    pub node_id: String,
    pub bind_address: String,
    pub grpc_port: u16,
    pub regions: Vec<Region>,
    pub policy: PolicyConfig,
    pub routing: RoutingConfig,
    pub health: HealthConfig,
}

impl Default for ControllerConfig {
    fn default() -> Self {
        Self {
            node_id: "controller-1".into(),
            bind_address: "0.0.0.0".into(),
            grpc_port: 50051,
            regions: vec![
                Region::from("us-east"), Region::from("us-west"),
                Region::from("eu-west"), Region::from("ap-south"),
            ],
            policy: PolicyConfig::default(),
            routing: RoutingConfig::default(),
            health: HealthConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyConfig {
    pub version_retention: u32,
    pub distribution_timeout_ms: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RoutingConfig {
    pub convergence_timeout_ms: u64,
    pub max_routes_per_pop: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    pub check_interval_ms: u64,
    pub unhealthy_threshold: u32,
    pub healthy_threshold: u32,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self { check_interval_ms: 5000, unhealthy_threshold: 3, healthy_threshold: 2 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// CENTRAL CONTROLLER
// ═══════════════════════════════════════════════════════════════════════════

/// Central Controller - manages global state and policy distribution
pub struct CentralController {
    config: ControllerConfig,
    state: GlobalState,
    regional_controllers: HashMap<Region, RegionalController>,
}

impl CentralController {
    pub fn new(config: ControllerConfig) -> Self {
        let mut regional_controllers = HashMap::new();
        for region in &config.regions {
            regional_controllers.insert(region.clone(), RegionalController::new(region.clone()));
        }
        
        Self { config, state: GlobalState::default(), regional_controllers }
    }
    
    /// Distribute policy to all PoPs
    pub fn distribute_policy(&mut self, policy: Policy) -> Result<(), ControllerError> {
        // Version the policy
        let versioned = self.version_policy(policy);
        
        // Update central state
        self.state.policies.insert(versioned.id.clone(), versioned.clone());
        
        // Distribute to regional controllers
        for rc in self.regional_controllers.values_mut() {
            rc.apply_policy(&versioned)?;
        }
        
        Ok(())
    }
    
    /// Handle PoP failure - reroute traffic
    pub fn handle_pop_failure(&mut self, pop_id: &str) -> Result<(), ControllerError> {
        // Get affected tunnels
        let affected_tunnels: Vec<Tunnel> = self.state.tunnels.values()
            .filter(|t| t.endpoints.contains(&pop_id.to_string()))
            .cloned().collect();
        
        // Reroute each tunnel
        for tunnel in affected_tunnels {
            let new_pop = self.find_nearest_healthy_pop(&tunnel)?;
            self.reroute_tunnel(&tunnel.id, &new_pop)?;
        }
        
        Ok(())
    }
    
    /// Register a new PoP
    pub fn register_pop(&mut self, pop: PopInfo) -> Result<(), ControllerError> {
        self.state.pops.insert(pop.id.clone(), pop);
        Ok(())
    }
    
    /// Process PoP heartbeat
    pub fn process_heartbeat(&mut self, pop_id: &str, status: PopStatus) -> Result<(), ControllerError> {
        if let Some(pop) = self.state.pops.get_mut(pop_id) {
            pop.health = status.health;
            pop.cpu_usage = status.cpu_usage;
            pop.memory_usage = status.memory_usage;
            pop.active_connections = status.active_connections;
            pop.last_heartbeat = chrono::Utc::now().to_rfc3339();
        }
        Ok(())
    }
    
    /// Get current global state
    pub fn get_state(&self) -> &GlobalState { &self.state }
    
    /// Get PoP status
    pub fn get_pop_status(&self, pop_id: &str) -> Option<&PopInfo> {
        self.state.pops.get(pop_id)
    }
    
    fn version_policy(&self, mut policy: Policy) -> Policy {
        policy.version = self.state.policies.get(&policy.id)
            .map(|p| p.version + 1)
            .unwrap_or(1);
        policy.updated_at = chrono::Utc::now().to_rfc3339();
        policy
    }
    
    fn find_nearest_healthy_pop(&self, tunnel: &Tunnel) -> Result<String, ControllerError> {
        let mut candidates: Vec<_> = self.state.pops.values()
            .filter(|p| p.health.is_healthy())
            .map(|p| (p.id.clone(), p.latency_to(&tunnel.client_location)))
            .collect();
        
        candidates.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        
        candidates.first()
            .map(|(id, _)| id.clone())
            .ok_or(ControllerError::NoHealthyPops)
    }
    
    fn reroute_tunnel(&mut self, tunnel_id: &str, new_pop: &str) -> Result<(), ControllerError> {
        if let Some(tunnel) = self.state.tunnels.get_mut(tunnel_id) {
            tunnel.endpoints = vec![new_pop.into()];
            tunnel.status = TunnelStatus::Active;
        }
        Ok(())
    }
}

/// PoP status update
#[derive(Debug, Clone)]
pub struct PopStatus {
    pub health: HealthStatus,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub active_connections: u64,
}

// ═══════════════════════════════════════════════════════════════════════════
// REGIONAL CONTROLLER
// ═══════════════════════════════════════════════════════════════════════════

/// Regional Controller - coordinates PoPs within a region
pub struct RegionalController {
    region: Region,
    pops: Vec<String>,
    active_policies: HashMap<String, Policy>,
}

impl RegionalController {
    pub fn new(region: Region) -> Self {
        Self { region, pops: Vec::new(), active_policies: HashMap::new() }
    }
    
    pub fn apply_policy(&mut self, policy: &Policy) -> Result<(), ControllerError> {
        self.active_policies.insert(policy.id.clone(), policy.clone());
        // In production: push to all PoPs via gRPC
        Ok(())
    }
    
    pub fn register_pop(&mut self, pop_id: String) { self.pops.push(pop_id); }
    
    pub fn get_region(&self) -> &Region { &self.region }
}

// ═══════════════════════════════════════════════════════════════════════════
// FAILOVER MANAGER
// ═══════════════════════════════════════════════════════════════════════════

/// Atomic role wrapper
pub struct AtomicRole(AtomicU8);

impl AtomicRole {
    pub fn new(role: Role) -> Self { Self(AtomicU8::new(role as u8)) }
    pub fn load(&self) -> Role {
        match self.0.load(Ordering::SeqCst) {
            0 => Role::Primary,
            1 => Role::Secondary,
            _ => Role::Observer,
        }
    }
    pub fn store(&self, role: Role) { self.0.store(role as u8, Ordering::SeqCst); }
}

/// Lease for leader election
#[derive(Debug, Clone)]
pub struct Lease {
    pub holder: String,
    pub acquired_at: String,
    pub expires_at: String,
    pub version: u64,
}

/// Peer controller info
#[derive(Debug, Clone)]
pub struct PeerController {
    pub id: String,
    pub address: String,
    pub last_heartbeat: Option<String>,
}

/// Failover Manager for HA
pub struct FailoverManager {
    node_id: String,
    role: AtomicRole,
    peers: Vec<PeerController>,
    lease: Option<Lease>,
    lease_duration_secs: u64,
}

impl FailoverManager {
    pub fn new(node_id: String, peers: Vec<PeerController>) -> Self {
        Self {
            node_id,
            role: AtomicRole::new(Role::Secondary),
            peers,
            lease: None,
            lease_duration_secs: 15,
        }
    }
    
    pub fn get_role(&self) -> Role { self.role.load() }
    
    pub fn is_primary(&self) -> bool { self.role.load() == Role::Primary }
    
    /// Attempt to acquire leadership lease
    pub fn acquire_lease(&mut self) -> Result<(), ControllerError> {
        // In production: use distributed lock (etcd, Consul, etc.)
        let now = chrono::Utc::now();
        self.lease = Some(Lease {
            holder: self.node_id.clone(),
            acquired_at: now.to_rfc3339(),
            expires_at: (now + chrono::Duration::seconds(self.lease_duration_secs as i64)).to_rfc3339(),
            version: 1,
        });
        self.role.store(Role::Primary);
        Ok(())
    }
    
    /// Renew existing lease
    pub fn renew_lease(&mut self) -> Result<(), ControllerError> {
        if let Some(ref mut lease) = self.lease {
            let now = chrono::Utc::now();
            lease.expires_at = (now + chrono::Duration::seconds(self.lease_duration_secs as i64)).to_rfc3339();
            lease.version += 1;
            Ok(())
        } else {
            Err(ControllerError::NoLease)
        }
    }
    
    /// Check if primary is alive
    pub fn primary_alive(&self) -> bool {
        // In production: check heartbeat from primary
        self.peers.iter().any(|p| p.last_heartbeat.is_some())
    }
    
    /// Step down from primary role
    pub fn step_down(&mut self) {
        self.role.store(Role::Secondary);
        self.lease = None;
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// HEALTH MONITOR
// ═══════════════════════════════════════════════════════════════════════════

/// Health Monitor for PoP status tracking
pub struct HealthMonitor {
    config: HealthConfig,
    pop_health: HashMap<String, HealthTracker>,
}

#[derive(Debug, Clone)]
pub struct HealthTracker {
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
    pub current_status: HealthStatus,
}

impl HealthMonitor {
    pub fn new(config: HealthConfig) -> Self {
        Self { config, pop_health: HashMap::new() }
    }
    
    pub fn record_success(&mut self, pop_id: &str) {
        let tracker = self.pop_health.entry(pop_id.into()).or_insert(HealthTracker {
            consecutive_failures: 0, consecutive_successes: 0, current_status: HealthStatus::Unknown,
        });
        
        tracker.consecutive_successes += 1;
        tracker.consecutive_failures = 0;
        
        if tracker.consecutive_successes >= self.config.healthy_threshold {
            tracker.current_status = HealthStatus::Healthy;
        }
    }
    
    pub fn record_failure(&mut self, pop_id: &str) {
        let tracker = self.pop_health.entry(pop_id.into()).or_insert(HealthTracker {
            consecutive_failures: 0, consecutive_successes: 0, current_status: HealthStatus::Unknown,
        });
        
        tracker.consecutive_failures += 1;
        tracker.consecutive_successes = 0;
        
        if tracker.consecutive_failures >= self.config.unhealthy_threshold {
            tracker.current_status = HealthStatus::Unhealthy;
        } else if tracker.consecutive_failures > 0 {
            tracker.current_status = HealthStatus::Degraded;
        }
    }
    
    pub fn get_status(&self, pop_id: &str) -> HealthStatus {
        self.pop_health.get(pop_id).map(|t| t.current_status).unwrap_or(HealthStatus::Unknown)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// ERRORS
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
pub enum ControllerError {
    NoHealthyPops,
    PolicyDistributionFailed(String),
    NoLease,
    ConnectionFailed(String),
}

impl std::fmt::Display for ControllerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoHealthyPops => write!(f, "No healthy PoPs available"),
            Self::PolicyDistributionFailed(e) => write!(f, "Policy distribution failed: {}", e),
            Self::NoLease => write!(f, "No active lease"),
            Self::ConnectionFailed(e) => write!(f, "Connection failed: {}", e),
        }
    }
}

impl std::error::Error for ControllerError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_central_controller() {
        let config = ControllerConfig::default();
        let mut controller = CentralController::new(config);
        
        // Register a PoP
        controller.register_pop(PopInfo {
            id: "pop-us-east-1".into(), location: "Virginia".into(), region: "us-east".into(),
            health: HealthStatus::Healthy, active_connections: 1000, cpu_usage: 45.0,
            memory_usage: 60.0, bandwidth_mbps: 500.0, last_heartbeat: chrono::Utc::now().to_rfc3339(),
        }).unwrap();
        
        assert!(controller.get_pop_status("pop-us-east-1").is_some());
    }
    
    #[test]
    fn test_policy_distribution() {
        let config = ControllerConfig::default();
        let mut controller = CentralController::new(config);
        
        let policy = Policy {
            id: "firewall-1".into(), name: "Default Firewall".into(), version: 0,
            policy_type: PolicyType::Firewall, rules: vec![], tenant_id: "tenant-1".into(),
            target_pops: vec![], created_at: chrono::Utc::now().to_rfc3339(), updated_at: String::new(),
        };
        
        controller.distribute_policy(policy).unwrap();
        assert!(controller.get_state().policies.contains_key("firewall-1"));
    }
    
    #[test]
    fn test_failover_manager() {
        let peers = vec![PeerController { id: "node-2".into(), address: "10.0.0.2:50051".into(), last_heartbeat: None }];
        let mut fm = FailoverManager::new("node-1".into(), peers);
        
        assert_eq!(fm.get_role(), Role::Secondary);
        
        fm.acquire_lease().unwrap();
        assert!(fm.is_primary());
        
        fm.step_down();
        assert_eq!(fm.get_role(), Role::Secondary);
    }
    
    #[test]
    fn test_health_monitor() {
        let config = HealthConfig { check_interval_ms: 5000, unhealthy_threshold: 3, healthy_threshold: 2 };
        let mut monitor = HealthMonitor::new(config);
        
        // Simulate health checks
        monitor.record_success("pop-1");
        assert_eq!(monitor.get_status("pop-1"), HealthStatus::Unknown);
        
        monitor.record_success("pop-1");
        assert_eq!(monitor.get_status("pop-1"), HealthStatus::Healthy);
        
        monitor.record_failure("pop-1");
        assert_eq!(monitor.get_status("pop-1"), HealthStatus::Degraded);
    }
    
    #[test]
    fn test_pop_heartbeat() {
        let config = ControllerConfig::default();
        let mut controller = CentralController::new(config);
        
        controller.register_pop(PopInfo {
            id: "pop-1".into(), location: "NYC".into(), region: "us-east".into(),
            health: HealthStatus::Unknown, active_connections: 0, cpu_usage: 0.0,
            memory_usage: 0.0, bandwidth_mbps: 0.0, last_heartbeat: String::new(),
        }).unwrap();
        
        controller.process_heartbeat("pop-1", PopStatus {
            health: HealthStatus::Healthy, cpu_usage: 50.0, memory_usage: 70.0, active_connections: 500,
        }).unwrap();
        
        let pop = controller.get_pop_status("pop-1").unwrap();
        assert_eq!(pop.health, HealthStatus::Healthy);
        assert_eq!(pop.cpu_usage, 50.0);
    }
    
    #[test]
    fn test_regional_controller() {
        let mut rc = RegionalController::new(Region::from("eu-west"));
        
        rc.register_pop("pop-eu-west-1".into());
        assert_eq!(rc.get_region().0, "eu-west");
    }
}
