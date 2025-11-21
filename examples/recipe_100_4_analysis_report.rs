//! # Recipe 100-4: Generate Analysis Report
//!
//! **Level:** 100 (Foundational)
//! **Time Estimate:** 10 hours
//! **Priority:** P1 (High)
//!
//! ## Overview
//!
//! This recipe demonstrates how to generate comprehensive project analysis reports
//! in multiple formats (JSON, Markdown, HTML). Reports include TDG scores, metrics,
//! language distribution, and actionable recommendations.
//!
//! ## Features
//!
//! - **Multiple Formats:** JSON, Markdown, and HTML output
//! - **Rich Metrics:** Lines of code, file counts, language distribution
//! - **TDG Scoring:** Technical debt grade with detailed breakdown
//! - **Recommendations:** Actionable improvement suggestions
//! - **Customization:** Configurable report sections and styling
//!
//! ## Examples
//!
//! Run individual examples with:
//! ```bash
//! cargo run --example recipe_100_4_analysis_report
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_100_4_analysis_report
//! ```

use batuta_cookbook::types::{Grade, Result, TdgScore};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Format number with thousands separator
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result
}

/// Project metrics collected during analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetrics {
    /// Total lines of code
    pub total_lines: usize,
    /// Number of files analyzed
    pub file_count: usize,
    /// Distribution of code by language
    pub language_distribution: HashMap<String, usize>,
    /// Average lines per file
    pub avg_lines_per_file: f64,
    /// Project complexity estimate (0-100)
    pub complexity_score: f64,
}

impl ProjectMetrics {
    /// Create a new ProjectMetrics instance
    pub fn new() -> Self {
        Self {
            total_lines: 0,
            file_count: 0,
            language_distribution: HashMap::new(),
            avg_lines_per_file: 0.0,
            complexity_score: 0.0,
        }
    }

    /// Calculate average lines per file
    pub fn calculate_averages(&mut self) {
        if self.file_count > 0 {
            self.avg_lines_per_file = self.total_lines as f64 / self.file_count as f64;
        }
    }
}

impl Default for ProjectMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Analysis report containing all project insights
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    /// Project name
    pub project_name: String,
    /// Analysis timestamp
    pub timestamp: String,
    /// Project metrics
    pub metrics: ProjectMetrics,
    /// TDG score
    pub tdg_score: TdgScoreData,
    /// Recommendations for improvement
    pub recommendations: Vec<String>,
    /// Warnings and issues found
    pub warnings: Vec<String>,
}

/// Serializable TDG score data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TdgScoreData {
    /// Score value (0-100)
    pub score: f64,
    /// Letter grade
    pub grade: String,
    /// Detailed breakdown
    pub breakdown: HashMap<String, f64>,
}

impl From<TdgScore> for TdgScoreData {
    fn from(tdg: TdgScore) -> Self {
        let mut breakdown = HashMap::new();
        breakdown.insert("Test Coverage".to_string(), 85.0);
        breakdown.insert("Documentation".to_string(), 90.0);
        breakdown.insert("Code Complexity".to_string(), tdg.score);
        breakdown.insert("Security".to_string(), 95.0);

        Self {
            score: tdg.score,
            grade: tdg.grade.to_string(),
            breakdown,
        }
    }
}

/// Report output format
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReportFormat {
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
    /// HTML format
    Html,
}

impl ReportFormat {
    /// Get file extension for this format
    pub fn extension(self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Markdown => "md",
            Self::Html => "html",
        }
    }
}

/// Report generator
pub struct ReportGenerator {
    /// Report format
    format: ReportFormat,
    /// Whether to include recommendations
    include_recommendations: bool,
    /// Whether to include detailed metrics
    include_detailed_metrics: bool,
}

impl ReportGenerator {
    /// Create a new ReportGenerator with specified format
    pub fn new(format: ReportFormat) -> Self {
        Self {
            format,
            include_recommendations: true,
            include_detailed_metrics: true,
        }
    }

    /// Set whether to include recommendations
    pub fn with_recommendations(mut self, include: bool) -> Self {
        self.include_recommendations = include;
        self
    }

    /// Set whether to include detailed metrics
    pub fn with_detailed_metrics(mut self, include: bool) -> Self {
        self.include_detailed_metrics = include;
        self
    }

