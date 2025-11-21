//! Project analysis and TDG scoring

use crate::types::{Error, Grade, Language, Result, TdgScore};
use std::collections::HashMap;
use std::path::Path;

/// Project analyzer for language detection and quality scoring
pub struct Analyzer {
    /// Path to project directory
    path: String,
}

impl Analyzer {
    /// Create a new analyzer for the given path
    ///
    /// # Examples
    ///
    /// ```
    /// use batuta_cookbook::Analyzer;
    ///
    /// let analyzer = Analyzer::new("./examples/data/sample_project");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().to_string(),
        }
    }

    /// Analyze the project and return a report
    ///
    /// # Errors
    ///
    /// Returns `Error::InvalidPath` if path doesn't exist
    /// Returns `Error::NoFilesFound` if directory is empty
    pub fn analyze(&self) -> Result<AnalysisReport> {
        // Stub implementation for now
        // TODO: Implement actual file scanning and analysis
        let path = Path::new(&self.path);

        if !path.exists() {
            return Err(Error::InvalidPath(self.path.clone()));
        }

        // For now, return a stub report
        Ok(AnalysisReport {
            path: self.path.clone(),
            primary_language: Language::Python,
            languages: HashMap::from([(Language::Python, 1000)]),
            file_count: 10,
            total_lines: 1000,
            tdg_score: None,
        })
    }

    /// Analyze with TDG scoring
    ///
    /// # Errors
    ///
    /// Same as `analyze()`
    pub fn analyze_with_tdg(&self) -> Result<AnalysisReport> {
        let mut report = self.analyze()?;

        // Calculate TDG score
        // TODO: Implement actual TDG calculation based on metrics
        let score = 85.0; // Stub value
        report.tdg_score = Some(TdgScore {
            score,
            grade: Grade::from_score(score),
        });

        Ok(report)
    }
}

/// Analysis report containing project metrics
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    /// Project path
    pub path: String,
    /// Primary (most common) language
    pub primary_language: Language,
    /// Language breakdown: Language -> line count
    pub languages: HashMap<Language, usize>,
    /// Total file count
    pub file_count: usize,
    /// Total lines of code
    pub total_lines: usize,
    /// Technical Debt Grade (if calculated)
    pub tdg_score: Option<TdgScore>,
}

impl AnalysisReport {
    /// Get TDG score or calculate a default
    #[must_use] 
    pub fn tdg(&self) -> TdgScore {
        self.tdg_score.unwrap_or(TdgScore {
            score: 75.0,
            grade: Grade::B,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = Analyzer::new("./test_path");
        assert_eq!(analyzer.path, "./test_path");
    }

    #[test]
    fn test_invalid_path() {
        let analyzer = Analyzer::new("/nonexistent/path/12345");
        let result = analyzer.analyze();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::InvalidPath(_)));
    }

    #[test]
    fn test_analyze_current_dir() {
        let analyzer = Analyzer::new(".");
        let result = analyzer.analyze();
        assert!(result.is_ok());

        let report = result.unwrap();
        assert!(report.file_count > 0);
        assert!(report.total_lines > 0);
    }

    #[test]
    fn test_tdg_score_bounds() {
        let analyzer = Analyzer::new(".");
        if let Ok(report) = analyzer.analyze_with_tdg() {
            if let Some(tdg) = report.tdg_score {
                assert!(tdg.score >= 0.0);
                assert!(tdg.score <= 100.0);
            }
        }
    }
}
