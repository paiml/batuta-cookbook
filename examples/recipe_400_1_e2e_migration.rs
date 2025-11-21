//! # RECIPE-400-1: End-to-End Microservice Migration
//!
//! **Level:** 400 (Expert)
//! **Estimated Time:** 40 hours
//! **Prerequisites:** All Level 100-300 recipes
//!
//! ## Learning Objectives
//! - Orchestrate complete microservice migration workflows
//! - Analyze legacy service dependencies and architecture
//! - Plan and execute multi-phase migrations
//! - Implement testing and validation strategies
//! - Handle rollback and disaster recovery
//!
//! ## Concepts Covered
//! - End-to-end migration orchestration
//! - Dependency analysis and service mapping
//! - Phased migration strategies (strangler pattern)
//! - Integration testing and smoke tests
//! - Deployment automation and rollback
//! - Health checks and monitoring
//!
//! ## Examples
//! This file demonstrates three migration scenarios:
//! 1. Simple stateless service migration
//! 2. Stateful service with database dependencies
//! 3. Multi-service migration with orchestration

use batuta_cookbook::Result;
use std::collections::HashSet;
use std::path::PathBuf;

/// Migration phase stages
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MigrationPhase {
    /// Discovery and analysis
    Discovery,
    /// Planning and validation
    Planning,
    /// Code transpilation
    Transpilation,
    /// Testing and validation
    Testing,
    /// Deployment preparation
    Deployment,
    /// Production cutover
    Cutover,
    /// Monitoring and validation
    Validation,
    /// Complete
    Complete,
}

impl MigrationPhase {
    pub fn name(&self) -> &str {
        match self {
            Self::Discovery => "Discovery",
            Self::Planning => "Planning",
            Self::Transpilation => "Transpilation",
            Self::Testing => "Testing",
            Self::Deployment => "Deployment",
            Self::Cutover => "Cutover",
            Self::Validation => "Validation",
            Self::Complete => "Complete",
        }
    }
}

/// Service type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceType {
    /// REST API service
    RestApi,
    /// gRPC service
    Grpc,
    /// Message queue consumer
    MessageConsumer,
    /// Background worker
    Worker,
    /// Database service
    Database,
}

/// Service dependency
#[derive(Debug, Clone)]
pub struct ServiceDependency {
    pub service_name: String,
    pub dependency_type: DependencyType,
    pub required: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyType {
    Database,
    Cache,
    MessageQueue,
    ExternalApi,
    InternalService,
}

/// Service specification
#[derive(Debug, Clone)]
pub struct ServiceSpec {
    pub name: String,
    pub service_type: ServiceType,
    pub source_language: String,
    pub target_language: String,
    pub source_path: PathBuf,
    pub dependencies: Vec<ServiceDependency>,
    pub endpoints: Vec<String>,
    pub has_state: bool,
}

impl ServiceSpec {
    pub fn new(name: String, service_type: ServiceType) -> Self {
        Self {
            name,
            service_type,
            source_language: "python".to_string(),
            target_language: "rust".to_string(),
            source_path: PathBuf::from("src"),
            dependencies: Vec::new(),
            endpoints: Vec::new(),
            has_state: false,
        }
    }

    pub fn with_dependency(mut self, dep: ServiceDependency) -> Self {
        self.dependencies.push(dep);
        self
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoints.push(endpoint);
        self
    }

