//! # Recipe 200-4: Optimization Profiles
//!
//! **Level:** 200 (Intermediate)
//! **Time Estimate:** 16 hours
//! **Priority:** P2 (Medium)
//!
//! ## Overview
//!
//! This recipe demonstrates how to use optimization profiles to balance compilation
//! speed, runtime performance, and resource usage. It provides configurable profiles
//! for different scenarios and includes benchmarking tools to measure the impact.
//!
//! ## Features
//!
//! - **Multiple Profiles:** Development, Production, and Balanced modes
//! - **Optimization Levels:** Configurable optimization strategies
//! - **Performance Benchmarking:** Measure and compare profile impact
//! - **Resource Monitoring:** Track memory and CPU usage
//! - **Profile Recommendations:** Automatic profile selection
//! - **Custom Profiles:** Define domain-specific optimization strategies
//!
//! ## Profiles
//!
//! - **Development:** Fast compilation, minimal optimizations
//! - **Balanced:** Good compromise between speed and performance
//! - **Production:** Maximum runtime performance
//! - **Size-Optimized:** Minimize binary size
//! - **Custom:** User-defined optimization parameters
//!
//! ## Examples
//!
//! Run examples with:
//! ```bash
//! cargo run --example recipe_200_4_optimization_profiles
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_200_4_optimization_profiles
//! ```

use batuta_cookbook::Result;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationLevel {
    /// No optimization (O0)
    None,
    /// Basic optimization (O1)
    Basic,
    /// Moderate optimization (O2)
    Moderate,
    /// Aggressive optimization (O3)
    Aggressive,
    /// Size optimization (Oz)
    Size,
}

impl std::fmt::Display for OptimizationLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "O0 (None)"),
            Self::Basic => write!(f, "O1 (Basic)"),
            Self::Moderate => write!(f, "O2 (Moderate)"),
            Self::Aggressive => write!(f, "O3 (Aggressive)"),
            Self::Size => write!(f, "Oz (Size)"),
        }
    }
}

/// Optimization profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationProfile {
    /// Profile name
    pub name: String,
    /// Description
    pub description: String,
    /// Optimization level
    pub optimization_level: OptimizationLevel,
    /// Enable parallel compilation
    pub parallel_compilation: bool,
    /// Enable incremental compilation
    pub incremental: bool,
    /// Enable link-time optimization
    pub lto: bool,
    /// Enable debug info
    pub debug_info: bool,
    /// Maximum parallel jobs
    pub max_jobs: usize,
    /// Custom flags
    pub custom_flags: Vec<String>,
}

impl OptimizationProfile {
    /// Create a development profile (fast compilation)
    pub fn development() -> Self {
        Self {
            name: "development".to_string(),
            description: "Fast compilation for rapid development".to_string(),
            optimization_level: OptimizationLevel::None,
            parallel_compilation: true,
            incremental: true,
            lto: false,
            debug_info: true,
            max_jobs: num_cpus::get(),
            custom_flags: Vec::new(),
        }
    }

    /// Create a production profile (maximum performance)
    pub fn production() -> Self {
        Self {
            name: "production".to_string(),
            description: "Maximum runtime performance".to_string(),
            optimization_level: OptimizationLevel::Aggressive,
            parallel_compilation: true,
            incremental: false,
            lto: true,
            debug_info: false,
            max_jobs: num_cpus::get(),
            custom_flags: vec![
                "--codegen-units=1".to_string(),
                "--enable-vectorization".to_string(),
            ],
        }
    }

    /// Create a balanced profile
    pub fn balanced() -> Self {
        Self {
            name: "balanced".to_string(),
            description: "Balance between compilation speed and runtime performance".to_string(),
            optimization_level: OptimizationLevel::Moderate,
            parallel_compilation: true,
            incremental: true,
            lto: false,
            debug_info: true,
            max_jobs: num_cpus::get(),
            custom_flags: Vec::new(),
        }
    }

    /// Create a size-optimized profile
    pub fn size_optimized() -> Self {
        Self {
            name: "size".to_string(),
            description: "Minimize binary size".to_string(),
            optimization_level: OptimizationLevel::Size,
            parallel_compilation: true,
            incremental: false,
            lto: true,
            debug_info: false,
            max_jobs: num_cpus::get(),
            custom_flags: vec!["--strip-symbols".to_string()],
        }
    }

    /// Create a custom profile
    pub fn custom(name: String, optimization_level: OptimizationLevel) -> Self {
        Self {
            name,
            description: "Custom optimization profile".to_string(),
            optimization_level,
            parallel_compilation: true,
            incremental: true,
            lto: false,
            debug_info: true,
            max_jobs: num_cpus::get(),
            custom_flags: Vec::new(),
        }
    }

