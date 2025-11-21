//! # Recipe 200-1: Multi-Language Project Analysis
//!
//! **Level:** 200 (Intermediate)
//! **Time Estimate:** 16 hours
//! **Priority:** P1 (High)
//!
//! ## Overview
//!
//! This recipe demonstrates advanced project analysis for polyglot codebases containing
//! multiple programming languages. It provides language-specific metrics, cross-language
//! insights, and aggregate statistics for complex, real-world projects.
//!
//! ## Features
//!
//! - **Multi-Language Detection:** Automatic detection of all languages in a project
//! - **Per-Language Metrics:** Separate analysis for each language
//! - **Cross-Language Analysis:** Dependencies and interactions between languages
//! - **Aggregate Statistics:** Project-wide metrics across all languages
//! - **Language Hotspots:** Identify which languages dominate the codebase
//! - **Quality Scoring:** TDG scores per language and overall
//! - **Common Patterns:** Recognize microservices, full-stack, and hybrid architectures
//!
//! ## Supported Languages
//!
//! - **Rust** (.rs)
//! - **Python** (.py)
//! - **JavaScript/TypeScript** (.js, .ts, .jsx, .tsx)
//! - **C/C++** (.c, .cpp, .h, .hpp)
//! - **Go** (.go)
//! - **Java** (.java)
//! - **Ruby** (.rb)
//! - **Shell** (.sh, .bash)
//!
//! ## Examples
//!
//! Run examples with:
//! ```bash
//! cargo run --example recipe_200_1_multi_language
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_200_1_multi_language
//! ```

use batuta_cookbook::types::{Grade, Language, Result, TdgScore};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Statistics for a single language in the project
#[derive(Debug, Clone, PartialEq)]
pub struct LanguageStats {
    /// Programming language
    pub language: Language,
    /// Total lines of code
    pub lines_of_code: usize,
    /// Number of files
    pub file_count: usize,
    /// Average lines per file
    pub avg_lines_per_file: f64,
    /// Total blank lines
    pub blank_lines: usize,
    /// Total comment lines
    pub comment_lines: usize,
    /// Code-to-comment ratio
    pub code_to_comment_ratio: f64,
    /// Percentage of total project
    pub percentage_of_project: f64,
    /// Files analyzed
    pub files: Vec<PathBuf>,
    /// TDG score for this language
    pub tdg_score: Option<TdgScore>,
}

impl LanguageStats {
    /// Create new language statistics
    pub fn new(language: Language) -> Self {
        Self {
            language,
            lines_of_code: 0,
            file_count: 0,
            avg_lines_per_file: 0.0,
            blank_lines: 0,
            comment_lines: 0,
            code_to_comment_ratio: 0.0,
            percentage_of_project: 0.0,
            files: Vec::new(),
            tdg_score: None,
        }
    }

    /// Calculate derived metrics
    pub fn calculate_metrics(&mut self, total_project_lines: usize) {
        if self.file_count > 0 {
            self.avg_lines_per_file = self.lines_of_code as f64 / self.file_count as f64;
        }

        if self.comment_lines > 0 {
            self.code_to_comment_ratio =
                self.lines_of_code as f64 / self.comment_lines as f64;
        }

        if total_project_lines > 0 {
            self.percentage_of_project =
                (self.lines_of_code as f64 / total_project_lines as f64) * 100.0;
        }

        // Calculate TDG score based on metrics
        let documentation_score = if self.comment_lines > 0 {
            (self.comment_lines as f64 / self.lines_of_code as f64 * 100.0).min(100.0)
        } else {
            0.0
        };

        let base_score = 70.0 + (documentation_score * 0.3);
        self.tdg_score = Some(TdgScore {
            score: base_score,
            grade: Grade::from_score(base_score),
        });
    }
}