    pub fn with_state(mut self, has_state: bool) -> Self {
        self.has_state = has_state;
        self
    }
}

/// Migration plan
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub service: ServiceSpec,
    pub phases: Vec<MigrationPhase>,
    pub current_phase: usize,
    pub risk_level: RiskLevel,
    pub rollback_strategy: RollbackStrategy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RollbackStrategy {
    /// Immediate rollback on any failure
    Immediate,
    /// Blue-green deployment with traffic switching
    BlueGreen,
    /// Canary deployment with gradual rollout
    Canary,
    /// Feature flags for gradual migration
    FeatureFlags,
}

impl MigrationPlan {
    pub fn new(service: ServiceSpec) -> Self {
        let risk_level = if service.has_state {
            RiskLevel::High
        } else if service.dependencies.iter().any(|d| d.required) {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };

        Self {
            service,
            phases: vec![
                MigrationPhase::Discovery,
                MigrationPhase::Planning,
                MigrationPhase::Transpilation,
                MigrationPhase::Testing,
                MigrationPhase::Deployment,
                MigrationPhase::Cutover,
                MigrationPhase::Validation,
                MigrationPhase::Complete,
            ],
            current_phase: 0,
            risk_level,
            rollback_strategy: RollbackStrategy::BlueGreen,
        }
    }

    pub fn current_phase(&self) -> MigrationPhase {
        self.phases[self.current_phase]
    }

    pub fn advance_phase(&mut self) -> Result<()> {
        if self.current_phase < self.phases.len() - 1 {
            self.current_phase += 1;
            Ok(())
        } else {
            Err(batuta_cookbook::Error::Other(
                "Migration already complete".to_string(),
            ))
        }
    }

    pub fn is_complete(&self) -> bool {
        self.current_phase() == MigrationPhase::Complete
    }

    pub fn progress_percentage(&self) -> f64 {
        (self.current_phase as f64 / (self.phases.len() - 1) as f64) * 100.0
    }
}

/// Migration executor
pub struct MigrationExecutor {
    plans: Vec<MigrationPlan>,
    completed: HashSet<String>,
    failed: HashSet<String>,
}

impl MigrationExecutor {
    pub fn new() -> Self {
        Self {
            plans: Vec::new(),
            completed: HashSet::new(),
            failed: HashSet::new(),
        }
    }

    pub fn add_plan(&mut self, plan: MigrationPlan) {
        self.plans.push(plan);
    }

    pub fn execute_all(&mut self) -> Result<MigrationReport> {
        let start = std::time::Instant::now();
        let mut phase_results = Vec::new();

        let plan_count = self.plans.len();
        for i in 0..plan_count {
            let plan = &mut self.plans[i];
            let service_name = plan.service.name.clone();

            match Self::execute_plan_static(plan) {
                Ok(results) => {
                    self.completed.insert(service_name);
                    phase_results.extend(results);
                }
                Err(e) => {
                    self.failed.insert(service_name);
                    return Err(e);
                }
            }
        }

        Ok(MigrationReport {
            total_services: plan_count,
            completed: self.completed.len(),
            failed: self.failed.len(),
            duration: start.elapsed(),
            phase_results,
        })
    }

    fn execute_plan_static(plan: &mut MigrationPlan) -> Result<Vec<PhaseResult>> {
        let mut results = Vec::new();

        while !plan.is_complete() {
            let phase = plan.current_phase();
            let result = Self::execute_phase_static(plan, phase)?;
            results.push(result);
            plan.advance_phase()?;
        }

        Ok(results)
    }

    fn execute_phase_static(plan: &MigrationPlan, phase: MigrationPhase) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        match phase {
            MigrationPhase::Discovery => Self::execute_discovery(plan),
            MigrationPhase::Planning => Self::execute_planning(plan),
            MigrationPhase::Transpilation => Self::execute_transpilation(plan),
            MigrationPhase::Testing => Self::execute_testing(plan),
            MigrationPhase::Deployment => Self::execute_deployment(plan),
            MigrationPhase::Cutover => Self::execute_cutover(plan),
            MigrationPhase::Validation => Self::execute_validation(plan),
            MigrationPhase::Complete => Ok(PhaseResult {
                phase,
                success: true,
                duration: start.elapsed(),
                message: "Migration complete".to_string(),
            }),
        }
    }

