//! # Recipe 100-3: Detect Dependency Managers
//!
//! **Level:** 100 (Foundational)
//! **Time Estimate:** 8 hours
//! **Priority:** P1 (High)
//!
//! ## Overview
//!
//! This recipe demonstrates how to detect dependency management systems across multiple
//! programming languages. It scans project directories and identifies package managers
//! based on manifest files, enabling automated tooling decisions.
//!
//! ## Supported Package Managers
//!
//! - **JavaScript/TypeScript:** npm (package.json), yarn (yarn.lock), pnpm (pnpm-lock.yaml)
//! - **Python:** pip (requirements.txt), poetry (pyproject.toml), pipenv (Pipfile)
//! - **Rust:** cargo (Cargo.toml)
//! - **Java:** maven (pom.xml), gradle (build.gradle, build.gradle.kts)
//! - **Go:** go modules (go.mod)
//! - **Ruby:** bundler (Gemfile)
//! - **PHP:** composer (composer.json)
//! - **.NET:** nuget (*.csproj, packages.config)
//!
//! ## Examples
//!
//! Run individual examples with:
//! ```bash
//! cargo run --example recipe_100_3_dependency_detection
//! ```
//!
//! ## Tests
//!
//! Run tests with:
//! ```bash
//! cargo test --example recipe_100_3_dependency_detection
//! ```

use batuta_cookbook::{Error, Result};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a detected dependency manager
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DependencyManager {
    /// Name of the package manager (e.g., "npm", "cargo", "pip")
    pub name: String,
    /// Manifest file that was detected (e.g., "package.json", "Cargo.toml")
    pub manifest_file: String,
    /// Path to the manifest file relative to project root
    pub path: PathBuf,
    /// Ecosystem this manager belongs to
    pub ecosystem: String,
}

impl DependencyManager {
    /// Create a new DependencyManager instance
    pub fn new(name: String, manifest_file: String, path: PathBuf, ecosystem: String) -> Self {
        Self {
            name,
            manifest_file,
            path,
            ecosystem,
        }
    }
}

/// Configuration for dependency detection
#[derive(Debug, Clone)]
pub struct DetectionConfig {
    /// Maximum directory depth to scan (default: 3)
    pub max_depth: usize,
    /// Whether to follow symbolic links (default: false)
    pub follow_symlinks: bool,
    /// Directories to exclude from scanning (e.g., "node_modules", "target")
    pub exclude_dirs: HashSet<String>,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        let mut exclude_dirs = HashSet::new();
        exclude_dirs.insert("node_modules".to_string());
        exclude_dirs.insert("target".to_string());
        exclude_dirs.insert("dist".to_string());
        exclude_dirs.insert("build".to_string());
        exclude_dirs.insert(".git".to_string());
        exclude_dirs.insert("vendor".to_string());
        exclude_dirs.insert("venv".to_string());
        exclude_dirs.insert(".venv".to_string());

        Self {
            max_depth: 3,
            follow_symlinks: false,
            exclude_dirs,
        }
    }
}

/// Detect dependency managers in a project directory
pub fn detect_dependency_managers(
    project_path: &Path,
    config: &DetectionConfig,
) -> Result<Vec<DependencyManager>> {
    let mut managers = Vec::new();
    let project_path = project_path.canonicalize().map_err(|e| {
        Error::Analysis(format!(
            "Failed to canonicalize path {}: {}",
            project_path.display(),
            e
        ))
    })?;

    scan_directory(&project_path, &project_path, 0, config, &mut managers)?;

    // Sort for consistent output
    managers.sort();
    Ok(managers)
}

/// Recursively scan directory for dependency manifests
fn scan_directory(
    current_path: &Path,
    root_path: &Path,
    depth: usize,
    config: &DetectionConfig,
    managers: &mut Vec<DependencyManager>,
) -> Result<()> {
    if depth > config.max_depth {
        return Ok(());
    }

    let entries = fs::read_dir(current_path).map_err(|e| {
        Error::Analysis(format!(
            "Failed to read directory {}: {}",
            current_path.display(),
            e
        ))
    })?;

    for entry in entries {
        let entry = entry.map_err(|e| Error::Analysis(format!("Failed to read entry: {}", e)))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip excluded directories
        if path.is_dir() && config.exclude_dirs.contains(file_name_str.as_ref()) {
            continue;
        }

        // Check if this is a manifest file
        if let Some(manager) = identify_manifest(&path, root_path) {
            managers.push(manager);
        }

        // Recurse into subdirectories
        if path.is_dir() && (config.follow_symlinks || !is_symlink(&path)) {
            scan_directory(&path, root_path, depth + 1, config, managers)?;
        }
    }

    Ok(())
}

