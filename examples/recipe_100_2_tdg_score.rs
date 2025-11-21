//! Recipe 100-2: Calculate Technical Debt Grade (TDG)
//!
//! **Level:** 100 (Basic)
//! **Category:** Analysis
//! **Difficulty:** Basic
//!
//! ## Description
//!
//! Learn how to calculate and interpret Technical Debt Grade (TDG) scores
//! for your projects. Understand quality metrics and how to improve them.
//!
//! ## Run This Recipe
//!
//! ```bash
//! cargo run --example recipe_100_2_tdg_score
//! ```
//!
//! ## Related Papers
//!
//! - Fowler (1999) - Code Smells and Refactoring
//! - `McCabe` (1976) - Cyclomatic Complexity

#![allow(clippy::print_stdout)]
#![allow(clippy::unwrap_used)]

use batuta_cookbook::Analyzer;

fn main() {
    println!("=== Recipe 100-2: TDG Score Calculator ===\n");

    example_1_calculate_tdg();
    example_2_interpret_grades();

    println!("\n✅ Recipe completed!");
}

fn example_1_calculate_tdg() {
    println!("Example 1: Calculate TDG Score");
    println!("-------------------------------");

    let analyzer = Analyzer::new(".");
    match analyzer.analyze_with_tdg() {
        Ok(report) => {
            let tdg = report.tdg();
            println!("✓ TDG Score: {:.1}/100 ({})", tdg.score, tdg.grade);
        }
        Err(e) => println!("✗ Error: {e}"),
    }

    println!();
}

fn example_2_interpret_grades() {
    println!("Example 2: Grade Interpretation");
    println!("--------------------------------");

    let grades = vec![
        (96.0, "A+ - Excellent quality"),
        (92.0, "A - Very good quality"),
        (87.0, "A- - Good quality"),
        (82.0, "B+ - Above average"),
        (75.0, "B - Average"),
        (60.0, "C - Needs improvement"),
        (45.0, "F - Major issues"),
    ];

    for (score, description) in grades {
        println!("  {score:.0}% - {description}");
    }

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples_run() {
        example_1_calculate_tdg();
        example_2_interpret_grades();
    }

    #[test]
    fn test_tdg_bounds() {
        let analyzer = Analyzer::new(".");
        if let Ok(report) = analyzer.analyze_with_tdg() {
            let tdg = report.tdg();
            assert!(tdg.score >= 0.0);
            assert!(tdg.score <= 100.0);
        }
    }
}