    fn execute_discovery(plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Discover service dependencies
        let dep_count = plan.service.dependencies.len();
        let endpoint_count = plan.service.endpoints.len();

        Ok(PhaseResult {
            phase: MigrationPhase::Discovery,
            success: true,
            duration: start.elapsed(),
            message: format!(
                "Discovered {} dependencies, {} endpoints",
                dep_count, endpoint_count
            ),
        })
    }

    fn execute_planning(plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Validate migration feasibility
        if plan.service.has_state && plan.risk_level == RiskLevel::Critical {
            return Err(batuta_cookbook::Error::Other(
                "High-risk stateful migration requires manual review".to_string(),
            ));
        }

        Ok(PhaseResult {
            phase: MigrationPhase::Planning,
            success: true,
            duration: start.elapsed(),
            message: format!(
                "Migration plan validated, risk level: {:?}",
                plan.risk_level
            ),
        })
    }

    fn execute_transpilation(_plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Simulate transpilation
        // In a real implementation, this would call the actual transpiler

        Ok(PhaseResult {
            phase: MigrationPhase::Transpilation,
            success: true,
            duration: start.elapsed(),
            message: "Service transpiled successfully".to_string(),
        })
    }

    fn execute_testing(plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Run integration tests
        let test_count = plan.service.endpoints.len() * 3; // Simulate 3 tests per endpoint

        Ok(PhaseResult {
            phase: MigrationPhase::Testing,
            success: true,
            duration: start.elapsed(),
            message: format!("{} tests passed", test_count),
        })
    }

    fn execute_deployment(plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Deploy using selected strategy
        let strategy = match plan.rollback_strategy {
            RollbackStrategy::BlueGreen => "blue-green",
            RollbackStrategy::Canary => "canary",
            RollbackStrategy::FeatureFlags => "feature-flags",
            RollbackStrategy::Immediate => "immediate",
        };

        Ok(PhaseResult {
            phase: MigrationPhase::Deployment,
            success: true,
            duration: start.elapsed(),
            message: format!("Deployed using {} strategy", strategy),
        })
    }

    fn execute_cutover(_plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Switch traffic to new service
        Ok(PhaseResult {
            phase: MigrationPhase::Cutover,
            success: true,
            duration: start.elapsed(),
            message: "Traffic cutover complete".to_string(),
        })
    }

    fn execute_validation(plan: &MigrationPlan) -> Result<PhaseResult> {
        let start = std::time::Instant::now();

        // Run health checks
        let health_checks = plan.service.endpoints.len() + 2; // Endpoints + DB + cache

        Ok(PhaseResult {
            phase: MigrationPhase::Validation,
            success: true,
            duration: start.elapsed(),
            message: format!("{} health checks passed", health_checks),
        })
    }
}

impl Default for MigrationExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Phase execution result
#[derive(Debug, Clone)]
pub struct PhaseResult {
    pub phase: MigrationPhase,
    pub success: bool,
    pub duration: std::time::Duration,
    pub message: String,
}

/// Migration report
#[derive(Debug, Clone)]
pub struct MigrationReport {
    pub total_services: usize,
    pub completed: usize,
    pub failed: usize,
    pub duration: std::time::Duration,
    pub phase_results: Vec<PhaseResult>,
}

impl MigrationReport {
    pub fn print_summary(&self) {
        println!("=== Migration Report ===");
        println!("Total Services: {}", self.total_services);
        println!("Completed: {}", self.completed);
        println!("Failed: {}", self.failed);
        println!("Duration: {:?}", self.duration);
        println!("\nPhase Results:");
        for result in &self.phase_results {
            println!(
                "  {:?}: {} ({}ms) - {}",
                result.phase,
                if result.success { "✓" } else { "✗" },
                result.duration.as_millis(),
                result.message
            );
        }
    }
}

