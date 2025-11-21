//! # RECIPE-400-2: Production Pipeline Integration
//!
//! **Level:** 400 (Expert)
//! **Estimated Time:** 36 hours
//! **Priority:** P1 (Tracer Bullet - per Toyota Way review)
//! **Prerequisites:** All Level 100-300 recipes, RECIPE-400-1
//!
//! ## Learning Objectives
//! - Integrate Batuta into CI/CD pipelines
//! - Implement automated transpilation workflows
//! - Configure quality gates and validation
//! - Set up production deployment automation
//! - Implement rollback and monitoring strategies
//!
//! ## Concepts Covered
//! - CI/CD pipeline configuration (GitHub Actions, GitLab CI)
//! - Automated testing and validation
//! - Quality gates and code coverage thresholds
//! - Artifact management and versioning
//! - Deployment strategies (blue-green, canary)
//! - Monitoring and alerting integration
//!
//! ## Examples
//! This file demonstrates three pipeline configurations:
//! 1. Basic CI pipeline with transpilation and testing
//! 2. Advanced CD pipeline with quality gates
//! 3. Production pipeline with monitoring integration

use batuta_cookbook::Result;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// CI/CD platform types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CiPlatform {
    GitHubActions,
    GitLabCi,
    Jenkins,
    CircleCi,
}

/// Pipeline stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PipelineStage {
    /// Source code checkout
    Checkout,
    /// Dependency installation
    Dependencies,
    /// Code linting and formatting
    Lint,
    /// Transpilation
    Transpile,
    /// Unit testing
    Test,
    /// Integration testing
    IntegrationTest,
    /// Quality gates check
    QualityGates,
    /// Build artifacts
    Build,
    /// Deploy to staging
    DeployStaging,
    /// Smoke tests
    SmokeTest,
    /// Deploy to production
    DeployProduction,
    /// Post-deployment validation
    Validation,
}

impl PipelineStage {
    pub fn name(&self) -> &str {
        match self {
            Self::Checkout => "checkout",
            Self::Dependencies => "dependencies",
            Self::Lint => "lint",
            Self::Transpile => "transpile",
            Self::Test => "test",
            Self::IntegrationTest => "integration-test",
            Self::QualityGates => "quality-gates",
            Self::Build => "build",
            Self::DeployStaging => "deploy-staging",
            Self::SmokeTest => "smoke-test",
            Self::DeployProduction => "deploy-production",
            Self::Validation => "validation",
        }
    }
}

/// Quality gate thresholds
#[derive(Debug, Clone)]
pub struct QualityGates {
    pub min_test_coverage: f64,
    pub min_mutation_score: f64,
    pub max_complexity: usize,
    pub max_duplication: f64,
    pub require_all_tests_pass: bool,
}

impl Default for QualityGates {
    fn default() -> Self {
        Self {
            min_test_coverage: 80.0,
            min_mutation_score: 70.0,
            max_complexity: 15,
            max_duplication: 5.0,
            require_all_tests_pass: true,
        }
    }
}

impl QualityGates {
    pub fn strict() -> Self {
        Self {
            min_test_coverage: 90.0,
            min_mutation_score: 80.0,
            max_complexity: 10,
            max_duplication: 3.0,
            require_all_tests_pass: true,
        }
    }

    pub fn relaxed() -> Self {
        Self {
            min_test_coverage: 70.0,
            min_mutation_score: 60.0,
            max_complexity: 20,
            max_duplication: 10.0,
            require_all_tests_pass: false,
        }
    }
}

/// Pipeline configuration
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub name: String,
    pub platform: CiPlatform,
    pub stages: Vec<PipelineStage>,
    pub quality_gates: QualityGates,
    pub environment_variables: HashMap<String, String>,
    pub timeout_minutes: u32,
}

impl PipelineConfig {
    pub fn new(name: String, platform: CiPlatform) -> Self {
        Self {
            name,
            platform,
            stages: vec![
                PipelineStage::Checkout,
                PipelineStage::Dependencies,
                PipelineStage::Lint,
                PipelineStage::Transpile,
                PipelineStage::Test,
                PipelineStage::Build,
            ],
            quality_gates: QualityGates::default(),
            environment_variables: HashMap::new(),
            timeout_minutes: 30,
        }
    }

    pub fn with_stage(mut self, stage: PipelineStage) -> Self {
        if !self.stages.contains(&stage) {
            self.stages.push(stage);
            self.stages.sort();
        }
        self
    }

    pub fn with_quality_gates(mut self, gates: QualityGates) -> Self {
        self.quality_gates = gates;
        self
    }

    pub fn with_env(mut self, key: String, value: String) -> Self {
        self.environment_variables.insert(key, value);
        self
    }

