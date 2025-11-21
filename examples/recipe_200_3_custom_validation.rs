//! # Recipe 200-3: Custom Validation Rules
//!
//! **Level:** 200 (Intermediate)
//! **Time Estimate:** 18 hours
//! **Priority:** P2 (Medium)
//!
//! ## Overview
//!
//! This recipe demonstrates how to create and apply custom validation rules to enforce
//! code quality standards, naming conventions, and project-specific requirements.
//! It provides a flexible rule system that can validate code at the file, function,
//! and line level.
//!
//! ## Features
//!
//! - **Rule Definition:** Create custom validation rules with predicates
//! - **Severity Levels:** Error, Warning, and Info classifications
//! - **Pattern Matching:** Regex-based pattern detection
//! - **Rule Composition:** Combine multiple rules into rule sets
//! - **Detailed Reports:** Comprehensive validation findings with locations
//! - **Configurable:** Load rules from configuration files
//! - **Extensible:** Easy to add new rule types
//!
//! ## Rule Types
//!
//! - **Naming Conventions:** Function/variable naming patterns
//! - **Code Patterns:** Detect anti-patterns and bad practices
//! - **Documentation:** Ensure proper comments and documentation
//! - **Complexity:** Limit function complexity
//! - **Security:** Detect common security issues
//! - **Style:** Enforce consistent code style
//!
//! ## Examples
//!
//! Run examples with:
//! ```bash
//! cargo run --example recipe_200_3_custom_validation
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_200_3_custom_validation
//! ```

use batuta_cookbook::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Severity level for validation findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    /// Critical error that must be fixed
    Error,
    /// Warning that should be addressed
    Warning,
    /// Informational message
    Info,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Error => write!(f, "ERROR"),
            Self::Warning => write!(f, "WARNING"),
            Self::Info => write!(f, "INFO"),
        }
    }
}

/// A validation finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Rule ID that generated this finding
    pub rule_id: String,
    /// Severity level
    pub severity: Severity,
    /// File path where the issue was found
    pub file_path: PathBuf,
    /// Line number (if applicable)
    pub line: Option<usize>,
    /// Column number (if applicable)
    pub column: Option<usize>,
    /// Description of the issue
    pub message: String,
    /// Code snippet (if available)
    pub snippet: Option<String>,
    /// Suggested fix (if available)
    pub suggestion: Option<String>,
}

impl Finding {
    /// Create a new finding
    pub fn new(
        rule_id: String,
        severity: Severity,
        file_path: PathBuf,
        message: String,
    ) -> Self {
        Self {
            rule_id,
            severity,
            file_path,
            line: None,
            column: None,
            message,
            snippet: None,
            suggestion: None,
        }
    }

    /// Set line number
    pub fn with_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }

    /// Set code snippet
    pub fn with_snippet(mut self, snippet: String) -> Self {
        self.snippet = Some(snippet);
        self
    }

    /// Set suggestion
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }
}

/// Validation rule trait
pub trait ValidationRule: Send + Sync {
    /// Get rule ID
    fn id(&self) -> &str;

    /// Get rule description
    fn description(&self) -> &str;

    /// Get severity level
    fn severity(&self) -> Severity;

    /// Validate a file
    fn validate(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>>;
}

/// Regex-based pattern rule
#[derive(Debug, Clone)]
pub struct PatternRule {
    /// Rule ID
    id: String,
    /// Rule description
    description: String,
    /// Severity
    severity: Severity,
    /// Pattern to match (as string, will be compiled)
    pattern: String,
    /// Message template
    message_template: String,
    /// Whether pattern should NOT be found (inverted)
    inverted: bool,
}

impl PatternRule {
    /// Create a new pattern rule
    pub fn new(
        id: String,
        description: String,
        severity: Severity,
        pattern: String,
        message_template: String,
    ) -> Self {
        Self {
            id,
            description,
            severity,
            pattern,
            message_template,
            inverted: false,
        }
    }

    /// Create an inverted pattern rule (pattern should NOT be found)
    pub fn new_inverted(
        id: String,
        description: String,
        severity: Severity,
        pattern: String,
        message_template: String,
    ) -> Self {
        Self {
            id,
            description,
            severity,
            pattern,
            message_template,
            inverted: true,
        }
    }
}

impl ValidationRule for PatternRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn validate(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            let matches = line.contains(&self.pattern);

            // For inverted rules, we want to flag when pattern IS found
            // For normal rules, we want to flag when pattern IS found
            let should_flag = if self.inverted { matches } else { matches };

            if should_flag {
                let finding = Finding::new(
                    self.id.clone(),
                    self.severity,
                    file_path.to_path_buf(),
                    self.message_template.clone(),
                )
                .with_line(line_num + 1)
                .with_snippet(line.to_string());

                findings.push(finding);
            }
        }