//
// Example 1: Simple stateless service migration
//
pub fn example_1_stateless_migration() -> Result<()> {
    println!("=== Example 1: Simple Stateless Service Migration ===\n");

    let service = ServiceSpec::new("user-api".to_string(), ServiceType::RestApi)
        .with_endpoint("/api/users".to_string())
        .with_endpoint("/api/users/:id".to_string())
        .with_dependency(ServiceDependency {
            service_name: "postgres".to_string(),
            dependency_type: DependencyType::Database,
            required: true,
        });

    let plan = MigrationPlan::new(service);
    println!("Service: {}", plan.service.name);
    println!("Risk Level: {:?}", plan.risk_level);
    println!("Rollback Strategy: {:?}", plan.rollback_strategy);
    println!();

    let mut executor = MigrationExecutor::new();
    executor.add_plan(plan);

    let report = executor.execute_all()?;
    report.print_summary();

    Ok(())
}

//
// Example 2: Stateful service migration
//
pub fn example_2_stateful_migration() -> Result<()> {
    println!("\n=== Example 2: Stateful Service Migration ===\n");

    let service = ServiceSpec::new("session-manager".to_string(), ServiceType::Worker)
        .with_state(true)
        .with_dependency(ServiceDependency {
            service_name: "redis".to_string(),
            dependency_type: DependencyType::Cache,
            required: true,
        })
        .with_dependency(ServiceDependency {
            service_name: "postgres".to_string(),
            dependency_type: DependencyType::Database,
            required: true,
        });

    let plan = MigrationPlan::new(service);
    println!("Service: {}", plan.service.name);
    println!("Has State: {}", plan.service.has_state);
    println!("Risk Level: {:?}", plan.risk_level);
    println!("Dependencies: {}", plan.service.dependencies.len());
    println!();

    let mut executor = MigrationExecutor::new();
    executor.add_plan(plan);

    let report = executor.execute_all()?;
    report.print_summary();

    Ok(())
}

//
// Example 3: Multi-service migration
//
pub fn example_3_multi_service_migration() -> Result<()> {
    println!("\n=== Example 3: Multi-Service Migration ===\n");

    let services = vec![
        ServiceSpec::new("gateway".to_string(), ServiceType::RestApi)
            .with_endpoint("/health".to_string())
            .with_endpoint("/api/*".to_string()),
        ServiceSpec::new("auth-service".to_string(), ServiceType::RestApi)
            .with_endpoint("/auth/login".to_string())
            .with_endpoint("/auth/logout".to_string())
            .with_dependency(ServiceDependency {
                service_name: "postgres".to_string(),
                dependency_type: DependencyType::Database,
                required: true,
            }),
        ServiceSpec::new("notification-worker".to_string(), ServiceType::MessageConsumer)
            .with_dependency(ServiceDependency {
                service_name: "rabbitmq".to_string(),
                dependency_type: DependencyType::MessageQueue,
                required: true,
            }),
    ];

    let mut executor = MigrationExecutor::new();

    for service in services {
        let plan = MigrationPlan::new(service);
        println!("Planning migration for: {}", plan.service.name);
        executor.add_plan(plan);
    }

    println!("\nExecuting migrations...\n");
    let report = executor.execute_all()?;
    report.print_summary();

    Ok(())
}