/// Identify if a file is a dependency manifest and return the corresponding manager
fn identify_manifest(file_path: &Path, root_path: &Path) -> Option<DependencyManager> {
    let file_name = file_path.file_name()?.to_string_lossy();
    let relative_path = file_path.strip_prefix(root_path).ok()?.to_path_buf();

    match file_name.as_ref() {
        // JavaScript/TypeScript ecosystem
        "package.json" => Some(DependencyManager::new(
            "npm".to_string(),
            file_name.to_string(),
            relative_path,
            "JavaScript".to_string(),
        )),
        "yarn.lock" => Some(DependencyManager::new(
            "yarn".to_string(),
            file_name.to_string(),
            relative_path,
            "JavaScript".to_string(),
        )),
        "pnpm-lock.yaml" => Some(DependencyManager::new(
            "pnpm".to_string(),
            file_name.to_string(),
            relative_path,
            "JavaScript".to_string(),
        )),

        // Python ecosystem
        "requirements.txt" => Some(DependencyManager::new(
            "pip".to_string(),
            file_name.to_string(),
            relative_path,
            "Python".to_string(),
        )),
        "Pipfile" => Some(DependencyManager::new(
            "pipenv".to_string(),
            file_name.to_string(),
            relative_path,
            "Python".to_string(),
        )),
        "pyproject.toml" => Some(DependencyManager::new(
            "poetry".to_string(),
            file_name.to_string(),
            relative_path,
            "Python".to_string(),
        )),

        // Rust ecosystem
        "Cargo.toml" => Some(DependencyManager::new(
            "cargo".to_string(),
            file_name.to_string(),
            relative_path,
            "Rust".to_string(),
        )),

        // Java ecosystem
        "pom.xml" => Some(DependencyManager::new(
            "maven".to_string(),
            file_name.to_string(),
            relative_path,
            "Java".to_string(),
        )),
        "build.gradle" | "build.gradle.kts" => Some(DependencyManager::new(
            "gradle".to_string(),
            file_name.to_string(),
            relative_path,
            "Java".to_string(),
        )),

        // Go ecosystem
        "go.mod" => Some(DependencyManager::new(
            "go".to_string(),
            file_name.to_string(),
            relative_path,
            "Go".to_string(),
        )),

        // Ruby ecosystem
        "Gemfile" => Some(DependencyManager::new(
            "bundler".to_string(),
            file_name.to_string(),
            relative_path,
            "Ruby".to_string(),
        )),

        // PHP ecosystem
        "composer.json" => Some(DependencyManager::new(
            "composer".to_string(),
            file_name.to_string(),
            relative_path,
            "PHP".to_string(),
        )),

        _ => {
            // Check for .NET project files
            if file_name.ends_with(".csproj") || file_name == "packages.config" {
                return Some(DependencyManager::new(
                    "nuget".to_string(),
                    file_name.to_string(),
                    relative_path,
                    ".NET".to_string(),
                ));
            }
            None
        }
    }
}