    /// Generate report from analysis data
    pub fn generate(&self, report: &AnalysisReport) -> Result<String> {
        match self.format {
            ReportFormat::Json => self.generate_json(report),
            ReportFormat::Markdown => self.generate_markdown(report),
            ReportFormat::Html => self.generate_html(report),
        }
    }

    /// Generate JSON report
    fn generate_json(&self, report: &AnalysisReport) -> Result<String> {
        let json = serde_json::to_string_pretty(report)
            .map_err(|e| batuta_cookbook::Error::Other(format!("JSON generation failed: {}", e)))?;
        Ok(json)
    }

    /// Generate Markdown report
    fn generate_markdown(&self, report: &AnalysisReport) -> Result<String> {
        let mut md = String::new();

        // Header
        md.push_str(&format!("# Analysis Report: {}\n\n", report.project_name));
        md.push_str(&format!("**Generated:** {}\n\n", report.timestamp));

        // TDG Score
        md.push_str("## 📊 Technical Debt Grade\n\n");
        md.push_str(&format!(
            "**Overall Score:** {} ({})\n\n",
            report.tdg_score.score, report.tdg_score.grade
        ));

        if self.include_detailed_metrics {
            md.push_str("### Score Breakdown\n\n");
            let mut breakdown: Vec<_> = report.tdg_score.breakdown.iter().collect();
            breakdown.sort_by_key(|(k, _)| *k);
            for (category, score) in breakdown {
                md.push_str(&format!("- **{}:** {:.1}/100\n", category, score));
            }
            md.push_str("\n");
        }

        // Metrics
        md.push_str("## 📈 Project Metrics\n\n");
        md.push_str(&format!("- **Total Lines of Code:** {}\n", format_number(report.metrics.total_lines)));
        md.push_str(&format!("- **Files Analyzed:** {}\n", report.metrics.file_count));
        md.push_str(&format!(
            "- **Average Lines per File:** {:.1}\n",
            report.metrics.avg_lines_per_file
        ));
        md.push_str(&format!(
            "- **Complexity Score:** {:.1}/100\n\n",
            report.metrics.complexity_score
        ));

        // Language Distribution
        if !report.metrics.language_distribution.is_empty() {
            md.push_str("### Language Distribution\n\n");
            let mut langs: Vec<_> = report.metrics.language_distribution.iter().collect();
            langs.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
            for (lang, lines) in langs {
                let percentage =
                    (*lines as f64 / report.metrics.total_lines as f64) * 100.0;
                md.push_str(&format!("- **{}:** {} lines ({:.1}%)\n", lang, format_number(*lines), percentage));
            }
            md.push_str("\n");
        }

        // Warnings
        if !report.warnings.is_empty() {
            md.push_str("## ⚠️ Warnings\n\n");
            for warning in &report.warnings {
                md.push_str(&format!("- {}\n", warning));
            }
            md.push_str("\n");
        }

        // Recommendations
        if self.include_recommendations && !report.recommendations.is_empty() {
            md.push_str("## 💡 Recommendations\n\n");
            for (i, rec) in report.recommendations.iter().enumerate() {
                md.push_str(&format!("{}. {}\n", i + 1, rec));
            }
            md.push_str("\n");
        }

        Ok(md)
    }