    /// Estimate compilation time multiplier (relative to development)
    pub fn compilation_time_estimate(&self) -> f64 {
        let mut multiplier = 1.0;

        // Optimization level impact
        multiplier *= match self.optimization_level {
            OptimizationLevel::None => 1.0,
            OptimizationLevel::Basic => 1.2,
            OptimizationLevel::Moderate => 1.5,
            OptimizationLevel::Aggressive => 2.0,
            OptimizationLevel::Size => 1.8,
        };

        // LTO adds significant time
        if self.lto {
            multiplier *= 1.5;
        }

        // Incremental reduces time on rebuilds
        if self.incremental {
            multiplier *= 0.7;
        }

        multiplier
    }

    /// Estimate runtime performance improvement (relative to no optimization)
    pub fn performance_improvement(&self) -> f64 {
        match self.optimization_level {
            OptimizationLevel::None => 1.0,
            OptimizationLevel::Basic => 1.3,
            OptimizationLevel::Moderate => 1.8,
            OptimizationLevel::Aggressive => 2.5,
            OptimizationLevel::Size => 1.5,
        }
    }
}

/// Benchmark result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    /// Profile name
    pub profile_name: String,
    /// Compilation time
    pub compilation_time: Duration,
    /// Runtime performance (operations per second)
    pub operations_per_sec: f64,
    /// Memory usage (MB)
    pub memory_usage_mb: f64,
    /// Binary size (KB)
    pub binary_size_kb: Option<usize>,
}

impl BenchmarkResult {
    /// Calculate score (higher is better)
    pub fn score(&self) -> f64 {
        // Normalize compilation time (lower is better, so invert)
        let compile_score = 100.0 / (1.0 + self.compilation_time.as_secs_f64());

        // Runtime performance (higher is better)
        let perf_score = self.operations_per_sec / 100.0;

        // Memory usage (lower is better, so invert)
        let memory_score = 100.0 / (1.0 + self.memory_usage_mb);

        // Weighted average
        (compile_score * 0.3) + (perf_score * 0.5) + (memory_score * 0.2)
    }
}

/// Profile benchmarker
pub struct ProfileBenchmarker {
    profiles: Vec<OptimizationProfile>,
    results: Vec<BenchmarkResult>,
}

