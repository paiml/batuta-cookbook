//! Common types used across the cookbook

use std::fmt;

/// Cookbook-specific error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid path or file not found
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// No files found in directory
    #[error("No files found in directory: {0}")]
    NoFilesFound(String),

    /// Unsupported language
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),

    /// Transpilation error
    #[error("Transpilation failed: {0}")]
    TranspilationError(String),

    /// Validation error
    #[error("Validation failed: {0}")]
    ValidationError(String),

    /// Analysis error
    #[error("Analysis failed: {0}")]
    Analysis(String),

    /// Generic error
    #[error("Error: {0}")]
    Other(String),
}

/// Result type using cookbook Error
pub type Result<T> = std::result::Result<T, Error>;

/// Programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    /// Python
    Python,
    /// C
    C,
    /// C++
    Cpp,
    /// Rust
    Rust,
    /// Shell/Bash
    Shell,
    /// JavaScript
    JavaScript,
    /// Unknown language
    Unknown,
}

impl Language {
    /// Get file extensions for this language
    #[must_use] 
    pub fn extensions(self) -> &'static [&'static str] {
        match self {
            Self::Python => &["py", "pyw"],
            Self::C => &["c", "h"],
            Self::Cpp => &["cpp", "cc", "cxx", "hpp", "hxx"],
            Self::Rust => &["rs"],
            Self::Shell => &["sh", "bash"],
            Self::JavaScript => &["js", "jsx", "ts", "tsx"],
            Self::Unknown => &[],
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Python => write!(f, "Python"),
            Self::C => write!(f, "C"),
            Self::Cpp => write!(f, "C++"),
            Self::Rust => write!(f, "Rust"),
            Self::Shell => write!(f, "Shell"),
            Self::JavaScript => write!(f, "JavaScript"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Technical Debt Grade
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TdgScore {
    /// Score from 0-100
    pub score: f64,
    /// Letter grade (A+, A, A-, B+, etc.)
    pub grade: Grade,
}

/// Letter grades for TDG scoring
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Grade {
    /// 95-100
    APlus,
    /// 90-94
    A,
    /// 85-89
    AMinus,
    /// 80-84
    BPlus,
    /// 75-79
    B,
    /// 70-74
    BMinus,
    /// 60-69
    C,
    /// 50-59
    D,
    /// <50
    F,
}

impl Grade {
    /// Convert score to grade
    #[must_use] 
    pub fn from_score(score: f64) -> Self {
        match score {
            s if s >= 95.0 => Self::APlus,
            s if s >= 90.0 => Self::A,
            s if s >= 85.0 => Self::AMinus,
            s if s >= 80.0 => Self::BPlus,
            s if s >= 75.0 => Self::B,
            s if s >= 70.0 => Self::BMinus,
            s if s >= 60.0 => Self::C,
            s if s >= 50.0 => Self::D,
            _ => Self::F,
        }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::APlus => write!(f, "A+"),
            Self::A => write!(f, "A"),
            Self::AMinus => write!(f, "A-"),
            Self::BPlus => write!(f, "B+"),
            Self::B => write!(f, "B"),
            Self::BMinus => write!(f, "B-"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::F => write!(f, "F"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_extensions() {
        assert!(Language::Python.extensions().contains(&"py"));
        assert!(Language::Rust.extensions().contains(&"rs"));
    }

    #[test]
    fn test_grade_from_score() {
        assert_eq!(Grade::from_score(96.0), Grade::APlus);
        assert_eq!(Grade::from_score(92.0), Grade::A);
        assert_eq!(Grade::from_score(87.0), Grade::AMinus);
        assert_eq!(Grade::from_score(45.0), Grade::F);
    }

    #[test]
    fn test_grade_display() {
        assert_eq!(Grade::APlus.to_string(), "A+");
        assert_eq!(Grade::A.to_string(), "A");
        assert_eq!(Grade::BMinus.to_string(), "B-");
    }
}
