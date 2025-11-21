//! Recipe 100-1: Basic Project Analysis
//!
//! **Level:** 100 (Basic)
//! **Category:** Analysis
//! **Difficulty:** Basic
//!
//! ## Description
//!
//! This recipe demonstrates how to analyze a project's languages and structure
//! using the Batuta analyzer. You'll learn to detect primary languages, count
//! files, and understand project composition.
//!
//! ## Prerequisites
//!
//! - Batuta cookbook installed
//! - Basic Rust knowledge
//!
//! ## Learning Objectives
//!
//! After completing this recipe, you will understand:
//! - How to create an Analyzer instance
//! - How to run basic project analysis
//! - How to read and interpret analysis reports
//! - How language detection works
//!
//! ## Run This Recipe
//!
//! ```bash
//! cargo run --example recipe_100_1_basic_analysis
//! cargo test recipe_100_1
//! ```
//!
//! ## Related Papers
//!
//! - Abstract Interpretation (Cousot & Cousot, 1977) - Sound program analysis

#![allow(clippy::print_stdout)] // Examples should print output
#![allow(clippy::unwrap_used)] // Examples can use unwrap for clarity

use batuta_cookbook::Analyzer;

fn main() {
    println!("=== Recipe 100-1: Basic Project Analysis ===\n");

    // Example 1: Analyze a Python project
    example_1_analyze_python();

    // Example 2: Analyze current directory
    example_2_analyze_current_dir();

    // Example 3: Error handling
    example_3_error_handling();

    println!("\n✅ All examples completed successfully!");
}

/// Example 1: Analyze a Python project
///
/// Demonstrates basic analysis of a sample Python project
fn example_1_analyze_python() {
    println!("Example 1: Analyze Python Project");
    println!("----------------------------------");

    // Create analyzer for sample project
    let analyzer = Analyzer::new("./examples/data/sample_python_project");

    // Run analysis
    match analyzer.analyze() {
        Ok(report) => {
            println!("✓ Analysis complete!");
            println!("  Primary language: {}", report.primary_language);
            println!("  Total files: {}", report.file_count);
            println!("  Total lines: {}", report.total_lines);
            println!("  Languages detected: {}", report.languages.len());
        }
        Err(e) => {
            println!("✗ Analysis failed: {e}");
        }
    }

    println!();
}

/// Example 2: Analyze current directory
///
/// Shows how to analyze the current working directory
fn example_2_analyze_current_dir() {
    println!("Example 2: Analyze Current Directory");
    println!("-------------------------------------");

    let analyzer = Analyzer::new(".");
    let report = analyzer.analyze().unwrap();

    println!("✓ Current directory analysis:");
    println!("  Path: {}", report.path);
    println!("  Primary language: {}", report.primary_language);
    println!("  Files: {}", report.file_count);

    println!();
}

/// Example 3: Error handling
///
/// Demonstrates proper error handling for invalid paths
fn example_3_error_handling() {
    println!("Example 3: Error Handling");
    println!("-------------------------");

    // Try to analyze a non-existent path
    let analyzer = Analyzer::new("/nonexistent/path");

    match analyzer.analyze() {
        Ok(_) => println!("✗ Unexpected success"),
        Err(e) => println!("✓ Expected error: {e}"),
    }

    println!();
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that example 1 runs without panicking
    #[test]
    fn test_example_1_runs() {
        example_1_analyze_python();
    }

    /// Test that example 2 runs without panicking
    #[test]
    fn test_example_2_runs() {
        example_2_analyze_current_dir();
    }

    /// Test that example 3 runs without panicking
    #[test]
    fn test_example_3_runs() {
        example_3_error_handling();
    }

    /// Test the actual functionality
    #[test]
    fn test_analyze_sample_project() {
        let analyzer = Analyzer::new("./examples/data/sample_python_project");
        let result = analyzer.analyze();

        assert!(result.is_ok(), "Analysis should succeed");

        let report = result.unwrap();
        assert_eq!(report.path, "./examples/data/sample_python_project");
        assert!(report.file_count > 0, "Should find at least one file");
    }

    /// Test error handling for invalid path
    #[test]
    fn test_invalid_path_error() {
        let analyzer = Analyzer::new("/this/path/does/not/exist/12345");
        let result = analyzer.analyze();

        assert!(result.is_err(), "Should fail for invalid path");
    }

    /// Test analyzing current directory
    #[test]
    fn test_analyze_current_directory() {
        let analyzer = Analyzer::new(".");
        let result = analyzer.analyze();

        assert!(result.is_ok(), "Current directory should exist");

        let report = result.unwrap();
        assert!(report.file_count > 0);
        assert!(report.total_lines > 0);
    }
}

// ============================================================================
// PROPERTY-BASED TESTS
// ============================================================================

#[cfg(all(test, feature = "proptest"))]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        /// Property: Analyzing any valid path should not panic
        #[test]
        fn test_analyze_does_not_panic(path in "[a-z./]{1,20}") {
            let analyzer = Analyzer::new(&path);
            let _ = analyzer.analyze(); // May succeed or fail, but shouldn't panic
        }

        /// Property: File count should never be negative
        #[test]
        fn test_file_count_non_negative(path in "[a-z./]{1,10}") {
            let analyzer = Analyzer::new(&path);
            if let Ok(report) = analyzer.analyze() {
                // If analysis succeeds, counts should be valid
                prop_assert!(report.file_count < usize::MAX);
                prop_assert!(report.total_lines < usize::MAX);
            }
        }
    }
}