fn main() -> Result<()> {
    example_1_stateless_migration()?;
    example_2_stateful_migration()?;
    example_3_multi_service_migration()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_spec_creation() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        assert_eq!(service.name, "test");
        assert_eq!(service.service_type, ServiceType::RestApi);
        assert!(!service.has_state);
    }

    #[test]
    fn test_service_with_dependencies() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi)
            .with_dependency(ServiceDependency {
                service_name: "db".to_string(),
                dependency_type: DependencyType::Database,
                required: true,
            });

        assert_eq!(service.dependencies.len(), 1);
    }

    #[test]
    fn test_migration_plan_creation() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let plan = MigrationPlan::new(service);

        assert_eq!(plan.current_phase(), MigrationPhase::Discovery);
        assert_eq!(plan.risk_level, RiskLevel::Low);
        assert!(!plan.is_complete());
    }

    #[test]
    fn test_migration_plan_with_state() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::Worker).with_state(true);
        let plan = MigrationPlan::new(service);

        assert_eq!(plan.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_phase_advancement() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let mut plan = MigrationPlan::new(service);

        assert_eq!(plan.current_phase(), MigrationPhase::Discovery);
        plan.advance_phase().unwrap();
        assert_eq!(plan.current_phase(), MigrationPhase::Planning);
    }

    #[test]
    fn test_progress_percentage() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let mut plan = MigrationPlan::new(service);

        assert_eq!(plan.progress_percentage(), 0.0);

        plan.advance_phase().unwrap(); // Planning
        assert!(plan.progress_percentage() > 0.0);
        assert!(plan.progress_percentage() < 100.0);
    }

    #[test]
    fn test_migration_executor_creation() {
        let executor = MigrationExecutor::new();
        assert_eq!(executor.plans.len(), 0);
        assert_eq!(executor.completed.len(), 0);
    }

    #[test]
    fn test_executor_add_plan() {
        let mut executor = MigrationExecutor::new();
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let plan = MigrationPlan::new(service);

        executor.add_plan(plan);
        assert_eq!(executor.plans.len(), 1);
    }

    #[test]
    fn test_simple_migration_execution() {
        let mut executor = MigrationExecutor::new();
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi)
            .with_endpoint("/test".to_string());
        let plan = MigrationPlan::new(service);

        executor.add_plan(plan);
        let report = executor.execute_all().unwrap();

        assert_eq!(report.completed, 1);
        assert_eq!(report.failed, 0);
        assert_eq!(report.total_services, 1);
    }

    #[test]
    fn test_multi_service_execution() {
        let mut executor = MigrationExecutor::new();

        for i in 0..3 {
            let service =
                ServiceSpec::new(format!("service-{}", i), ServiceType::RestApi);
            let plan = MigrationPlan::new(service);
            executor.add_plan(plan);
        }

        let report = executor.execute_all().unwrap();
        assert_eq!(report.completed, 3);
        assert_eq!(report.total_services, 3);
    }

    #[test]
    fn test_risk_level_calculation() {
        let stateless = ServiceSpec::new("stateless".to_string(), ServiceType::RestApi);
        let stateful =
            ServiceSpec::new("stateful".to_string(), ServiceType::Worker).with_state(true);

        let plan1 = MigrationPlan::new(stateless);
        let plan2 = MigrationPlan::new(stateful);

        assert_eq!(plan1.risk_level, RiskLevel::Low);
        assert_eq!(plan2.risk_level, RiskLevel::High);
    }

    #[test]
    fn test_phase_names() {
        assert_eq!(MigrationPhase::Discovery.name(), "Discovery");
        assert_eq!(MigrationPhase::Complete.name(), "Complete");
    }

    #[test]
    fn test_migration_completion() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let mut plan = MigrationPlan::new(service);

        // Advance through all phases
        while !plan.is_complete() {
            plan.advance_phase().unwrap();
        }

        assert!(plan.is_complete());
        assert_eq!(plan.current_phase(), MigrationPhase::Complete);
    }

    #[test]
    fn test_rollback_strategies() {
        let service = ServiceSpec::new("test".to_string(), ServiceType::RestApi);
        let plan = MigrationPlan::new(service);

        // Default should be BlueGreen
        assert_eq!(plan.rollback_strategy, RollbackStrategy::BlueGreen);
    }

    #[test]
    fn test_service_with_multiple_endpoints() {
        let service = ServiceSpec::new("api".to_string(), ServiceType::RestApi)
            .with_endpoint("/users".to_string())
            .with_endpoint("/posts".to_string())
            .with_endpoint("/comments".to_string());

        assert_eq!(service.endpoints.len(), 3);
    }
}