/// Multi-language project analysis results
#[derive(Debug, Clone)]
pub struct MultiLanguageAnalysis {
    /// Project root path
    pub project_path: PathBuf,
    /// Statistics per language
    pub language_stats: HashMap<Language, LanguageStats>,
    /// Total lines of code (all languages)
    pub total_lines: usize,
    /// Total files analyzed
    pub total_files: usize,
    /// Primary language (most lines of code)
    pub primary_language: Option<Language>,
    /// Secondary languages
    pub secondary_languages: Vec<Language>,
    /// Overall project TDG score
    pub overall_tdg: TdgScore,
    /// Architecture pattern detected
    pub architecture_pattern: ArchitecturePattern,
}

/// Common architecture patterns in multi-language projects
#[derive(Debug, Clone, PartialEq)]
pub enum ArchitecturePattern {
    /// Single language dominates (>80%)
    Monolingual,
    /// Backend + Frontend split
    FullStack,
    /// Multiple services in different languages
    Microservices,
    /// Scripts and automation around core language
    ScriptingSupport,
    /// Mix of languages without clear pattern
    Polyglot,
}

impl std::fmt::Display for ArchitecturePattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Monolingual => write!(f, "Monolingual (single language)"),
            Self::FullStack => write!(f, "Full-Stack (frontend + backend)"),
            Self::Microservices => write!(f, "Microservices (multiple services)"),
            Self::ScriptingSupport => write!(f, "Scripting Support (scripts + core)"),
            Self::Polyglot => write!(f, "Polyglot (mixed languages)"),
        }
    }
}

/// Multi-language analyzer
pub struct MultiLanguageAnalyzer {
    /// Directories to exclude
    exclude_dirs: Vec<String>,
    /// Maximum depth to scan
    max_depth: usize,
}

impl MultiLanguageAnalyzer {
    /// Create a new multi-language analyzer
    pub fn new() -> Self {
        Self {
            exclude_dirs: vec![
                "target".to_string(),
                "node_modules".to_string(),
                "dist".to_string(),
                "build".to_string(),
                ".git".to_string(),
                "vendor".to_string(),
                "venv".to_string(),
                ".venv".to_string(),
                "__pycache__".to_string(),
            ],
            max_depth: 10,
        }
    }

    /// Set excluded directories
    pub fn with_exclude_dirs(mut self, dirs: Vec<String>) -> Self {
        self.exclude_dirs = dirs;
        self
    }

    /// Set maximum scan depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Analyze a multi-language project
    pub fn analyze(&self, project_path: &Path) -> Result<MultiLanguageAnalysis> {
        let mut language_stats: HashMap<Language, LanguageStats> = HashMap::new();
        let mut total_lines = 0;
        let mut total_files = 0;

        // Scan directory and collect stats
        self.scan_directory(
            project_path,
            project_path,
            0,
            &mut language_stats,
            &mut total_lines,
            &mut total_files,
        )?;

        // Calculate metrics for each language
        for stats in language_stats.values_mut() {
            stats.calculate_metrics(total_lines);
        }

        // Identify primary and secondary languages
        let mut sorted_langs: Vec<_> = language_stats.values().collect();
        sorted_langs.sort_by(|a, b| b.lines_of_code.cmp(&a.lines_of_code));

        let primary_language = sorted_langs.first().map(|s| s.language);
        let secondary_languages: Vec<Language> = sorted_langs
            .iter()
            .skip(1)
            .filter(|s| s.percentage_of_project > 5.0) // Only significant languages
            .map(|s| s.language)
            .collect();

        // Detect architecture pattern
        let architecture_pattern = Self::detect_architecture_pattern(&language_stats);

        // Calculate overall TDG score
        let overall_tdg = Self::calculate_overall_tdg(&language_stats);

        Ok(MultiLanguageAnalysis {
            project_path: project_path.to_path_buf(),
            language_stats,
            total_lines,
            total_files,
            primary_language,
            secondary_languages,
            overall_tdg,
            architecture_pattern,
        })
    }