    /// Generate HTML report
    fn generate_html(&self, report: &AnalysisReport) -> Result<String> {
        let mut html = String::new();

        html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str(&format!("    <title>Analysis Report - {}</title>\n", report.project_name));
        html.push_str("    <style>\n");
        html.push_str(REPORT_CSS);
        html.push_str("    </style>\n");
        html.push_str("</head>\n<body>\n");

        // Header
        html.push_str(&format!(
            "    <div class=\"container\">\n        <h1>📊 Analysis Report: {}</h1>\n",
            report.project_name
        ));
        html.push_str(&format!("        <p class=\"timestamp\">Generated: {}</p>\n\n", report.timestamp));

        // TDG Score Card
        let grade_class = match report.tdg_score.grade.as_str() {
            "A+" | "A" => "grade-a",
            "A-" | "B+" | "B" => "grade-b",
            _ => "grade-c",
        };
        html.push_str("        <div class=\"score-card\">\n");
        html.push_str("            <h2>Technical Debt Grade</h2>\n");
        html.push_str(&format!(
            "            <div class=\"score {}\">{}</div>\n",
            grade_class, report.tdg_score.grade
        ));
        html.push_str(&format!(
            "            <p class=\"score-value\">{:.1}/100</p>\n",
            report.tdg_score.score
        ));
        html.push_str("        </div>\n\n");

        // Metrics
        html.push_str("        <div class=\"metrics\">\n");
        html.push_str("            <h2>Project Metrics</h2>\n");
        html.push_str("            <table>\n");
        html.push_str(&format!(
            "                <tr><td>Total Lines of Code</td><td>{}</td></tr>\n",
            format_number(report.metrics.total_lines)
        ));
        html.push_str(&format!(
            "                <tr><td>Files Analyzed</td><td>{}</td></tr>\n",
            report.metrics.file_count
        ));
        html.push_str(&format!(
            "                <tr><td>Average Lines per File</td><td>{:.1}</td></tr>\n",
            report.metrics.avg_lines_per_file
        ));
        html.push_str(&format!(
            "                <tr><td>Complexity Score</td><td>{:.1}/100</td></tr>\n",
            report.metrics.complexity_score
        ));
        html.push_str("            </table>\n");
        html.push_str("        </div>\n\n");

        // Recommendations
        if self.include_recommendations && !report.recommendations.is_empty() {
            html.push_str("        <div class=\"recommendations\">\n");
            html.push_str("            <h2>💡 Recommendations</h2>\n");
            html.push_str("            <ol>\n");
            for rec in &report.recommendations {
                html.push_str(&format!("                <li>{}</li>\n", rec));
            }
            html.push_str("            </ol>\n");
            html.push_str("        </div>\n");
        }

        html.push_str("    </div>\n");
        html.push_str("</body>\n</html>");

        Ok(html)
    }

    /// Write report to file
    pub fn write_to_file(&self, report: &AnalysisReport, output_path: &Path) -> Result<()> {
        let content = self.generate(report)?;
        fs::write(output_path, content).map_err(|e| {
            batuta_cookbook::Error::Other(format!("Failed to write report: {}", e))
        })?;
        Ok(())
    }
}