/// Check if a path is a symbolic link
fn is_symlink(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

// ============================================================================
// EXAMPLE 1: Basic Dependency Detection
// ============================================================================

fn example_1_basic_detection() -> Result<()> {
    println!("=== Example 1: Basic Dependency Detection ===\n");

    // Detect dependency managers in the current project
    let project_path = Path::new(".");
    let config = DetectionConfig::default();

    let managers = detect_dependency_managers(project_path, &config)?;

    println!("Found {} dependency manager(s):\n", managers.len());
    for manager in &managers {
        println!("  • {} ({})", manager.name, manager.ecosystem);
        println!("    Manifest: {}", manager.manifest_file);
        println!("    Location: {}", manager.path.display());
        println!();
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 2: Custom Detection Configuration
// ============================================================================

fn example_2_custom_config() -> Result<()> {
    println!("=== Example 2: Custom Detection Configuration ===\n");

    let project_path = Path::new(".");
    let mut config = DetectionConfig::default();

    // Scan deeper and include more directories
    config.max_depth = 5;
    config.exclude_dirs.remove("target"); // Include target directory for demo

    let managers = detect_dependency_managers(project_path, &config)?;

    println!("Found {} dependency manager(s) with custom config:\n", managers.len());
    println!("Config: max_depth={}, follow_symlinks={}", config.max_depth, config.follow_symlinks);
    println!();

    for manager in &managers {
        println!("  • {}", manager.name);
    }

    Ok(())
}

// ============================================================================
// EXAMPLE 3: Multi-Ecosystem Detection
// ============================================================================

fn example_3_multi_ecosystem() -> Result<()> {
    println!("=== Example 3: Multi-Ecosystem Detection ===\n");

    let project_path = Path::new(".");
    let config = DetectionConfig::default();

    let managers = detect_dependency_managers(project_path, &config)?;

    // Group by ecosystem
    let mut ecosystems: HashSet<String> = HashSet::new();
    for manager in &managers {
        ecosystems.insert(manager.ecosystem.clone());
    }

    println!("Detected {} ecosystem(s):", ecosystems.len());
    for ecosystem in &ecosystems {
        println!("\n  📦 {}", ecosystem);
        for manager in &managers {
            if manager.ecosystem == *ecosystem {
                println!("    - {} ({})", manager.name, manager.manifest_file);
            }
        }
    }

    Ok(())
}

// ============================================================================
// MAIN FUNCTION - Run all examples
// ============================================================================

fn main() -> Result<()> {
    example_1_basic_detection()?;
    println!("\n{}\n", "=".repeat(70));

    example_2_custom_config()?;
    println!("\n{}\n", "=".repeat(70));

    example_3_multi_ecosystem()?;

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

    /// Helper to create a test project structure
    fn create_test_project(files: &[&str]) -> TempDir {
        let temp_dir = TempDir::new().unwrap();
        for file_path in files {
            let full_path = temp_dir.path().join(file_path);
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&full_path, "").unwrap();
        }
        temp_dir
    }

    #[test]
    fn test_detect_single_manager() {
        let temp_dir = create_test_project(&["package.json"]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 1);
        assert_eq!(managers[0].name, "npm");
        assert_eq!(managers[0].manifest_file, "package.json");
        assert_eq!(managers[0].ecosystem, "JavaScript");
    }

    #[test]
    fn test_detect_multiple_managers() {
        let temp_dir = create_test_project(&["package.json", "Cargo.toml", "requirements.txt"]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 3);

        let names: Vec<&str> = managers.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"npm"));
        assert!(names.contains(&"cargo"));
        assert!(names.contains(&"pip"));
    }

    #[test]
    fn test_detect_nested_manifests() {
        let temp_dir = create_test_project(&[
            "package.json",
            "frontend/package.json",
            "backend/Cargo.toml",
        ]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 3);
    }

    #[test]
    fn test_exclude_directories() {
        let temp_dir = create_test_project(&[
            "package.json",
            "node_modules/foo/package.json",
            "target/package.json",
        ]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        // Should only find the root package.json, not the ones in excluded dirs
        assert_eq!(managers.len(), 1);
        assert_eq!(managers[0].path, PathBuf::from("package.json"));
    }

    #[test]
    fn test_max_depth_limit() {
        let temp_dir = create_test_project(&[
            "package.json",
            "a/package.json",
            "a/b/package.json",
            "a/b/c/package.json",
            "a/b/c/d/package.json", // Beyond depth 3
        ]);
        let mut config = DetectionConfig::default();
        config.max_depth = 2;

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        // Should find root, a/, and a/b/ but not deeper
        assert!(managers.len() <= 3);
    }

    #[test]
    fn test_all_supported_ecosystems() {
        let temp_dir = create_test_project(&[
            "package.json",      // JavaScript
            "Cargo.toml",        // Rust
            "requirements.txt",  // Python
            "pom.xml",          // Java (Maven)
            "build.gradle",     // Java (Gradle)
            "go.mod",           // Go
            "Gemfile",          // Ruby
            "composer.json",    // PHP
            "project.csproj",   // .NET
        ]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 9);

        let ecosystems: HashSet<String> = managers.iter()
            .map(|m| m.ecosystem.clone())
            .collect();

        assert!(ecosystems.contains("JavaScript"));
        assert!(ecosystems.contains("Rust"));
        assert!(ecosystems.contains("Python"));
        assert!(ecosystems.contains("Java"));
        assert!(ecosystems.contains("Go"));
        assert!(ecosystems.contains("Ruby"));
        assert!(ecosystems.contains("PHP"));
        assert!(ecosystems.contains(".NET"));
    }

    #[test]
    fn test_yarn_and_pnpm_detection() {
        let temp_dir = create_test_project(&["package.json", "yarn.lock", "pnpm-lock.yaml"]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 3);

        let names: Vec<&str> = managers.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"npm"));
        assert!(names.contains(&"yarn"));
        assert!(names.contains(&"pnpm"));
    }

    #[test]
    fn test_python_package_managers() {
        let temp_dir = create_test_project(&["requirements.txt", "Pipfile", "pyproject.toml"]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 3);

        let names: Vec<&str> = managers.iter().map(|m| m.name.as_str()).collect();
        assert!(names.contains(&"pip"));
        assert!(names.contains(&"pipenv"));
        assert!(names.contains(&"poetry"));
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        assert_eq!(managers.len(), 0);
    }

    #[test]
    fn test_dependency_manager_ordering() {
        let temp_dir = create_test_project(&["Cargo.toml", "package.json", "requirements.txt"]);
        let config = DetectionConfig::default();

        let managers = detect_dependency_managers(temp_dir.path(), &config).unwrap();

        // Results should be sorted consistently
        assert_eq!(managers.len(), 3);

        // Check that running detection twice gives the same order
        let managers2 = detect_dependency_managers(temp_dir.path(), &config).unwrap();
        assert_eq!(managers, managers2);
    }
}