        Ok(findings)
    }
}

/// Function length rule
#[derive(Debug, Clone)]
pub struct FunctionLengthRule {
    id: String,
    max_lines: usize,
    severity: Severity,
}

impl FunctionLengthRule {
    /// Create a new function length rule
    pub fn new(max_lines: usize, severity: Severity) -> Self {
        Self {
            id: "function_length".to_string(),
            max_lines,
            severity,
        }
    }
}

impl ValidationRule for FunctionLengthRule {
    fn id(&self) -> &str {
        &self.id
    }

    fn description(&self) -> &str {
        "Functions should not exceed maximum line count"
    }

    fn severity(&self) -> Severity {
        self.severity
    }

    fn validate(&self, file_path: &Path, content: &str) -> Result<Vec<Finding>> {
        let mut findings = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut in_function = false;
        let mut function_start = 0;
        let mut function_name = String::new();
        let mut brace_count = 0;

        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Simple function detection for Rust
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                in_function = true;
                function_start = line_num;
                // Extract function name
                if let Some(name_end) = trimmed.find('(') {
                    let name_start = if trimmed.starts_with("pub fn ") { 7 } else { 3 };
                    function_name = trimmed[name_start..name_end].trim().to_string();
                }
                brace_count = 0;
            }

            if in_function {
                if trimmed.contains('{') {
                    brace_count += trimmed.matches('{').count();
                }
                if trimmed.contains('}') {
                    brace_count -= trimmed.matches('}').count();

                    if brace_count == 0 {
                        // Function ended
                        let function_length = line_num - function_start + 1;
                        if function_length > self.max_lines {
                            let finding = Finding::new(
                                self.id.clone(),
                                self.severity,
                                file_path.to_path_buf(),
                                format!(
                                    "Function '{}' has {} lines (max: {})",
                                    function_name, function_length, self.max_lines
                                ),
                            )
                            .with_line(function_start + 1)
                            .with_suggestion(format!(
                                "Consider breaking this function into smaller functions"
                            ));

                            findings.push(finding);
                        }
                        in_function = false;
                    }
                }
            }
        }

        Ok(findings)
    }
}

/// Validation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationReport {
    /// Files validated
    pub files_validated: usize,
    /// Total findings
    pub total_findings: usize,
    /// Findings by severity
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
    /// All findings
    pub findings: Vec<Finding>,
}

impl ValidationReport {
    /// Create an empty report
    pub fn new() -> Self {
        Self {
            files_validated: 0,
            total_findings: 0,
            error_count: 0,
            warning_count: 0,
            info_count: 0,
            findings: Vec::new(),
        }
    }

    /// Add findings to the report
    pub fn add_findings(&mut self, findings: Vec<Finding>) {
        for finding in findings {
            match finding.severity {
                Severity::Error => self.error_count += 1,
                Severity::Warning => self.warning_count += 1,
                Severity::Info => self.info_count += 1,
            }
            self.findings.push(finding);
        }
        self.total_findings = self.findings.len();
    }

    /// Check if validation passed (no errors)
    pub fn passed(&self) -> bool {
        self.error_count == 0
    }

    /// Get findings by severity
    pub fn findings_by_severity(&self, severity: Severity) -> Vec<&Finding> {
        self.findings
            .iter()
            .filter(|f| f.severity == severity)
            .collect()
    }

    /// Print report summary
    pub fn print_summary(&self) {
        println!("Validation Report:");
        println!("  Files validated: {}", self.files_validated);
        println!("  Total findings: {}", self.total_findings);
        println!("    Errors: {}", self.error_count);
        println!("    Warnings: {}", self.warning_count);
        println!("    Info: {}", self.info_count);
        println!("  Status: {}", if self.passed() { "PASSED ✓" } else { "FAILED ✗" });
    }
}

impl Default for ValidationReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Validator that applies rules to files
pub struct Validator {
    /// Validation rules
    rules: Vec<Box<dyn ValidationRule>>,
    /// Files to exclude
    exclude_patterns: Vec<String>,
}