    /// Generate pipeline configuration file content
    pub fn generate_config(&self) -> String {
        match self.platform {
            CiPlatform::GitHubActions => self.generate_github_actions(),
            CiPlatform::GitLabCi => self.generate_gitlab_ci(),
            CiPlatform::Jenkins => self.generate_jenkinsfile(),
            CiPlatform::CircleCi => self.generate_circleci(),
        }
    }

    fn generate_github_actions(&self) -> String {
        let mut config = format!(
            "name: {}\n\non:\n  push:\n    branches: [ main ]\n  pull_request:\n    branches: [ main ]\n\njobs:\n  pipeline:\n    runs-on: ubuntu-latest\n    timeout-minutes: {}\n    steps:\n",
            self.name, self.timeout_minutes
        );

        for stage in &self.stages {
            config.push_str(&format!("      - name: {}\n", stage.name()));
            config.push_str("        run: |\n");

            match stage {
                PipelineStage::Checkout => {
                    config.push_str("          echo 'Checking out code...'\n");
                }
                PipelineStage::Dependencies => {
                    config.push_str("          cargo build --release\n");
                }
                PipelineStage::Lint => {
                    config.push_str("          cargo clippy -- -D warnings\n");
                    config.push_str("          cargo fmt -- --check\n");
                }
                PipelineStage::Transpile => {
                    config.push_str("          cargo run --example transpile_all\n");
                }
                PipelineStage::Test => {
                    config.push_str("          cargo test --all-features\n");
                }
                PipelineStage::QualityGates => {
                    config.push_str(&format!(
                        "          # Check coverage >= {}%\n",
                        self.quality_gates.min_test_coverage
                    ));
                }
                PipelineStage::Build => {
                    config.push_str("          cargo build --release\n");
                }
                _ => {
                    config.push_str(&format!("          echo 'Running {}...'\n", stage.name()));
                }
            }
            config.push('\n');
        }

        config
    }

    fn generate_gitlab_ci(&self) -> String {
        let mut config = String::from("stages:\n");

        // Group stages
        let mut seen_stages = std::collections::HashSet::new();
        for stage in &self.stages {
            let stage_group = match stage {
                PipelineStage::Checkout | PipelineStage::Dependencies => "build",
                PipelineStage::Lint => "lint",
                PipelineStage::Transpile | PipelineStage::Build => "build",
                PipelineStage::Test | PipelineStage::IntegrationTest => "test",
                PipelineStage::QualityGates => "quality",
                _ => "deploy",
            };
            if !seen_stages.contains(stage_group) {
                config.push_str(&format!("  - {}\n", stage_group));
                seen_stages.insert(stage_group);
            }
        }

        config.push_str("\nvariables:\n");
        config.push_str(&format!("  PIPELINE_TIMEOUT: {}\n", self.timeout_minutes));

        for (key, value) in &self.environment_variables {
            config.push_str(&format!("  {}: {}\n", key, value));
        }

        config.push_str("\ntranspile_job:\n");
        config.push_str("  stage: build\n");
        config.push_str("  script:\n");
        config.push_str("    - cargo test\n");
        config.push_str("    - cargo build --release\n");

        config
    }

    fn generate_jenkinsfile(&self) -> String {
        let mut config = String::from("pipeline {\n");
        config.push_str("  agent any\n\n");
        config.push_str("  stages {\n");

        for stage in &self.stages {
            config.push_str(&format!("    stage('{}') {{\n", stage.name()));
            config.push_str("      steps {\n");
            config.push_str(&format!("        sh 'echo Running {}'\n", stage.name()));
            config.push_str("      }\n");
            config.push_str("    }\n");
        }

        config.push_str("  }\n}\n");
        config
    }

    fn generate_circleci(&self) -> String {
        let mut config = String::from("version: 2.1\n\njobs:\n  build:\n    docker:\n      - image: rust:latest\n    steps:\n");

        for stage in &self.stages {
            config.push_str(&format!("      - run:\n          name: {}\n", stage.name()));
            config.push_str(&format!("          command: echo 'Running {}'\n", stage.name()));
        }

        config.push_str("\nworkflows:\n  version: 2\n  build_and_test:\n    jobs:\n      - build\n");
        config
    }
}

/// Pipeline execution result
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub stage: PipelineStage,
    pub success: bool,
    pub duration: Duration,
    pub output: String,
}

/// Pipeline executor
pub struct PipelineExecutor {
    config: PipelineConfig,
    results: Vec<PipelineResult>,
}