impl ProfileBenchmarker {
    /// Create a new benchmarker
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            results: Vec::new(),
        }
    }

    /// Add a profile to benchmark
    pub fn add_profile(mut self, profile: OptimizationProfile) -> Self {
        self.profiles.push(profile);
        self
    }

    /// Add standard profiles
    pub fn with_standard_profiles(self) -> Self {
        self.add_profile(OptimizationProfile::development())
            .add_profile(OptimizationProfile::balanced())
            .add_profile(OptimizationProfile::production())
    }

    /// Run benchmarks
    pub fn run_benchmarks(&mut self, iterations: usize) -> Result<()> {
        for profile in &self.profiles {
            let result = self.benchmark_profile(profile, iterations)?;
            self.results.push(result);
        }
        Ok(())
    }

    /// Benchmark a single profile
    fn benchmark_profile(
        &self,
        profile: &OptimizationProfile,
        _iterations: usize,
    ) -> Result<BenchmarkResult> {
        // Simulate compilation
        let compile_start = Instant::now();
        std::thread::sleep(Duration::from_millis(
            (10.0 * profile.compilation_time_estimate()) as u64,
        ));
        let compilation_time = compile_start.elapsed();

        // Simulate runtime performance
        let perf_baseline = 1000.0;
        let operations_per_sec = perf_baseline * profile.performance_improvement();

        // Simulate memory usage (optimized code uses less memory)
        let memory_baseline = 100.0;
        let memory_factor = match profile.optimization_level {
            OptimizationLevel::None => 1.0,
            OptimizationLevel::Basic => 0.9,
            OptimizationLevel::Moderate => 0.8,
            OptimizationLevel::Aggressive => 0.75,
            OptimizationLevel::Size => 0.7,
        };
        let memory_usage_mb = memory_baseline * memory_factor;

        // Simulate binary size
        let binary_size_kb = Some(match profile.optimization_level {
            OptimizationLevel::None => 500,
            OptimizationLevel::Basic => 450,
            OptimizationLevel::Moderate => 400,
            OptimizationLevel::Aggressive => 380,
            OptimizationLevel::Size => 300,
        });

        Ok(BenchmarkResult {
            profile_name: profile.name.clone(),
            compilation_time,
            operations_per_sec,
            memory_usage_mb,
            binary_size_kb,
        })
    }

    /// Get results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Get best profile by score
    pub fn best_profile(&self) -> Option<&BenchmarkResult> {
        self.results.iter().max_by(|a, b| {
            a.score()
                .partial_cmp(&b.score())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
    }

    /// Print comparison table
    pub fn print_comparison(&self) {
        println!("Profile Comparison:");
        println!("{:<15} {:>12} {:>15} {:>12} {:>10} {:>8}",
                 "Profile", "Compile (ms)", "Ops/sec", "Memory (MB)", "Size (KB)", "Score");
        println!("{}", "-".repeat(85));

        for result in &self.results {
            println!(
                "{:<15} {:>12} {:>15.0} {:>12.1} {:>10} {:>8.2}",
                result.profile_name,
                result.compilation_time.as_millis(),
                result.operations_per_sec,
                result.memory_usage_mb,
                result
                    .binary_size_kb
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
                result.score()
            );
        }
    }
}

impl Default for ProfileBenchmarker {
    fn default() -> Self {
        Self::new()
    }
}

/// Profile selector - recommends best profile based on criteria
pub struct ProfileSelector {
    /// Available profiles
    profiles: Vec<OptimizationProfile>,
}

impl ProfileSelector {
    /// Create a new selector
    pub fn new() -> Self {
        Self {
            profiles: vec![
                OptimizationProfile::development(),
                OptimizationProfile::balanced(),
                OptimizationProfile::production(),
                OptimizationProfile::size_optimized(),
            ],
        }
    }

    /// Recommend profile based on use case
    pub fn recommend(&self, use_case: UseCase) -> &OptimizationProfile {
        match use_case {
            UseCase::Development => self
                .profiles
                .iter()
                .find(|p| p.name == "development")
                .unwrap(),
            UseCase::Production => self.profiles.iter().find(|p| p.name == "production").unwrap(),
            UseCase::Testing => self.profiles.iter().find(|p| p.name == "balanced").unwrap(),
            UseCase::Deployment => self.profiles.iter().find(|p| p.name == "size").unwrap(),
        }
    }

    /// Get all profiles
    pub fn all_profiles(&self) -> &[OptimizationProfile] {
        &self.profiles
    }
}

impl Default for ProfileSelector {
    fn default() -> Self {
        Self::new()
    }
}

/// Use case for profile selection
#[derive(Debug, Clone, Copy)]
pub enum UseCase {
    /// Development/debugging
    Development,
    /// Production deployment
    Production,
    /// Testing/CI
    Testing,
    /// Binary deployment (size matters)
    Deployment,
}

// Helper function to get number of CPUs
mod num_cpus {
    pub fn get() -> usize {
        // Simplified - in real implementation would use num_cpus crate
        std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4)
    }
}

// ============================================================================
// EXAMPLE 1: Profile Comparison
// ============================================================================