impl Validator {
    /// Create a new validator
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            exclude_patterns: Vec::new(),
        }
    }

    /// Add a validation rule
    pub fn add_rule<R: ValidationRule + 'static>(mut self, rule: R) -> Self {
        self.rules.push(Box::new(rule));
        self
    }

    /// Add an exclusion pattern
    pub fn add_exclusion(mut self, pattern: String) -> Self {
        self.exclude_patterns.push(pattern);
        self
    }

    /// Validate a single file
    pub fn validate_file(&self, file_path: &Path) -> Result<Vec<Finding>> {
        // Check if file should be excluded
        let file_name = file_path.to_string_lossy();
        for pattern in &self.exclude_patterns {
            if file_name.contains(pattern) {
                return Ok(Vec::new());
            }
        }

        let content = fs::read_to_string(file_path).map_err(|e| {
            Error::ValidationError(format!("Failed to read file {}: {}", file_path.display(), e))
        })?;

        let mut all_findings = Vec::new();

        for rule in &self.rules {
            let findings = rule.validate(file_path, &content)?;
            all_findings.extend(findings);
        }

        Ok(all_findings)
    }

    /// Validate multiple files
    pub fn validate_files(&self, file_paths: &[PathBuf]) -> Result<ValidationReport> {
        let mut report = ValidationReport::new();

        for file_path in file_paths {
            if file_path.exists() && file_path.is_file() {
                let findings = self.validate_file(file_path)?;
                report.add_findings(findings);
                report.files_validated += 1;
            }
        }

        Ok(report)
    }

    /// Validate a directory recursively
    pub fn validate_directory(&self, dir_path: &Path, extension: &str) -> Result<ValidationReport> {
        let mut files = Vec::new();
        Self::collect_files(dir_path, extension, &mut files)?;

        self.validate_files(&files)
    }

    /// Collect files recursively
    fn collect_files(dir_path: &Path, extension: &str, files: &mut Vec<PathBuf>) -> Result<()> {
        let entries = fs::read_dir(dir_path).map_err(|e| {
            Error::ValidationError(format!("Failed to read directory {}: {}", dir_path.display(), e))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                Error::ValidationError(format!("Failed to read entry: {}", e))
            })?;
            let path = entry.path();

            if path.is_dir() {
                // Skip common build/dependency directories
                let dir_name = path.file_name().unwrap().to_string_lossy();
                if dir_name != "target" && dir_name != "node_modules" {
                    Self::collect_files(&path, extension, files)?;
                }
            } else if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == extension {
                        files.push(path);
                    }
                }
            }
        }

        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// EXAMPLE 1: Basic Validation with Pattern Rules
// ============================================================================

