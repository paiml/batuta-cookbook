//! # Batuta Cookbook Library
//!
//! This library provides core types and utilities for the Batuta Cookbook recipes.
//! Each recipe in the `examples/` directory demonstrates real, runnable code for
//! code orchestration, transpilation, and optimization.
//!
//! ## Core Principle: EXTREME TDD
//!
//! Every example in this cookbook:
//! - ✅ Compiles and runs without errors
//! - ✅ Has comprehensive tests (>90% coverage)
//! - ✅ Demonstrates real functionality
//! - ✅ Includes benchmarks where performance claims are made
//!
//! ## Usage
//!
//! ```bash
//! # Run a recipe
//! cargo run --example recipe_100_1_basic_analysis
//!
//! # Test a recipe
//! cargo test recipe_100_1
//!
//! # Benchmark a recipe
//! cargo bench recipe_100_1
//! ```
//!
//! ## Module Structure
//!
//! - [`analyzer`] - Project analysis and TDG scoring
//! - [`transpiler`] - Code transpilation utilities
//! - [`optimizer`] - Performance optimization
//! - [`validator`] - Semantic equivalence validation
//! - [`types`] - Common types used across recipes

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

pub mod analyzer;
pub mod optimizer;
pub mod transpiler;
pub mod types;
pub mod validator;

// Re-export commonly used types
pub use analyzer::{AnalysisReport, Analyzer};
pub use types::{Error, Result};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_defined() {
        // VERSION is a const from env!, so it's always non-empty at compile time
        assert!(VERSION.starts_with(char::is_numeric));
    }
}