impl PipelineExecutor {
    pub fn new(config: PipelineConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    pub fn execute(&mut self) -> Result<PipelineReport> {
        let start = Instant::now();

        for &stage in &self.config.stages.clone() {
            let result = self.execute_stage(stage)?;

            if !result.success {
                return Err(batuta_cookbook::Error::Other(format!(
                    "Pipeline failed at stage: {:?}",
                    stage
                )));
            }

            self.results.push(result);
        }

        Ok(PipelineReport {
            pipeline_name: self.config.name.clone(),
            total_duration: start.elapsed(),
            stages_completed: self.results.len(),
            all_passed: self.results.iter().all(|r| r.success),
            results: self.results.clone(),
        })
    }

    fn execute_stage(&self, stage: PipelineStage) -> Result<PipelineResult> {
        let start = Instant::now();

        let (success, output) = match stage {
            PipelineStage::Checkout => (true, "Code checked out successfully".to_string()),
            PipelineStage::Dependencies => (true, "Dependencies installed".to_string()),
            PipelineStage::Lint => (true, "Linting passed".to_string()),
            PipelineStage::Transpile => (true, "Transpilation successful".to_string()),
            PipelineStage::Test => (true, "All tests passed (42/42)".to_string()),
            PipelineStage::IntegrationTest => {
                (true, "Integration tests passed (12/12)".to_string())
            }
            PipelineStage::QualityGates => {
                let passed = self.check_quality_gates();
                (
                    passed,
                    if passed {
                        "Quality gates passed".to_string()
                    } else {
                        "Quality gates failed".to_string()
                    },
                )
            }
            PipelineStage::Build => (true, "Artifacts built successfully".to_string()),
            PipelineStage::DeployStaging => (true, "Deployed to staging".to_string()),
            PipelineStage::SmokeTest => (true, "Smoke tests passed".to_string()),
            PipelineStage::DeployProduction => (true, "Deployed to production".to_string()),
            PipelineStage::Validation => (true, "Post-deployment validation passed".to_string()),
        };

        Ok(PipelineResult {
            stage,
            success,
            duration: start.elapsed(),
            output,
        })
    }

    fn check_quality_gates(&self) -> bool {
        // Simulate quality gate checks
        let test_coverage = 92.0;  // Good coverage
        let mutation_score = 82.0;  // Good mutation score
        let complexity = 8;  // Low complexity
        let duplication = 2.5;  // Low duplication

        test_coverage >= self.config.quality_gates.min_test_coverage
            && mutation_score >= self.config.quality_gates.min_mutation_score
            && complexity <= self.config.quality_gates.max_complexity
            && duplication <= self.config.quality_gates.max_duplication
    }
}

/// Pipeline execution report
#[derive(Debug, Clone)]
pub struct PipelineReport {
    pub pipeline_name: String,
    pub total_duration: Duration,
    pub stages_completed: usize,
    pub all_passed: bool,
    pub results: Vec<PipelineResult>,
}

impl PipelineReport {
    pub fn print_summary(&self) {
        println!("=== Pipeline Report: {} ===", self.pipeline_name);
        println!("Status: {}", if self.all_passed { "✓ PASSED" } else { "✗ FAILED" });
        println!("Total Duration: {:?}", self.total_duration);
        println!("Stages Completed: {}/{}", self.stages_completed, self.results.len());
        println!("\nStage Results:");

        for result in &self.results {
            println!(
                "  {:?}: {} ({:?}) - {}",
                result.stage,
                if result.success { "✓" } else { "✗" },
                result.duration,
                result.output
            );
        }
    }
}

//
// Example 1: Basic CI pipeline
//
pub fn example_1_basic_ci_pipeline() -> Result<()> {
    println!("=== Example 1: Basic CI Pipeline ===\n");

    let config = PipelineConfig::new(
        "Basic CI".to_string(),
        CiPlatform::GitHubActions,
    )
    .with_env("RUST_VERSION".to_string(), "1.75".to_string());

    println!("Generated GitHub Actions config:\n");
    println!("{}", config.generate_config());

    let mut executor = PipelineExecutor::new(config);
    let report = executor.execute()?;
    println!();
    report.print_summary();

    Ok(())
}

//
// Example 2: Advanced CD pipeline with quality gates
//
pub fn example_2_advanced_cd_pipeline() -> Result<()> {
    println!("\n=== Example 2: Advanced CD Pipeline with Quality Gates ===\n");

    let config = PipelineConfig::new(
        "Production CD".to_string(),
        CiPlatform::GitLabCi,
    )
    .with_stage(PipelineStage::IntegrationTest)
    .with_stage(PipelineStage::QualityGates)
    .with_stage(PipelineStage::DeployStaging)
    .with_quality_gates(QualityGates::strict());

    println!("Pipeline stages: {:?}", config.stages.len());
    println!("Quality gates: Strict mode");
    println!("Min coverage: {}%", config.quality_gates.min_test_coverage);
    println!();

    let mut executor = PipelineExecutor::new(config);
    let report = executor.execute()?;
    report.print_summary();

    Ok(())
}

//
// Example 3: Full production pipeline
//
pub fn example_3_full_production_pipeline() -> Result<()> {
    println!("\n=== Example 3: Full Production Pipeline ===\n");

    let config = PipelineConfig::new(
        "Full Production".to_string(),
        CiPlatform::GitHubActions,
    )
    .with_stage(PipelineStage::IntegrationTest)
    .with_stage(PipelineStage::QualityGates)
    .with_stage(PipelineStage::DeployStaging)
    .with_stage(PipelineStage::SmokeTest)
    .with_stage(PipelineStage::DeployProduction)
    .with_stage(PipelineStage::Validation)
    .with_quality_gates(QualityGates::default())
    .with_env("DEPLOY_ENV".to_string(), "production".to_string());

    println!("Full pipeline with {} stages", config.stages.len());
    println!();

    let mut executor = PipelineExecutor::new(config);
    let report = executor.execute()?;
    report.print_summary();

    Ok(())
}

fn main() -> Result<()> {
    example_1_basic_ci_pipeline()?;
    example_2_advanced_cd_pipeline()?;
    example_3_full_production_pipeline()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_gates_default() {
        let gates = QualityGates::default();
        assert_eq!(gates.min_test_coverage, 80.0);
        assert!(gates.require_all_tests_pass);
    }

    #[test]
    fn test_quality_gates_strict() {
        let gates = QualityGates::strict();
        assert_eq!(gates.min_test_coverage, 90.0);
        assert_eq!(gates.min_mutation_score, 80.0);
    }

    #[test]
    fn test_pipeline_config_creation() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        );