/// Simple CSS for HTML reports (embedded)
const REPORT_CSS: &str = r#"
body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
.container { max-width: 900px; margin: 0 auto; background: white; padding: 40px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }
h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }
h2 { color: #555; margin-top: 30px; }
.timestamp { color: #777; font-size: 0.9em; }
.score-card { text-align: center; padding: 30px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; border-radius: 8px; margin: 20px 0; }
.score { font-size: 4em; font-weight: bold; margin: 20px 0; }
.score-value { font-size: 1.2em; opacity: 0.9; }
.grade-a { color: #4CAF50; }
.grade-b { color: #FFC107; }
.grade-c { color: #F44336; }
.metrics table { width: 100%; border-collapse: collapse; }
.metrics td { padding: 12px; border-bottom: 1px solid #eee; }
.metrics td:first-child { font-weight: bold; color: #555; }
.metrics td:last-child { text-align: right; color: #333; }
.recommendations { background: #E8F5E9; padding: 20px; border-radius: 8px; margin-top: 20px; }
.recommendations ol { margin: 0; padding-left: 20px; }
.recommendations li { margin: 10px 0; color: #2E7D32; }
"#;

// ============================================================================
// EXAMPLE 1: Generate JSON Report
// ============================================================================

fn example_1_json_report() -> Result<()> {
    println!("=== Example 1: Generate JSON Report ===\n");

    // Create sample analysis data
    let mut metrics = ProjectMetrics::new();
    metrics.total_lines = 5420;
    metrics.file_count = 42;
    metrics.language_distribution.insert("Rust".to_string(), 3800);
    metrics.language_distribution.insert("Python".to_string(), 1200);
    metrics.language_distribution.insert("JavaScript".to_string(), 420);
    metrics.complexity_score = 72.5;
    metrics.calculate_averages();

    let tdg_score = TdgScore {
        score: 87.3,
        grade: Grade::from_score(87.3),
    };

    let report = AnalysisReport {
        project_name: "sample-project".to_string(),
        timestamp: "2025-11-21T10:30:00Z".to_string(),
        metrics,
        tdg_score: tdg_score.into(),
        recommendations: vec![
            "Consider increasing test coverage to 90%".to_string(),
            "Reduce cyclomatic complexity in module 'core'".to_string(),
            "Add API documentation for public functions".to_string(),
        ],
        warnings: vec!["Found 3 TODO comments in codebase".to_string()],
    };

    // Generate JSON report
    let generator = ReportGenerator::new(ReportFormat::Json);
    let json_output = generator.generate(&report)?;

    println!("{}\n", json_output);

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Generate Markdown Report
// ============================================================================

fn example_2_markdown_report() -> Result<()> {
    println!("=== Example 2: Generate Markdown Report ===\n");

    let mut metrics = ProjectMetrics::new();
    metrics.total_lines = 12840;
    metrics.file_count = 89;
    metrics.language_distribution.insert("Rust".to_string(), 8200);
    metrics.language_distribution.insert("TOML".to_string(), 320);
    metrics.language_distribution.insert("Markdown".to_string(), 4320);
    metrics.complexity_score = 65.8;
    metrics.calculate_averages();

    let tdg_score = TdgScore {
        score: 92.1,
        grade: Grade::from_score(92.1),
    };

    let report = AnalysisReport {
        project_name: "batuta-cookbook".to_string(),
        timestamp: "2025-11-21T10:35:00Z".to_string(),
        metrics,
        tdg_score: tdg_score.into(),
        recommendations: vec![
            "Excellent code quality! Maintain current standards".to_string(),
            "Consider adding performance benchmarks".to_string(),
        ],
        warnings: vec![],
    };

    // Generate Markdown report
    let generator = ReportGenerator::new(ReportFormat::Markdown)
        .with_recommendations(true)
        .with_detailed_metrics(true);

    let md_output = generator.generate(&report)?;

    println!("{}", md_output);

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Generate and Save Multiple Report Formats
// ============================================================================

fn example_3_save_reports() -> Result<()> {
    println!("=== Example 3: Generate and Save Multiple Report Formats ===\n");

    let mut metrics = ProjectMetrics::new();
    metrics.total_lines = 8500;
    metrics.file_count = 64;
    metrics.language_distribution.insert("Rust".to_string(), 7000);
    metrics.language_distribution.insert("Shell".to_string(), 1500);
    metrics.complexity_score = 78.2;
    metrics.calculate_averages();

    let tdg_score = TdgScore {
        score: 85.5,
        grade: Grade::from_score(85.5),
    };

    let report = AnalysisReport {
        project_name: "multi-format-demo".to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        metrics,
        tdg_score: tdg_score.into(),
        recommendations: vec![
            "Add integration tests for API endpoints".to_string(),
            "Document deployment procedures".to_string(),
        ],
        warnings: vec!["High complexity in module 'parser'".to_string()],
    };

    // Generate all formats
    let formats = vec![
        (ReportFormat::Json, "report.json"),
        (ReportFormat::Markdown, "report.md"),
        (ReportFormat::Html, "report.html"),
    ];

    for (format, filename) in formats {
        let generator = ReportGenerator::new(format);
        let output_path = Path::new("/tmp").join(filename);

        generator.write_to_file(&report, &output_path)?;
        println!("✓ Generated {} report: {}", format.extension(), output_path.display());
    }

    println!("\nAll reports generated successfully!");

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_json_report()?;
    println!("{}\n", "=".repeat(70));

    example_2_markdown_report()?;
    println!("{}\n", "=".repeat(70));

    example_3_save_reports()?;

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
    fn test_project_metrics_default() {
        let metrics = ProjectMetrics::default();
        assert_eq!(metrics.total_lines, 0);
        assert_eq!(metrics.file_count, 0);
        assert_eq!(metrics.avg_lines_per_file, 0.0);
    }

    #[test]
    fn test_calculate_averages() {
        let mut metrics = ProjectMetrics::new();
        metrics.total_lines = 1000;
        metrics.file_count = 10;
        metrics.calculate_averages();

        assert_eq!(metrics.avg_lines_per_file, 100.0);
    }

    #[test]
    fn test_tdg_score_conversion() {
        let tdg = TdgScore {
            score: 92.5,
            grade: Grade::A,
        };

        let data: TdgScoreData = tdg.into();
        assert_eq!(data.score, 92.5);
        assert_eq!(data.grade, "A");
        assert!(!data.breakdown.is_empty());
    }

    #[test]
    fn test_report_format_extension() {
        assert_eq!(ReportFormat::Json.extension(), "json");
        assert_eq!(ReportFormat::Markdown.extension(), "md");
        assert_eq!(ReportFormat::Html.extension(), "html");
    }

    #[test]
    fn test_generate_json_report() {
        let metrics = ProjectMetrics::default();
        let tdg = TdgScore {
            score: 85.0,
            grade: Grade::AMinus,
        };

        let report = AnalysisReport {
            project_name: "test-project".to_string(),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics,
            tdg_score: tdg.into(),
            recommendations: vec!["Test recommendation".to_string()],
            warnings: vec![],
        };

        let generator = ReportGenerator::new(ReportFormat::Json);
        let json = generator.generate(&report).unwrap();

        assert!(json.contains("test-project"));
        assert!(json.contains("85"));
        assert!(json.contains("Test recommendation"));
    }

    #[test]
    fn test_generate_markdown_report() {
        let mut metrics = ProjectMetrics::new();
        metrics.total_lines = 1000;
        metrics.file_count = 10;
        metrics.calculate_averages();

        let tdg = TdgScore {
            score: 90.0,
            grade: Grade::A,
        };

        let report = AnalysisReport {
            project_name: "markdown-test".to_string(),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics,
            tdg_score: tdg.into(),
            recommendations: vec!["Improve tests".to_string()],
            warnings: vec!["Warning 1".to_string()],
        };

        let generator = ReportGenerator::new(ReportFormat::Markdown);
        let md = generator.generate(&report).unwrap();

        assert!(md.contains("# Analysis Report: markdown-test"));
        assert!(md.contains("90"));
        assert!(md.contains("A"));
        assert!(md.contains("1,000"));
        assert!(md.contains("Improve tests"));
        assert!(md.contains("Warning 1"));
    }

    #[test]
    fn test_generate_html_report() {
        let metrics = ProjectMetrics::default();
        let tdg = TdgScore {
            score: 75.0,
            grade: Grade::B,
        };

        let report = AnalysisReport {
            project_name: "html-test".to_string(),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics,
            tdg_score: tdg.into(),
            recommendations: vec![],
            warnings: vec![],
        };

        let generator = ReportGenerator::new(ReportFormat::Html);
        let html = generator.generate(&report).unwrap();

        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("html-test"));
        assert!(html.contains("75"));
        assert!(html.contains("B"));
    }

    #[test]
    fn test_write_report_to_file() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("test_report.json");

        let metrics = ProjectMetrics::default();
        let tdg = TdgScore {
            score: 80.0,
            grade: Grade::BPlus,
        };

        let report = AnalysisReport {
            project_name: "file-test".to_string(),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics,
            tdg_score: tdg.into(),
            recommendations: vec![],
            warnings: vec![],
        };

        let generator = ReportGenerator::new(ReportFormat::Json);
        generator.write_to_file(&report, &output_path).unwrap();

        assert!(output_path.exists());

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("file-test"));
    }

    #[test]
    fn test_report_generator_with_options() {
        let generator = ReportGenerator::new(ReportFormat::Markdown)
            .with_recommendations(false)
            .with_detailed_metrics(false);

        assert!(!generator.include_recommendations);
        assert!(!generator.include_detailed_metrics);
    }

    #[test]
    fn test_language_distribution_percentage() {
        let mut metrics = ProjectMetrics::new();
        metrics.total_lines = 1000;
        metrics.language_distribution.insert("Rust".to_string(), 700);
        metrics.language_distribution.insert("Python".to_string(), 300);

        let tdg = TdgScore {
            score: 85.0,
            grade: Grade::AMinus,
        };

        let report = AnalysisReport {
            project_name: "lang-dist-test".to_string(),
            timestamp: "2025-11-21T00:00:00Z".to_string(),
            metrics,
            tdg_score: tdg.into(),
            recommendations: vec![],
            warnings: vec![],
        };

        let generator = ReportGenerator::new(ReportFormat::Markdown);
        let md = generator.generate(&report).unwrap();

        assert!(md.contains("70.0%")); // Rust percentage
        assert!(md.contains("30.0%")); // Python percentage
    }
}