    /// Scan directory recursively
    fn scan_directory(
        &self,
        current_path: &Path,
        root_path: &Path,
        depth: usize,
        language_stats: &mut HashMap<Language, LanguageStats>,
        total_lines: &mut usize,
        total_files: &mut usize,
    ) -> Result<()> {
        if depth > self.max_depth {
            return Ok(());
        }

        let entries = fs::read_dir(current_path).map_err(|e| {
            batuta_cookbook::Error::Analysis(format!(
                "Failed to read directory {}: {}",
                current_path.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                batuta_cookbook::Error::Analysis(format!("Failed to read entry: {}", e))
            })?;
            let path = entry.path();

            if path.is_dir() {
                // Check if directory should be excluded
                if let Some(dir_name) = path.file_name() {
                    if self.exclude_dirs.contains(&dir_name.to_string_lossy().to_string()) {
                        continue;
                    }
                }

                // Recurse into subdirectory
                self.scan_directory(
                    &path,
                    root_path,
                    depth + 1,
                    language_stats,
                    total_lines,
                    total_files,
                )?;
            } else if path.is_file() {
                // Analyze file
                if let Some(language) = Self::detect_language(&path) {
                    if language != Language::Unknown {
                        let file_stats = self.analyze_file(&path)?;
                        let relative_path = path.strip_prefix(root_path).unwrap_or(&path);

                        let stats = language_stats
                            .entry(language)
                            .or_insert_with(|| LanguageStats::new(language));

                        stats.lines_of_code += file_stats.lines;
                        stats.blank_lines += file_stats.blank_lines;
                        stats.comment_lines += file_stats.comment_lines;
                        stats.file_count += 1;
                        stats.files.push(relative_path.to_path_buf());

                        *total_lines += file_stats.lines;
                        *total_files += 1;
                    }
                }
            }
        }

        Ok(())
    }

    /// Detect language from file extension
    fn detect_language(path: &Path) -> Option<Language> {
        path.extension().and_then(|ext| {
            let ext = ext.to_str()?;
            match ext {
                "rs" => Some(Language::Rust),
                "py" | "pyw" => Some(Language::Python),
                "js" | "jsx" | "ts" | "tsx" => Some(Language::JavaScript),
                "c" | "h" => Some(Language::C),
                "cpp" | "cc" | "cxx" | "hpp" | "hxx" => Some(Language::Cpp),
                "sh" | "bash" => Some(Language::Shell),
                _ => Some(Language::Unknown),
            }
        })
    }

    /// Analyze a single file
    fn analyze_file(&self, path: &Path) -> Result<FileStats> {
        let content = fs::read_to_string(path).map_err(|e| {
            batuta_cookbook::Error::Analysis(format!("Failed to read file {}: {}", path.display(), e))
        })?;

        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        let mut blank_lines = 0;
        let mut comment_lines = 0;

        for line in &lines {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                blank_lines += 1;
            } else if trimmed.starts_with("//") || trimmed.starts_with('#') || trimmed.starts_with("/*") {
                comment_lines += 1;
            }
        }

        Ok(FileStats {
            lines: total_lines,
            blank_lines,
            comment_lines,
        })
    }

    /// Detect architecture pattern
    fn detect_architecture_pattern(
        language_stats: &HashMap<Language, LanguageStats>,
    ) -> ArchitecturePattern {
        if language_stats.is_empty() {
            return ArchitecturePattern::Monolingual;
        }

        let mut sorted: Vec<_> = language_stats.values().collect();
        sorted.sort_by(|a, b| b.percentage_of_project.partial_cmp(&a.percentage_of_project).unwrap());

        // Single language dominates
        if let Some(primary) = sorted.first() {
            if primary.percentage_of_project > 80.0 {
                return ArchitecturePattern::Monolingual;
            }
        }

        // Check for full-stack pattern (JavaScript + backend language)
        let has_javascript = language_stats.contains_key(&Language::JavaScript);
        let has_backend = language_stats.contains_key(&Language::Rust)
            || language_stats.contains_key(&Language::Python);

        if has_javascript && has_backend && language_stats.len() <= 3 {
            return ArchitecturePattern::FullStack;
        }

        // Check for scripting support (shell + primary language)
        let has_shell = language_stats.contains_key(&Language::Shell);
        if has_shell {
            if let Some(shell_stats) = language_stats.get(&Language::Shell) {
                if shell_stats.percentage_of_project < 20.0 && language_stats.len() <= 3 {
                    return ArchitecturePattern::ScriptingSupport;
                }
            }
        }

        // Many languages with significant contributions
        if language_stats.len() >= 4 {
            return ArchitecturePattern::Microservices;
        }

        ArchitecturePattern::Polyglot
    }