fn example_1_profile_comparison() -> Result<()> {
    println!("=== Example 1: Profile Comparison ===\n");

    let mut benchmarker = ProfileBenchmarker::new().with_standard_profiles();

    println!("Running benchmarks...\n");
    benchmarker.run_benchmarks(1000)?;

    benchmarker.print_comparison();

    if let Some(best) = benchmarker.best_profile() {
        println!("\n🏆 Best Overall Profile: {}", best.profile_name);
        println!("   Score: {:.2}", best.score());
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Profile Recommendations
// ============================================================================

fn example_2_profile_recommendations() -> Result<()> {
    println!("=== Example 2: Profile Recommendations ===\n");

    let selector = ProfileSelector::new();

    let use_cases = vec![
        (UseCase::Development, "Development"),
        (UseCase::Production, "Production"),
        (UseCase::Testing, "Testing/CI"),
        (UseCase::Deployment, "Binary Deployment"),
    ];

    for (use_case, name) in use_cases {
        let profile = selector.recommend(use_case);
        println!("📋 Use Case: {}", name);
        println!("   Recommended Profile: {}", profile.name);
        println!("   Optimization Level: {}", profile.optimization_level);
        println!("   Compile Time Factor: {:.2}x", profile.compilation_time_estimate());
        println!("   Performance Gain: {:.2}x", profile.performance_improvement());
        println!();
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Custom Profile Creation
// ============================================================================

fn example_3_custom_profile() -> Result<()> {
    println!("=== Example 3: Custom Profile Creation ===\n");

    // Create a custom profile for specific needs
    let mut custom = OptimizationProfile::custom(
        "ci-optimized".to_string(),
        OptimizationLevel::Moderate,
    );
    custom.description = "Optimized for CI/CD pipelines".to_string();
    custom.incremental = false; // CI always builds from scratch
    custom.debug_info = false; // No need for debug in CI
    custom.max_jobs = 8; // Typical CI runner capacity

    println!("Custom Profile: {}", custom.name);
    println!("Description: {}", custom.description);
    println!("Settings:");
    println!("  - Optimization: {}", custom.optimization_level);
    println!("  - Parallel Compilation: {}", custom.parallel_compilation);
    println!("  - Incremental: {}", custom.incremental);
    println!("  - LTO: {}", custom.lto);
    println!("  - Debug Info: {}", custom.debug_info);
    println!("  - Max Jobs: {}", custom.max_jobs);
    println!("\nEstimates:");
    println!("  - Compile Time: {:.2}x baseline", custom.compilation_time_estimate());
    println!("  - Performance: {:.2}x improvement", custom.performance_improvement());

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_profile_comparison()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_profile_recommendations()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_custom_profile()?;

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_level_display() {
        assert_eq!(OptimizationLevel::None.to_string(), "O0 (None)");
        assert_eq!(OptimizationLevel::Aggressive.to_string(), "O3 (Aggressive)");
    }

    #[test]
    fn test_development_profile() {
        let profile = OptimizationProfile::development();

        assert_eq!(profile.name, "development");
        assert_eq!(profile.optimization_level, OptimizationLevel::None);
        assert!(profile.incremental);
        assert!(profile.debug_info);
        assert!(!profile.lto);
    }

    #[test]
    fn test_production_profile() {
        let profile = OptimizationProfile::production();

        assert_eq!(profile.name, "production");
        assert_eq!(profile.optimization_level, OptimizationLevel::Aggressive);
        assert!(!profile.incremental);
        assert!(!profile.debug_info);
        assert!(profile.lto);
    }

    #[test]
    fn test_balanced_profile() {
        let profile = OptimizationProfile::balanced();

        assert_eq!(profile.name, "balanced");
        assert_eq!(profile.optimization_level, OptimizationLevel::Moderate);
        assert!(profile.incremental);
        assert!(profile.debug_info);
    }

    #[test]
    fn test_compilation_time_estimates() {
        let dev = OptimizationProfile::development();
        let prod = OptimizationProfile::production();

        // Production should take longer than development
        assert!(prod.compilation_time_estimate() > dev.compilation_time_estimate());
    }

    #[test]
    fn test_performance_improvements() {
        let dev = OptimizationProfile::development();
        let prod = OptimizationProfile::production();

        // Production should have better performance
        assert!(prod.performance_improvement() > dev.performance_improvement());
        assert_eq!(dev.performance_improvement(), 1.0);
    }

    #[test]
    fn test_benchmark_result_score() {
        let result = BenchmarkResult {
            profile_name: "test".to_string(),
            compilation_time: Duration::from_millis(100),
            operations_per_sec: 2000.0,
            memory_usage_mb: 50.0,
            binary_size_kb: Some(400),
        };

        let score = result.score();
        assert!(score > 0.0);
    }

    #[test]
    fn test_profile_selector_recommendations() {
        let selector = ProfileSelector::new();

        let dev_profile = selector.recommend(UseCase::Development);
        assert_eq!(dev_profile.name, "development");

        let prod_profile = selector.recommend(UseCase::Production);
        assert_eq!(prod_profile.name, "production");

        let test_profile = selector.recommend(UseCase::Testing);
        assert_eq!(test_profile.name, "balanced");
    }

    #[test]
    fn test_custom_profile_creation() {
        let profile = OptimizationProfile::custom(
            "my-profile".to_string(),
            OptimizationLevel::Moderate,
        );

        assert_eq!(profile.name, "my-profile");
        assert_eq!(profile.optimization_level, OptimizationLevel::Moderate);
    }

    #[test]
    fn test_benchmarker_creation() {
        let benchmarker = ProfileBenchmarker::new()
            .add_profile(OptimizationProfile::development())
            .add_profile(OptimizationProfile::production());

        assert_eq!(benchmarker.profiles.len(), 2);
    }

    #[test]
    fn test_benchmarker_with_standard_profiles() {
        let benchmarker = ProfileBenchmarker::new().with_standard_profiles();

        assert_eq!(benchmarker.profiles.len(), 3);
    }

    #[test]
    fn test_run_benchmarks() {
        let mut benchmarker = ProfileBenchmarker::new()
            .add_profile(OptimizationProfile::development());

        benchmarker.run_benchmarks(100).unwrap();

        assert_eq!(benchmarker.results().len(), 1);
    }

    #[test]
    fn test_best_profile_selection() {
        let mut benchmarker = ProfileBenchmarker::new().with_standard_profiles();

        benchmarker.run_benchmarks(100).unwrap();

        let best = benchmarker.best_profile();
        assert!(best.is_some());
    }

    #[test]
    fn test_profile_selector_all_profiles() {
        let selector = ProfileSelector::new();
        let profiles = selector.all_profiles();

        assert_eq!(profiles.len(), 4);
    }
}