        assert_eq!(config.name, "Test");
        assert_eq!(config.platform, CiPlatform::GitHubActions);
        assert!(!config.stages.is_empty());
    }

    #[test]
    fn test_pipeline_with_stage() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        )
        .with_stage(PipelineStage::IntegrationTest);

        assert!(config.stages.contains(&PipelineStage::IntegrationTest));
    }

    #[test]
    fn test_pipeline_with_env() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        )
        .with_env("KEY".to_string(), "value".to_string());

        assert_eq!(config.environment_variables.get("KEY"), Some(&"value".to_string()));
    }

    #[test]
    fn test_github_actions_generation() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        );

        let generated = config.generate_config();
        assert!(generated.contains("name: Test"));
        assert!(generated.contains("runs-on: ubuntu-latest"));
    }

    #[test]
    fn test_gitlab_ci_generation() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitLabCi,
        );

        let generated = config.generate_config();
        assert!(generated.contains("stages:"));
    }

    #[test]
    fn test_pipeline_executor_creation() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        );

        let executor = PipelineExecutor::new(config);
        assert_eq!(executor.results.len(), 0);
    }

    #[test]
    fn test_pipeline_execution() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        );

        let mut executor = PipelineExecutor::new(config);
        let report = executor.execute().unwrap();

        assert!(report.all_passed);
        assert!(report.stages_completed > 0);
    }

    #[test]
    fn test_pipeline_stage_names() {
        assert_eq!(PipelineStage::Checkout.name(), "checkout");
        assert_eq!(PipelineStage::Test.name(), "test");
        assert_eq!(PipelineStage::Build.name(), "build");
    }

    #[test]
    fn test_quality_gates_relaxed() {
        let gates = QualityGates::relaxed();
        assert_eq!(gates.min_test_coverage, 70.0);
        assert!(!gates.require_all_tests_pass);
    }

    #[test]
    fn test_pipeline_with_custom_timeout() {
        let mut config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        );
        config.timeout_minutes = 60;

        assert_eq!(config.timeout_minutes, 60);
    }

    #[test]
    fn test_multiple_stages() {
        let config = PipelineConfig::new(
            "Test".to_string(),
            CiPlatform::GitHubActions,
        )
        .with_stage(PipelineStage::IntegrationTest)
        .with_stage(PipelineStage::QualityGates)
        .with_stage(PipelineStage::DeployStaging);

        assert!(config.stages.len() >= 9); // Base + 3 added
    }

    #[test]
    fn test_pipeline_report_summary() {
        let report = PipelineReport {
            pipeline_name: "Test".to_string(),
            total_duration: Duration::from_secs(10),
            stages_completed: 5,
            all_passed: true,
            results: vec![],
        };

        assert_eq!(report.pipeline_name, "Test");
        assert!(report.all_passed);
        assert_eq!(report.stages_completed, 5);
    }
}