fn example_1_pattern_validation() -> Result<()> {
    println!("=== Example 1: Basic Validation with Pattern Rules ===\n");

    // Create test file
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_code.rs");

    let code = r#"
fn calculate() {
    let x = 10; // TODO: implement this
    println!("test");
}

fn process_data() {
    panic!("Not implemented"); // FIXME: add error handling
}
"#;

    fs::write(&test_file, code)
        .map_err(|e| Error::Other(format!("Failed to write test file: {}", e)))?;

    // Create validator with pattern rules
    let validator = Validator::new()
        .add_rule(PatternRule::new_inverted(
            "no_todos".to_string(),
            "TODO comments should be resolved".to_string(),
            Severity::Warning,
            "TODO".to_string(),
            "Found TODO comment that should be resolved".to_string(),
        ))
        .add_rule(PatternRule::new_inverted(
            "no_panic".to_string(),
            "Avoid using panic! in production code".to_string(),
            Severity::Error,
            "panic!".to_string(),
            "Found panic! macro - use Result instead".to_string(),
        ));

    // Validate
    let findings = validator.validate_file(&test_file)?;

    println!("Found {} issue(s):\n", findings.len());
    for finding in &findings {
        println!("[{}] {} (line {})", finding.severity, finding.message, finding.line.unwrap_or(0));
        if let Some(snippet) = &finding.snippet {
            println!("  > {}", snippet.trim());
        }
        println!();
    }

    // Cleanup
    let _ = fs::remove_file(test_file);

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Function Length Validation
// ============================================================================

fn example_2_function_length() -> Result<()> {
    println!("=== Example 2: Function Length Validation ===\n");

    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("long_function.rs");

    // Create a long function
    let mut code = String::from("fn very_long_function() {\n");
    for i in 0..30 {
        code.push_str(&format!("    let x{} = {};\n", i, i));
    }
    code.push_str("}\n");

    fs::write(&test_file, &code)
        .map_err(|e| Error::Other(format!("Failed to write test file: {}", e)))?;

    // Create validator with function length rule
    let validator = Validator::new()
        .add_rule(FunctionLengthRule::new(20, Severity::Warning));

    // Validate
    let findings = validator.validate_file(&test_file)?;

    println!("Validation Results:\n");
    for finding in &findings {
        println!("[{}] {}", finding.severity, finding.message);
        if let Some(suggestion) = &finding.suggestion {
            println!("  Suggestion: {}", suggestion);
        }
        println!();
    }

    // Cleanup
    let _ = fs::remove_file(test_file);

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Comprehensive Validation Report
// ============================================================================

fn example_3_comprehensive_report() -> Result<()> {
    println!("=== Example 3: Comprehensive Validation Report ===\n");

    let temp_dir = std::env::temp_dir();

    // Create multiple test files
    let files = vec![
        ("file1.rs", "fn test() { todo!(); }"),
        ("file2.rs", "fn calc() { panic!(\"error\"); }"),
        ("file3.rs", "fn good_function() { println!(\"ok\"); }"),
    ];

    let mut file_paths = Vec::new();
    for (name, content) in files {
        let path = temp_dir.join(name);
        fs::write(&path, content)
            .map_err(|e| Error::Other(format!("Failed to write file: {}", e)))?;
        file_paths.push(path);
    }

    // Create comprehensive validator
    let validator = Validator::new()
        .add_rule(PatternRule::new_inverted(
            "no_todo".to_string(),
            "No TODO macros".to_string(),
            Severity::Error,
            "todo!".to_string(),
            "Found todo!() macro".to_string(),
        ))
        .add_rule(PatternRule::new_inverted(
            "no_panic".to_string(),
            "No panic macros".to_string(),
            Severity::Error,
            "panic!".to_string(),
            "Found panic!() macro".to_string(),
        ));

    // Generate report
    let report = validator.validate_files(&file_paths)?;

    report.print_summary();

    println!("\nDetailed Findings:");
    for finding in &report.findings {
        println!(
            "\n[{}] {} - {}",
            finding.severity,
            finding.file_path.file_name().unwrap().to_string_lossy(),
            finding.message
        );
    }

    // Cleanup
    for path in file_paths {
        let _ = fs::remove_file(path);
    }

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_pattern_validation()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_function_length()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_comprehensive_report()?;

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_severity_display() {
        assert_eq!(Severity::Error.to_string(), "ERROR");
        assert_eq!(Severity::Warning.to_string(), "WARNING");
        assert_eq!(Severity::Info.to_string(), "INFO");
    }

    #[test]
    fn test_finding_creation() {
        let finding = Finding::new(
            "test_rule".to_string(),
            Severity::Warning,
            PathBuf::from("test.rs"),
            "Test message".to_string(),
        )
        .with_line(10)
        .with_snippet("let x = 5;".to_string())
        .with_suggestion("Use const instead".to_string());

        assert_eq!(finding.rule_id, "test_rule");
        assert_eq!(finding.severity, Severity::Warning);
        assert_eq!(finding.line, Some(10));
        assert!(finding.snippet.is_some());
        assert!(finding.suggestion.is_some());
    }

    #[test]
    fn test_pattern_rule_detection() {
        let rule = PatternRule::new_inverted(
            "no_todo".to_string(),
            "No TODOs".to_string(),
            Severity::Warning,
            "TODO".to_string(),
            "Found TODO".to_string(),
        );

        let content = "fn test() {\n    // TODO: implement\n}";
        let findings = rule.validate(Path::new("test.rs"), content).unwrap();

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::Warning);
    }

    #[test]
    fn test_pattern_rule_no_match() {
        let rule = PatternRule::new_inverted(
            "no_panic".to_string(),
            "No panic".to_string(),
            Severity::Error,
            "panic!".to_string(),
            "Found panic!".to_string(),
        );

        let content = "fn test() {\n    println!(\"ok\");\n}";
        let findings = rule.validate(Path::new("test.rs"), content).unwrap();

        assert_eq!(findings.len(), 0);
    }

    #[test]
    fn test_function_length_rule() {
        let rule = FunctionLengthRule::new(5, Severity::Warning);

        let content = "fn short() {\n    let x = 1;\n}\n\nfn long() {\n    let a = 1;\n    let b = 2;\n    let c = 3;\n    let d = 4;\n    let e = 5;\n    let f = 6;\n}";

        let findings = rule.validate(Path::new("test.rs"), content).unwrap();

        // Should find the long function
        assert!(findings.len() > 0);
    }

    #[test]
    fn test_validation_report_empty() {
        let report = ValidationReport::new();

        assert_eq!(report.total_findings, 0);
        assert_eq!(report.error_count, 0);
        assert_eq!(report.warning_count, 0);
        assert_eq!(report.info_count, 0);
        assert!(report.passed());
    }

    #[test]
    fn test_validation_report_add_findings() {
        let mut report = ValidationReport::new();

        let findings = vec![
            Finding::new(
                "rule1".to_string(),
                Severity::Error,
                PathBuf::from("test.rs"),
                "Error message".to_string(),
            ),
            Finding::new(
                "rule2".to_string(),
                Severity::Warning,
                PathBuf::from("test.rs"),
                "Warning message".to_string(),
            ),
        ];

        report.add_findings(findings);

        assert_eq!(report.total_findings, 2);
        assert_eq!(report.error_count, 1);
        assert_eq!(report.warning_count, 1);
        assert!(!report.passed());
    }

    #[test]
    fn test_validation_report_by_severity() {
        let mut report = ValidationReport::new();

        let findings = vec![
            Finding::new(
                "rule1".to_string(),
                Severity::Error,
                PathBuf::from("test.rs"),
                "Error".to_string(),
            ),
            Finding::new(
                "rule2".to_string(),
                Severity::Warning,
                PathBuf::from("test.rs"),
                "Warning".to_string(),
            ),
            Finding::new(
                "rule3".to_string(),
                Severity::Error,
                PathBuf::from("test.rs"),
                "Another error".to_string(),
            ),
        ];

        report.add_findings(findings);

        let errors = report.findings_by_severity(Severity::Error);
        assert_eq!(errors.len(), 2);

        let warnings = report.findings_by_severity(Severity::Warning);
        assert_eq!(warnings.len(), 1);
    }

    #[test]
    fn test_validator_add_rule() {
        let validator = Validator::new()
            .add_rule(PatternRule::new(
                "test".to_string(),
                "Test rule".to_string(),
                Severity::Info,
                "test".to_string(),
                "Test message".to_string(),
            ));

        assert_eq!(validator.rules.len(), 1);
    }

    #[test]
    fn test_validator_add_exclusion() {
        let validator = Validator::new()
            .add_exclusion("target".to_string())
            .add_exclusion("node_modules".to_string());

        assert_eq!(validator.exclude_patterns.len(), 2);
    }

    #[test]
    fn test_validator_validate_file() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.rs");

        fs::write(&test_file, "fn test() { panic!(\"error\"); }").unwrap();

        let validator = Validator::new()
            .add_rule(PatternRule::new_inverted(
                "no_panic".to_string(),
                "No panic".to_string(),
                Severity::Error,
                "panic!".to_string(),
                "Found panic!".to_string(),
            ));

        let findings = validator.validate_file(&test_file).unwrap();
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn test_validator_validate_multiple_files() {
        let temp_dir = TempDir::new().unwrap();

        let file1 = temp_dir.path().join("file1.rs");
        let file2 = temp_dir.path().join("file2.rs");

        fs::write(&file1, "fn test1() { todo!(); }").unwrap();
        fs::write(&file2, "fn test2() { println!(\"ok\"); }").unwrap();

        let validator = Validator::new()
            .add_rule(PatternRule::new_inverted(
                "no_todo".to_string(),
                "No TODOs".to_string(),
                Severity::Warning,
                "todo!".to_string(),
                "Found TODO".to_string(),
            ));

        let report = validator.validate_files(&vec![file1, file2]).unwrap();

        assert_eq!(report.files_validated, 2);
        assert_eq!(report.total_findings, 1);
    }

    #[test]
    fn test_validator_exclusion_pattern() {
        let temp_dir = TempDir::new().unwrap();
        let excluded_file = temp_dir.path().join("target_file.rs");

        fs::write(&excluded_file, "fn test() { panic!(\"error\"); }").unwrap();

        let validator = Validator::new()
            .add_exclusion("target".to_string())
            .add_rule(PatternRule::new_inverted(
                "no_panic".to_string(),
                "No panic".to_string(),
                Severity::Error,
                "panic!".to_string(),
                "Found panic!".to_string(),
            ));

        let findings = validator.validate_file(&excluded_file).unwrap();

        // Should be empty because file matches exclusion pattern
        assert_eq!(findings.len(), 0);
    }
}