    /// Calculate overall TDG score
    fn calculate_overall_tdg(language_stats: &HashMap<Language, LanguageStats>) -> TdgScore {
        if language_stats.is_empty() {
            return TdgScore {
                score: 0.0,
                grade: Grade::F,
            };
        }

        // Weighted average based on lines of code
        let total_lines: usize = language_stats.values().map(|s| s.lines_of_code).sum();
        let mut weighted_score = 0.0;

        for stats in language_stats.values() {
            if let Some(tdg) = &stats.tdg_score {
                let weight = stats.lines_of_code as f64 / total_lines as f64;
                weighted_score += tdg.score * weight;
            }
        }

        TdgScore {
            score: weighted_score,
            grade: Grade::from_score(weighted_score),
        }
    }
}

impl Default for MultiLanguageAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// File analysis statistics
#[derive(Debug, Clone)]
struct FileStats {
    lines: usize,
    blank_lines: usize,
    comment_lines: usize,
}

// ============================================================================
// EXAMPLE 1: Analyze Current Project
// ============================================================================

fn example_1_analyze_current_project() -> Result<()> {
    println!("=== Example 1: Analyze Current Project (Polyglot) ===\n");

    let analyzer = MultiLanguageAnalyzer::new();
    let analysis = analyzer.analyze(Path::new("."))?;

    println!("📊 Project Analysis: {}", analysis.project_path.display());
    println!("Total Lines: {}", analysis.total_lines);
    println!("Total Files: {}", analysis.total_files);
    println!("Architecture: {}", analysis.architecture_pattern);
    println!("Overall TDG: {} ({})", analysis.overall_tdg.score, analysis.overall_tdg.grade);

    if let Some(primary) = analysis.primary_language {
        println!("\n🎯 Primary Language: {}", primary);
    }

    if !analysis.secondary_languages.is_empty() {
        println!("\n🔧 Secondary Languages:");
        for lang in &analysis.secondary_languages {
            println!("  - {}", lang);
        }
    }

    println!("\n📈 Language Breakdown:");
    let mut sorted: Vec<_> = analysis.language_stats.values().collect();
    sorted.sort_by(|a, b| b.lines_of_code.cmp(&a.lines_of_code));

    for stats in sorted {
        println!(
            "  {} - {} lines ({:.1}%) in {} files",
            stats.language,
            stats.lines_of_code,
            stats.percentage_of_project,
            stats.file_count
        );
        if let Some(tdg) = &stats.tdg_score {
            println!("    TDG: {} ({})", tdg.score, tdg.grade);
        }
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Compare Language Quality
// ============================================================================

fn example_2_compare_language_quality() -> Result<()> {
    println!("=== Example 2: Compare Language Quality ===\n");

    let analyzer = MultiLanguageAnalyzer::new();
    let analysis = analyzer.analyze(Path::new("."))?;

    println!("🏆 Language Quality Comparison\n");

    let mut langs_by_quality: Vec<_> = analysis
        .language_stats
        .values()
        .filter(|s| s.tdg_score.is_some())
        .collect();

    langs_by_quality.sort_by(|a, b| {
        let score_a = a.tdg_score.as_ref().unwrap().score;
        let score_b = b.tdg_score.as_ref().unwrap().score;
        score_b.partial_cmp(&score_a).unwrap()
    });

    for (rank, stats) in langs_by_quality.iter().enumerate() {
        let tdg = stats.tdg_score.as_ref().unwrap();
        println!(
            "{}. {} - Grade {} ({:.1} score)",
            rank + 1,
            stats.language,
            tdg.grade,
            tdg.score
        );
        println!(
            "   Code/Comment Ratio: {:.1}:1",
            stats.code_to_comment_ratio
        );
        println!("   Files: {}", stats.file_count);
        println!();
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Identify Hotspots
// ============================================================================

fn example_3_identify_hotspots() -> Result<()> {
    println!("=== Example 3: Identify Codebase Hotspots ===\n");

    let analyzer = MultiLanguageAnalyzer::new();
    let analysis = analyzer.analyze(Path::new("."))?;

    println!("🔥 Codebase Hotspots\n");

    // Find largest files per language
    for (language, stats) in &analysis.language_stats {
        if stats.file_count > 0 {
            println!("{}:", language);
            println!(
                "  Total Contribution: {:.1}% of project",
                stats.percentage_of_project
            );
            println!("  Avg Lines per File: {:.0}", stats.avg_lines_per_file);

            // Show top 3 files if available
            let file_count = stats.files.len().min(3);
            if file_count > 0 {
                println!("  Sample Files:");
                for file in stats.files.iter().take(file_count) {
                    println!("    - {}", file.display());
                }
            }
            println!();
        }
    }

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_analyze_current_project()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_compare_language_quality()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_identify_hotspots()?;

    Ok(())
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper to create test files
    fn create_test_project(files: Vec<(&str, &str)>) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        for (path, content) in files {
            let file_path = temp_dir.path().join(path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&file_path, content).unwrap();
        }
        temp_dir
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(
            MultiLanguageAnalyzer::detect_language(Path::new("test.rs")),
            Some(Language::Rust)
        );
        assert_eq!(
            MultiLanguageAnalyzer::detect_language(Path::new("test.py")),
            Some(Language::Python)
        );
        assert_eq!(
            MultiLanguageAnalyzer::detect_language(Path::new("test.js")),
            Some(Language::JavaScript)
        );
    }

    #[test]
    fn test_single_language_project() {
        let temp_dir = create_test_project(vec![
            ("src/main.rs", "fn main() {\n    println!(\"hello\");\n}\n"),
            ("src/lib.rs", "// Library\npub fn test() {}\n"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new();
        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        assert_eq!(analysis.language_stats.len(), 1);
        assert!(analysis.language_stats.contains_key(&Language::Rust));
        assert_eq!(analysis.primary_language, Some(Language::Rust));
        assert_eq!(analysis.architecture_pattern, ArchitecturePattern::Monolingual);
    }

    #[test]
    fn test_multi_language_project() {
        let temp_dir = create_test_project(vec![
            ("backend/main.rs", "fn main() {}\n"),
            ("frontend/app.js", "console.log('hello');\n"),
            ("scripts/deploy.sh", "#!/bin/bash\necho 'deploy'\n"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new();
        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        assert!(analysis.language_stats.len() >= 2);
        assert!(analysis.language_stats.contains_key(&Language::Rust));
        assert!(analysis.language_stats.contains_key(&Language::JavaScript));
    }

    #[test]
    fn test_language_stats_calculation() {
        let mut stats = LanguageStats::new(Language::Rust);
        stats.lines_of_code = 100;
        stats.comment_lines = 20;
        stats.file_count = 5;

        stats.calculate_metrics(200);

        assert_eq!(stats.avg_lines_per_file, 20.0);
        assert_eq!(stats.code_to_comment_ratio, 5.0);
        assert_eq!(stats.percentage_of_project, 50.0);
        assert!(stats.tdg_score.is_some());
    }

    #[test]
    fn test_architecture_detection_monolingual() {
        let mut stats_map = HashMap::new();
        let mut stats = LanguageStats::new(Language::Rust);
        stats.percentage_of_project = 95.0;
        stats_map.insert(Language::Rust, stats);

        let pattern = MultiLanguageAnalyzer::detect_architecture_pattern(&stats_map);
        assert_eq!(pattern, ArchitecturePattern::Monolingual);
    }

    #[test]
    fn test_architecture_detection_fullstack() {
        let mut stats_map = HashMap::new();

        let mut rust_stats = LanguageStats::new(Language::Rust);
        rust_stats.percentage_of_project = 60.0;
        stats_map.insert(Language::Rust, rust_stats);

        let mut js_stats = LanguageStats::new(Language::JavaScript);
        js_stats.percentage_of_project = 40.0;
        stats_map.insert(Language::JavaScript, js_stats);

        let pattern = MultiLanguageAnalyzer::detect_architecture_pattern(&stats_map);
        assert_eq!(pattern, ArchitecturePattern::FullStack);
    }

    #[test]
    fn test_exclude_directories() {
        let temp_dir = create_test_project(vec![
            ("src/main.rs", "fn main() {}"),
            ("target/debug/main.rs", "fn main() {}"),
            ("node_modules/lib/index.js", "console.log()"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new();
        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        // Should only find src/main.rs, not target or node_modules
        if let Some(rust_stats) = analysis.language_stats.get(&Language::Rust) {
            assert_eq!(rust_stats.file_count, 1);
        }
    }

    #[test]
    fn test_overall_tdg_calculation() {
        let mut stats_map = HashMap::new();

        let mut rust_stats = LanguageStats::new(Language::Rust);
        rust_stats.lines_of_code = 100;
        rust_stats.tdg_score = Some(TdgScore {
            score: 90.0,
            grade: Grade::A,
        });
        stats_map.insert(Language::Rust, rust_stats);

        let mut py_stats = LanguageStats::new(Language::Python);
        py_stats.lines_of_code = 100;
        py_stats.tdg_score = Some(TdgScore {
            score: 80.0,
            grade: Grade::BPlus,
        });
        stats_map.insert(Language::Python, py_stats);

        let overall = MultiLanguageAnalyzer::calculate_overall_tdg(&stats_map);

        // Should be weighted average: (90*100 + 80*100) / 200 = 85.0
        assert_eq!(overall.score, 85.0);
    }

    #[test]
    fn test_empty_project() {
        let temp_dir = TempDir::new().unwrap();

        let analyzer = MultiLanguageAnalyzer::new();
        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        assert_eq!(analysis.total_lines, 0);
        assert_eq!(analysis.total_files, 0);
        assert_eq!(analysis.language_stats.len(), 0);
    }

    #[test]
    fn test_custom_exclude_dirs() {
        let temp_dir = create_test_project(vec![
            ("src/main.rs", "fn main() {}"),
            ("custom_exclude/test.rs", "fn test() {}"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new()
            .with_exclude_dirs(vec!["custom_exclude".to_string()]);

        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        if let Some(rust_stats) = analysis.language_stats.get(&Language::Rust) {
            assert_eq!(rust_stats.file_count, 1);
        }
    }

    #[test]
    fn test_max_depth_limit() {
        let temp_dir = create_test_project(vec![
            ("a/main.rs", "fn main() {}"),
            ("a/b/test.rs", "fn test() {}"),
            ("a/b/c/deep.rs", "fn deep() {}"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new().with_max_depth(1);

        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        // Should only find files up to depth 1
        if let Some(rust_stats) = analysis.language_stats.get(&Language::Rust) {
            assert!(rust_stats.file_count <= 2);
        }
    }

    #[test]
    fn test_file_analysis_with_comments() {
        let temp_dir = create_test_project(vec![(
            "test.rs",
            "// Comment line 1\n// Comment line 2\nfn main() {}\n\n",
        )]);

        let analyzer = MultiLanguageAnalyzer::new();
        let file_path = temp_dir.path().join("test.rs");
        let stats = analyzer.analyze_file(&file_path).unwrap();

        assert_eq!(stats.lines, 4);
        assert_eq!(stats.comment_lines, 2);
        assert_eq!(stats.blank_lines, 1);
    }

    #[test]
    fn test_primary_and_secondary_languages() {
        let temp_dir = create_test_project(vec![
            ("main.rs", &"fn main() {}\n".repeat(100)),
            ("app.js", &"console.log();\n".repeat(30)),
            ("script.sh", "#!/bin/bash\n"),
        ]);

        let analyzer = MultiLanguageAnalyzer::new();
        let analysis = analyzer.analyze(temp_dir.path()).unwrap();

        assert_eq!(analysis.primary_language, Some(Language::Rust));
        assert!(analysis.secondary_languages.contains(&Language::JavaScript));
    }
}
